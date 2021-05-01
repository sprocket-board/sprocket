#![no_main]
#![no_std]

use cassette::{yield_now, Cassette};
use core::future::Future;
use core::pin::Pin;
use rtic::app;
use rtic_world as _; // global logger + panicking-behavior + memory layout
use sprocket_bsp::Sprocket;
use static_alloc::Bump;
use sprocket_bsp::hal::exti::Event;
use sprocket_bsp::hal::gpio::SignalEdge;
use sprocket_bsp::hal::exti::ExtiExt;
use sprocket_bsp::hal::{
    gpio::gpioa::PA0,
    gpio::gpioc::PC14,
    gpio::Output,
    gpio::PushPull,
    gpio::Input,
    gpio::Floating,
};
use sprocket_bsp::hal::prelude::*;

static A: Bump<[u8; 1024]> = Bump::uninit();
type StaticCassette<O> = Cassette<Pin<&'static mut (dyn Future<Output = O> + Send)>>;

fn caser<T, F, O>(data: T, func: fn(&'static mut T) -> F) -> Result<StaticCassette<O>, ()>
where
    F: Future<Output = O> + Send + 'static,
{
    // Leak the data...
    let leaked_data = A.leak(data).map_err(drop)?;

    // Leak the future...
    let leaked_futr = A.leak((func)(leaked_data)).map_err(drop)?;

    // Coerce into a dyn Trait...
    let coerce_futr: &mut (dyn Future<Output = O> + Send) = leaked_futr;

    // Pin it...
    //
    // SAFETY: All items are statically allocated and cannot be moved
    let pin_dyn_fut = unsafe { Pin::new_unchecked(coerce_futr) };

    // Success!
    Ok(Cassette::new(pin_dyn_fut))
}

#[app(device = sprocket_bsp::hal::stm32)]
const APP: () = {
    struct Resources {
        stepper_cas: StaticCassette<()>,
        exti: sprocket_bsp::hal::stm32::EXTI,
    }

    #[init]
    fn init(ctx: init::Context) -> init::LateResources {
        defmt::info!("Hello, world!");
        let board = defmt::unwrap!(Sprocket::new());

        let button_stepper = defmt::unwrap!(caser(
            StepdownButton {
                count: 5,
                led1: board.led1,
            },
            StepdownButton::entry,
        ));

        let mut exti = board.exti;

        board.button1.listen(
            SignalEdge::Falling,
            &mut exti
        );

        init::LateResources {
            stepper_cas: button_stepper,
            exti,
        }
    }

    #[task(binds = EXTI4_15, resources = [stepper_cas, exti])]
    fn button(ctx: button::Context) {
        let exti = ctx.resources.exti;
        let cas = ctx.resources.stepper_cas;

        if exti.is_pending(
            Event::GPIO14,
            SignalEdge::Falling,
        ) {
            exti.unpend(Event::GPIO14);
            match cas.poll_on() {
                Some(_) => {
                    defmt::info!("Oops.");
                    rtic_world::exit()
                },
                None => (),
            }
        }
    }

    #[idle]
    fn idle(_ctx: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }
};

struct StepdownButton {
    count: usize,
    led1: PA0<Output<PushPull>>,
}

impl StepdownButton {
    async fn entry(&mut self) {
        // TODO: Should be in a loop {}
        for _ in 0..self.count {
            self.led1.set_low().ok();
            yield_now().await;
            self.led1.set_high().ok();
            yield_now().await;
        }
    }
}
