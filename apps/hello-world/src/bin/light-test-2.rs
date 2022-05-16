#![no_main]
#![no_std]

use choreographer::{engine::{Sequence, LoopBehavior}, script};
use cortex_m::prelude::_embedded_hal_adc_OneShot;
use groundhog::RollingTimer;
use hello_world as _; use smart_leds::{SmartLedsWrite, RGB8, gamma, hsv::{hsv2rgb, Hsv}};
// global logger + panicking-behavior + memory layout
use sprocket_bsp::{
    Sprocket,
    hal::{
        self,
        prelude::{
            InputPin, PinState, ToggleableOutputPin, OutputPin,
        },
        gpio::{
            gpiob::{PB0, PB1, PB8},
            Output, PushPull,
            gpioa::{PA6, PA7, PA5, PA1, PA0},
            Input,
            PullUp,
            Analog, gpioc::{PC15, PC14}
        },
        analog::adc::Adc
    }, groundhog_stm32g031::GlobalRollingTimer,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    let mut bsp = Sprocket::new().unwrap();
    let timer = Sprocket::timer();

    let t_cal_s = timer.get_ticks();
    while timer.millis_since(t_cal_s) <= 50 { }
    bsp.adc.calibrate();

    defmt::println!("{=u32}", core::mem::size_of::<Sequence<GlobalRollingTimer, 3>>() as u32);
    while timer.millis_since(t_cal_s) <= 3_000 { }

    let script: Sequence<GlobalRollingTimer, 3> = Sequence::empty();

    let mut wl = WhiteLight {
        mode: Mode::Off,
        butt_1: bsp.button1.into_pull_up_input(),
        butt_2: bsp.button2.into_pull_up_input(),
        led_1: bsp.led1.into_push_pull_output_in_state(PinState::High),
        led_2: bsp.led2.into_push_pull_output_in_state(PinState::High),
        last_col: smart_leds::colors::BLACK,
        current: smart_leds::colors::BLACK,
        smartled: bsp.smartled2,
        dirty: false,
        cur_hue: 0,
        cur_val: 0,
        choreo: script,
        max_ticks: 0,
    };

    let start = timer.get_ticks();
    let mut last_button = start;
    let mut last_update = start;
    let mut last_chor = start;

    loop {
        if timer.millis_since(last_button) >= 20 {
            last_button = timer.get_ticks();
            wl.poll_buttons();
        }

        if timer.millis_since(last_chor) >= 10 {
            last_chor = timer.get_ticks();
            wl.poll_choreo();
        }

        if timer.millis_since(last_update) >= 2 {
            last_update = timer.get_ticks();
            wl.update_leds();
        }

    }
}

struct WhiteLight<T>
where
    T: SmartLedsWrite,
{
    mode: Mode,
    butt_1: PC14<Input<PullUp>>,
    butt_2: PC15<Input<PullUp>>,
    led_1: PA0<Output<PushPull>>,
    led_2: PB8<Output<PushPull>>,
    current: RGB8,
    last_col: RGB8,
    cur_hue: u8,
    cur_val: u8,
    smartled: T,
    dirty: bool,
    choreo: Sequence<GlobalRollingTimer, 3>,
    max_ticks: u32,
}

#[derive(Copy, Clone, PartialEq)]
enum Mode {
    Off,
    HsvColor,
    MaxBright,
}

impl<T> WhiteLight<T>
where
    T: SmartLedsWrite<Color = RGB8>,
{
    // Poll buttons, update mode, update button LEDs
    fn poll_buttons(&mut self) {
        let btn1 = self.butt_1.is_low().unwrap_or(false);
        let btn2 = self.butt_2.is_low().unwrap_or(false);

        if btn2 {
            self.cur_val = self.cur_val.wrapping_add(16);
        }

        // Wait for both buttons to be released
        loop {
            let btn1 = self.butt_1.is_low().unwrap_or(false);
            let btn2 = self.butt_2.is_low().unwrap_or(false);

            if !btn1 && !btn2 {
                break;
            }
        }
    }

    fn poll_choreo(&mut self) {
        let timer = GlobalRollingTimer::default();

        let start = timer.get_ticks();
        let x = self.choreo.poll();
        let elapsed = timer.ticks_since(start);
        self.max_ticks = self.max_ticks.max(elapsed);
        defmt::println!("ticks: {=u32}/{=u32}", elapsed, self.max_ticks);

        match x {
            Some(col) => {
                self.current = col;
                self.dirty = true;
            },
            None => {
                self.choreo.set(&script! {
                    | action |  color | duration_ms | period_ms_f | phase_offset_ms | repeat |
                    |  solid |  BLACK |        1000 |         0.0 |               0 |   once |
                    |    sin |  WHITE |        2500 |      2500.0 |               0 |   once |
                    |  solid |  BLACK |        1000 |         0.0 |               0 |   once |
                }, LoopBehavior::OneShot);
            },
        }
    }

    fn update_leds(&mut self) {
        if self.dirty {
            let mut color = self.current;

            // defmt::println!("{=u8},{=u8},{=u8},", self.current.r, self.current.g, self.current.b);
            let x = color.r;
            color.r = color.g;
            color.g = x;

            let x = &[color];
            let col = (x).iter().cloned().cycle().take(100);
            self.smartled.write(gamma(col)).ok();
            self.last_col = self.current;
        }
        self.dirty = false;
    }
}
