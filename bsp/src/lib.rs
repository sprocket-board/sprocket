#![no_std]

pub use stm32g0xx_hal as hal;
use hal::{
    stm32::{self, SPI2, SPI1, I2C1},
    prelude::*,
    spi::{Spi, NoSck, NoMiso},
    gpio::{
        gpioa, gpiob, gpioc,
        Analog,
    },
    analog::adc::Adc,
    i2c_periph::I2CPeripheral,
    rcc::{
        Config,
        PllConfig,
        Prescaler,
    },
};
use ws2812_spi::{Ws2812, MODE};
use ws2812_spi::prerendered::Ws2812 as PRWs2812;

#[allow(unused_imports)]
use smart_leds::{RGB8, SmartLedsWrite, colors, gamma};

/// You need to provide a buffer `data`, whoose length is at least 12 * the
/// length of the led strip + 20 byes (or 40, if using the `mosi_idle_high` feature)
pub const NUM_LEDS: usize = 120;
pub const BUF_LEN: usize = (12 * NUM_LEDS) + 40;

pub use groundhog_stm32g031;
use groundhog_stm32g031::GlobalRollingTimer;

pub struct Sprocket {
    // Onboard accessories
    pub button1: gpioc::PC14<Analog>,
    pub button2: gpioc::PC15<Analog>,
    pub led1: gpioa::PA0<Analog>,
    pub led2: gpiob::PB8<Analog>,
    pub smartled: Ws2812<Spi<SPI2, (NoSck, NoMiso, gpioa::PA4<Analog>)>>,
    pub smartled2: PRWs2812<'static, Spi<SPI1, (NoSck, NoMiso, gpiob::PB5<Analog>)>>,

    // GPIO port
    pub gpio1: gpioa::PA1<Analog>,
    pub gpio2: gpioa::PA5<Analog>,
    pub gpio3: gpioa::PA6<Analog>,
    pub gpio4: gpioa::PA7<Analog>,
    pub gpio5: gpiob::PB0<Analog>,
    pub gpio6: gpiob::PB1<Analog>,
    pub gpio7: gpioa::PA8<Analog>,
    pub gpio8: gpioc::PC6<Analog>,

    // Data ports
    pub spi_csn: gpioa::PA15<Analog>,
    pub spi_sck: gpiob::PB3<Analog>,
    pub spi_cipo: gpiob::PB4<Analog>,
    // pub spi_copi: gpiob::PB5<Analog>,

    pub uart_tx: gpioa::PA2<Analog>,
    pub uart_rx: gpioa::PA3<Analog>,

    pub i2c2_scl: gpioa::PA11<Analog>, // note: shadows pa09
    pub i2c2_sda: gpioa::PA12<Analog>, // note: shadows pa12

    // Left Side QWIIC/Annular Rings
    // pub i2c1: I2CPeripheral<I2C1>,

    // Debugging ports
    pub swdio: gpioa::PA13<Analog>,
    pub swclk: gpioa::PA14<Analog>, // also boot0

    pub core: stm32::CorePeripherals,

    pub adc: Adc,

    // TODO: All the other stuff from `board`

}

impl Sprocket {
    pub fn new() -> Option<Self> {
        let board = stm32::Peripherals::take()?;
        let core = stm32::CorePeripherals::take()?;

        // Configure clocks
        let config = Config::pll()
            .pll_cfg(PllConfig::with_hsi(1, 8, 2))
            .ahb_psc(Prescaler::NotDivided)
            .apb_psc(Prescaler::NotDivided);
        let mut rcc = board.RCC.freeze(config);

        let gpioa = board.GPIOA.split(&mut rcc);
        let gpiob = board.GPIOB.split(&mut rcc);
        let gpioc = board.GPIOC.split(&mut rcc);

        let buf = cortex_m::singleton!(: [u8; 1024 + 512] = [0u8; 1024 + 512])?;

        let smartled_spi = Spi::spi2(
            board.SPI2,
            (NoSck, NoMiso, gpioa.pa4),
            MODE,
            3_000_000.hz(),
            &mut rcc,
        );
        let smartled = Ws2812::new(smartled_spi);

        let smartled_spi = Spi::spi1(
            board.SPI1,
            (NoSck, NoMiso, gpiob.pb5),
            MODE,
            3_000_000.hz(),
            &mut rcc,
        );
        // smartled_spi.half_duplex_enable(true);
        let smartled2 = PRWs2812::new(smartled_spi, buf);
        // let sw = board.TIM16.stopwatch(&mut rcc);
        // TODO: read the settings page (or ram page?) for
        // I2C address, and possibly which peripherals to
        // load
        // let i2c1 = I2CPeripheral::new(
        //     board.I2C1,
        //     gpiob.pb7,
        //     gpiob.pb6,
        //     &mut rcc,
        //     0x69,
        //     sw,
        // );

        let spkt = Sprocket {
            button1: gpioc.pc14,
            button2: gpioc.pc15,
            led1: gpioa.pa0,
            led2: gpiob.pb8,
            smartled,
            smartled2,

            gpio1: gpioa.pa1,
            gpio2: gpioa.pa5,
            gpio3: gpioa.pa6,
            gpio4: gpioa.pa7,
            gpio5: gpiob.pb0,
            gpio6: gpiob.pb1,
            gpio7: gpioa.pa8,
            gpio8: gpioc.pc6,

            spi_csn: gpioa.pa15,
            spi_sck: gpiob.pb3,
            spi_cipo: gpiob.pb4,
            // spi_copi: gpiob.pb5,

            uart_tx: gpioa.pa2,
            uart_rx: gpioa.pa3,

            i2c2_scl: gpioa.pa11, // note: shadows pa09
            i2c2_sda: gpioa.pa12, // note: shadows pa12

            // i2c1,

            swdio: gpioa.pa13,
            swclk: gpioa.pa14, // also boot0

            adc: Adc::new(board.ADC, &mut rcc),

            core,
        };

        // Initialize global timer
        GlobalRollingTimer::init(board.TIM2);

        Some(spkt)
    }

    pub fn timer() -> GlobalRollingTimer {
        GlobalRollingTimer::new()
    }
}


pub struct JamesWs2812<SPI> {
    spi: SPI,
}

impl<SPI, E> JamesWs2812<SPI>
where
    SPI: embedded_hal::spi::FullDuplex<u8, Error = E>,
{
    /// Use ws2812 devices via spi
    ///
    /// The SPI bus should run within 2 MHz to 3.8 MHz
    ///
    /// You may need to look at the datasheet and your own hal to verify this.
    ///
    /// Please ensure that the mcu is pretty fast, otherwise weird timing
    /// issues will occur
    pub fn new(spi: SPI) -> Self {
        Self {
            spi,
        }
    }
}

impl<SPI, E> JamesWs2812<SPI>
where
    SPI: embedded_hal::spi::FullDuplex<u8, Error = E>,
{
    /// Write a single byte for ws2812 devices
    fn write_byte(&mut self, mut data: u8) -> Result<(), E> {
        // Send two bits in one spi byte. High time first, then the low time
        // The maximum for T0H is 500ns, the minimum for one bit 1063 ns.
        // These result in the upper and lower spi frequency limits
        let patterns = [
            0b1000_1000, // 00
            0b1000_1100, // 01
            0b1100_1000, // 10
            0b1100_1100, // 11
        ];
        for _ in 0..4 {
            let bits = (data & 0b1100_0000) >> 6;
            nb::block!(self.spi.send(patterns[bits as usize]))?;
            // nb::block!(self.spi.read()).ok();
            data <<= 2;
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), E> {
        // Should be > 300μs, so for an SPI Freq. of 3.8MHz, we have to send at least 1140 low bits or 140 low bytes
        for _ in 0..140 {
            nb::block!(self.spi.send(0))?;
            // nb::block!(self.spi.read()).ok();
        }
        Ok(())
    }
}

impl<SPI, E> SmartLedsWrite for JamesWs2812<SPI>
where
    SPI: embedded_hal::spi::FullDuplex<u8, Error = E>,
{
    type Error = E;
    type Color = RGB8;
    /// Write all the items of an iterator to a ws2812 strip
    fn write<T, I>(&mut self, iterator: T) -> Result<(), E>
    where
        T: Iterator<Item = I>,
        I: Into<Self::Color>,
    {
        // We introduce an offset in the fifo here, so there's always one byte in transit
        // Some MCUs (like the stm32f1) only a one byte fifo, which would result
        // in overrun error if two bytes need to be stored
        nb::block!(self.spi.send(0))?;
        if cfg!(feature = "mosi_idle_high") {
            self.flush()?;
        }

        for item in iterator {
            let item = item.into();
            self.write_byte(item.g)?;
            self.write_byte(item.r)?;
            self.write_byte(item.b)?;
        }
        self.flush()?;
        // Now, resolve the offset we introduced at the beginning
        nb::block!(self.spi.read())?;
        Ok(())
    }
}
