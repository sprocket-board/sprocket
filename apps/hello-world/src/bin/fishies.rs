#![no_main]
#![no_std]

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
            gpiob::{PB0, PB1},
            Output, PushPull,
            gpioa::{PA6, PA7, PA5, PA1},
            Input,
            PullUp,
            Analog
        },
        analog::adc::Adc
    },
};

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    let mut bsp = Sprocket::new().unwrap();
    let mut colors = [smart_leds::colors::BLACK; 100];
    let timer = Sprocket::timer();

    let dial_top = bsp.gpio1;
    let dial_bot = bsp.gpio2;

    let butt_l = bsp.gpio3.into_pull_up_input();
    let butt_r = bsp.gpio4.into_pull_up_input();

    let led_l = bsp.gpio6.into_push_pull_output_in_state(PinState::High);
    let led_r = bsp.gpio5.into_push_pull_output_in_state(PinState::High);
    bsp.smartled2.write(colors.iter().cloned()).ok();

    let t_cal_s = timer.get_ticks();
    while timer.millis_since(t_cal_s) <= 50 { }
    bsp.adc.calibrate();

    let mut wl = WhiteLight {
        mode: Mode::Off,
        butt_1: butt_l,
        butt_2: butt_r,
        led_1: led_l,
        led_2: led_r,
        dial_1: dial_top,
        dial_2: dial_bot,
        adc: bsp.adc,
        run_d1: 0,
        run_d2: 0,
        cur_hue: 0,
        last_col: smart_leds::colors::BLACK,
        current: smart_leds::colors::BLACK,
        leds: &mut colors,
        smartled: bsp.smartled2,
        dirty: false,
    };

    let start = timer.get_ticks();
    let mut last_button = start;
    let mut last_adc = start;
    let mut last_update = start;

    loop {
        // if timer.millis_since(last_button) >= 20 {
        //     last_button = timer.get_ticks();
        //     wl.poll_buttons();
        // }

        if timer.millis_since(last_adc) >= 10 {
            last_adc = timer.get_ticks();
            wl.poll_adcs();
        }

        if timer.millis_since(last_update) >= 50 {
            last_update = timer.get_ticks();
            wl.update_leds();
        }

    }
}

struct WhiteLight<'a, T>
where
    T: SmartLedsWrite,
{
    mode: Mode,
    butt_1: PA6<Input<PullUp>>,
    butt_2: PA7<Input<PullUp>>,
    led_1: PB1<Output<PushPull>>,
    led_2: PB0<Output<PushPull>>,
    dial_1: PA1<Analog>,
    dial_2: PA5<Analog>,
    adc: Adc,
    run_d1: u16,
    run_d2: u16,
    current: RGB8,
    last_col: RGB8,
    cur_hue: u8,
    leds: &'a mut [RGB8; 100],
    smartled: T,
    dirty: bool,
}

#[derive(Copy, Clone, PartialEq)]
enum Mode {
    Off,
    HsvColor,
    MaxBright,
}

impl<'a, T> WhiteLight<'a, T>
where
    T: SmartLedsWrite<Color = RGB8>,
{
    // Poll buttons, update mode, update button LEDs
    fn poll_buttons(&mut self) {
        let btn1 = self.butt_1.is_low().unwrap_or(false);
        let btn2 = self.butt_2.is_low().unwrap_or(false);

        let old_mode = self.mode;
        self.mode = match (self.mode, btn1, btn2) {
            // Ignore dual button press
            (_, true, true) => self.mode,
            // No button presses, no updates
            (_, false, false) => self.mode,

            (Mode::Off, true, false) => Mode::HsvColor,
            (Mode::Off, false, true) => Mode::MaxBright,
            (Mode::HsvColor, true, false) => Mode::Off,
            (Mode::HsvColor, false, true) => Mode::MaxBright,
            (Mode::MaxBright, true, false) => Mode::HsvColor,
            (Mode::MaxBright, false, true) => Mode::Off,
        };

        if self.mode != old_mode {
            match self.mode {
                Mode::Off => {
                    self.led_1.set_high().ok();
                    self.led_2.set_high().ok();
                }
                Mode::HsvColor => {
                    self.led_1.set_low().ok();
                    self.led_2.set_high().ok();
                }
                Mode::MaxBright => {
                    self.led_1.set_high().ok();
                    self.led_2.set_low().ok();
                }
            }
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

    // Poll ADCs, update IIR average, update LED color
    fn poll_adcs(&mut self) {
        defmt::println!("{:?}", self.adc.read(&mut self.dial_1).unwrap_or(0));
        // let (d1, d2) = match self.mode {
        //     Mode::Off => (0, 0),
        //     Mode::HsvColor | Mode::MaxBright => {
        //         let d1_read: u16 = self.adc.read(&mut self.dial_1).unwrap_or(0);
        //         let d2_read: u16 = self.adc.read(&mut self.dial_2).unwrap_or(0);
        //         let d1 = (d1_read >> 4).min(255);
        //         let d2 = (d2_read >> 4).min(255);
        //         (d1, d2)
        //     },
        // };

        // self.run_d1 = self.run_d1 - (self.run_d1 >> 5) + d1; // (old - (old / 32)) + (new / 32)
        // self.run_d2 = self.run_d2 - (self.run_d2 >> 5) + d2; // (old - (old / 32)) + (new / 32)

        // let new_col = match self.mode {
        //     Mode::Off => {
        //         RGB8 {
        //             r: self.current.r.saturating_add(1),
        //             g: self.current.g.saturating_add(1),
        //             b: self.current.b.saturating_add(1),
        //         }
        //     },
        //     Mode::HsvColor => {
        //         let new_hue = (self.run_d1 >> 5).min(255) as u8;
        //         let new_val = (self.run_d2 >> 5).min(255) as u8;

        //         hsv2rgb(Hsv { hue: new_hue, sat: 255, val: new_val })
        //     },
        //     Mode::MaxBright => {
        //         let new_rgb = (self.run_d2 >> 5).min(255) as u8;
        //         RGB8 {
        //             r: new_rgb,
        //             g: new_rgb,
        //             b: new_rgb,
        //         }
        //     },
        // };

        // let delta_r = ((self.last_col.r as i16) - (new_col.r as i16)).abs();
        // let delta_g = ((self.last_col.g as i16) - (new_col.g as i16)).abs();
        // let delta_b = ((self.last_col.b as i16) - (new_col.b as i16)).abs();

        // let delta = delta_r + delta_g + delta_b;

        // if delta >= 10 {
        //     defmt::println!("DIRTY {=i16}", delta);
        //     self.dirty = true;
        //     self.leds.iter_mut().for_each(|c| *c = new_col);
        // }
        // self.current = new_col;
    }

    fn update_leds(&mut self) {
        self.cur_hue = self.cur_hue.wrapping_add(1);
        self.current = hsv2rgb(Hsv { hue: self.cur_hue, sat: 255, val: 255 });
        defmt::println!("{=u8},{=u8},{=u8},", self.current.r, self.current.g, self.current.b);
        let x = self.current.r;
        self.current.r = self.current.g;
        self.current.g = x;
        let col = &[self.current];
        let ci = col.iter().cloned().cycle().take(100);
        self.smartled.write(gamma(ci)).ok();
    }
}
