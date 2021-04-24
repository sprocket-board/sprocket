#![no_std]

use stm32g0xx_hal as hal;
use hal::{
    stm32::{self, SPI2, I2C1},
    prelude::*,
    spi::{Spi, NoSck, NoMiso},
    gpio::{
        gpioa, gpiob, gpioc,
        Analog,
    },
    i2c_periph::I2CPeripheral,
    rcc::{
        Config,
        PllConfig,
        Prescaler,
    },
};
use ws2812_spi::{Ws2812, MODE};

#[allow(unused_imports)]
use smart_leds::{RGB8, SmartLedsWrite, colors, gamma};

use groundhog_stm32g031::GlobalRollingTimer;

pub struct Sprocket {
    // Onboard accessories
    pub button1: gpioc::PC14<Analog>,
    pub button2: gpioc::PC15<Analog>,
    pub led1: gpioa::PA0<Analog>,
    pub led2: gpiob::PB8<Analog>,
    pub smartled: Ws2812<Spi<SPI2, (NoSck, NoMiso, gpioa::PA4<Analog>)>>,

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
    pub spi_copi: gpiob::PB5<Analog>,

    pub uart_tx: gpioa::PA2<Analog>,
    pub uart_rx: gpioa::PA3<Analog>,

    pub i2c2_scl: gpioa::PA11<Analog>, // note: shadows pa09
    pub i2c2_sda: gpioa::PA12<Analog>, // note: shadows pa12

    // Left Side QWIIC/Annular Rings
    pub i2c1: I2CPeripheral<I2C1>,

    // Debugging ports
    pub swdio: gpioa::PA13<Analog>,
    pub swclk: gpioa::PA14<Analog>, // also boot0

    pub core: stm32::CorePeripherals,

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

        let smartled_spi = Spi::spi2(
            board.SPI2,
            (NoSck, NoMiso, gpioa.pa4),
            MODE,
            3_800_000.hz(),
            &mut rcc,
        );
        let smartled = Ws2812::new(smartled_spi);

        let i2c1 = I2CPeripheral::new(
            board.I2C1,
            gpiob.pb7,
            gpiob.pb6,
            &mut rcc,
            0x69,
        );

        let spkt = Sprocket {
            button1: gpioc.pc14,
            button2: gpioc.pc15,
            led1: gpioa.pa0,
            led2: gpiob.pb8,
            smartled,

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
            spi_copi: gpiob.pb5,

            uart_tx: gpioa.pa2,
            uart_rx: gpioa.pa3,

            i2c2_scl: gpioa.pa11, // note: shadows pa09
            i2c2_sda: gpioa.pa12, // note: shadows pa12

            i2c1,

            swdio: gpioa.pa13,
            swclk: gpioa.pa14, // also boot0

            core,
        };

        // Initialize global timer
        GlobalRollingTimer::init(board.TIM2);

        Some(spkt)
    }
}

