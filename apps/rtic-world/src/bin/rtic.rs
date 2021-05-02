#![no_main]
#![no_std]

use rtic_world as _; // global logger + panicking-behavior + memory layout

use core::{
    future::Future,
    pin::Pin,
    task::Poll,
};
use rtic::app;

use cassette::{yield_now, Cassette, futures::poll_fn};
use heapless::{
    i,
    spsc::{Queue, Consumer, Producer},
    consts,
};
use sprocket_bsp::{
    hal::exti::{Event, ExtiExt},
    hal::gpio::{gpioa::PA0, gpiob::PB8, Output, PushPull, SignalEdge},
    hal::prelude::*,
    Sprocket,
};
use static_alloc::Bump;

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
        main_cas: StaticCassette<()>,
        exti: sprocket_bsp::hal::stm32::EXTI,
    }

    #[init]
    fn init(_ctx: init::Context) -> init::LateResources {
        static mut BTN_TO_MAIN: Queue<u32, consts::U16> = Queue(i::Queue::new());

        defmt::info!("Hello, world!");
        let board = defmt::unwrap!(Sprocket::new());

        let (pro, con) = BTN_TO_MAIN.split();

        let button_stepper = defmt::unwrap!(caser(
            StepdownButton {
                count: 8,
                led1: board.led1,
                ttl_ct: 0,
                tx: pro,
            },
            StepdownButton::entry,
        ));

        let main_stepper = defmt::unwrap!(caser(
            MainHandler { rx: AsyncConsumer::new(con), led2: board.led2 },
            MainHandler::entry,
        ));

        let mut exti = board.exti;

        board.button1.listen(SignalEdge::Falling, &mut exti);

        init::LateResources {
            stepper_cas: button_stepper,
            exti,
            main_cas: main_stepper,
        }
    }

    #[task(binds = EXTI4_15, resources = [stepper_cas, exti])]
    fn button(ctx: button::Context) {
        let exti = ctx.resources.exti;
        let cas = ctx.resources.stepper_cas;

        if exti.is_pending(Event::GPIO14, SignalEdge::Falling) {
            exti.unpend(Event::GPIO14);
            match cas.poll_on() {
                Some(_) => {
                    defmt::info!("Done!");
                    rtic_world::exit()
                }
                None => (),
            }
        }
    }

    #[idle(resources = [main_cas])]
    fn idle(ctx: idle::Context) -> ! {
        loop {
            let _ = ctx.resources.main_cas.poll_on();
        }
    }
};

struct StepdownButton {
    count: usize,
    led1: PA0<Output<PushPull>>,
    tx: Producer<'static, u32, consts::U16>,
    ttl_ct: u32,
}

struct MainHandler {
    rx: AsyncConsumer,
    led2: PB8<Output<PushPull>>,
}

impl MainHandler {
    async fn entry(&mut self) {
        loop {
            let msg = self.rx.async_receive().await;
            defmt::info!("count_a: {=u32}", msg);
            self.led2.set_low().ok();

            let msg = self.rx.async_receive().await;
            defmt::info!("count_b: {=u32}", msg);
            self.led2.set_high().ok();
        }
    }
}

impl StepdownButton {
    async fn entry(&mut self) {
        loop {
            for _ in 0..(self.count + 1 / 2) {
                self.led1.set_low().ok();
                yield_now().await;
                self.led1.set_high().ok();
                yield_now().await;
                self.ttl_ct = self.ttl_ct.wrapping_add(1);

                // ding
                defmt::unwrap!(self.tx.enqueue(self.ttl_ct));
            }
        }
    }
}

struct AsyncConsumer {
    cons: Consumer<'static, u32, consts::U16>,
}

impl AsyncConsumer {
    fn new(cons: Consumer<'static, u32, consts::U16>) -> Self {
        Self {
            cons,
        }
    }

    fn async_receive(&mut self) -> impl Future<Output=u32> + '_ {
        poll_fn(move |_| {
            if let Some(msg) = self.cons.dequeue() {
                Poll::Ready(msg)
            } else {
                Poll::Pending
            }
        })
    }
}
