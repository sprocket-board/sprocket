# Hardware

This is where I put information about the hardware

## Image overview

> TODO: Do an ifixit/adafruit style highlighted picture

## Pins by Package Number

| **UQFPN-28 Number** | **Pin Name**   | **SPROCKET USE**          | **PWM** | **ADC** |
| :---              | :---             | :---                      | :---    | :---    |
| 1                 | PC14-OSC32_IN    | BUTTON1                   | No      | No      |
| 2                 | PC15-OSC32_OUT   | BUTTON2                   | No      | No      |
| 3                 | VDD/VDDA         | Supply                    | No      | No      |
| 4                 | VSS/VSSA         | Supply                    | No      | No      |
| 5                 | PF2-NRST         | SWD                       | No      | No      |
| 6                 | PA0              | LED1                      | Yes     | ADC0    |
| 7                 | PA1              | GPIO1                     | Yes     | ADC1    |
| 8                 | PA2              | UART-TX                   | Yes     | ADC2    |
| 9                 | PA3              | UART-RX                   | Yes     | ADC3    |
| 10                | PA4              | SmartLED (SPI2_MOSI)      | Yes     | ADC4    |
| 11                | PA5              | GPIO2                     | Yes     | ADC5    |
| 12                | PA6              | GPIO3                     | Yes     | ADC6    |
| 13                | PA7              | GPIO4                     | Yes     | ADC7    |
| 14                | PB0              | GPIO5                     | Yes     | ADC8    |
| 15                | PB1              | GPIO6                     | Yes     | ADC9    |
| 16                | PA8              | GPIO7                     | Yes     | No      |
| 17                | PC6              | GPIO                      | Yes     | No      |
| 18                | PA11/PA9         | I2C2-SCL                  | Yes     | ADC15   |
| 19                | PA12/PA10        | I2C2-SDA                  | Maybe?  | ADC16   |
| 20                | PA13             | SWDIO                     | No      | ADC17   |
| 21                | PA14-BOOT0       | SWCLK                     | Maybe?  | ADC18   |
| 22                | PA15             | SPI-CSn                   | Maybe?  | No      |
| 23                | PB3              | SPI-SCK                   | Yes     | No      |
| 24                | PB4              | SPI-CIPO                  | Yes     | No      |
| 25                | PB5              | SPI-COPI                  | Yes     | No      |
| 26                | PB6              | I2C1-SCL                  | Yes     | No      |
| 27                | PB7              | I2C1-SDA                  | Maybe?  | ADC11   |
| 28                | PB8              | LED2                      | Yes     | No      |


<details>
    <summary>Complete Pinout Table</summary>

| PIN Number | Pin Name         | PIN TYPE  | IO Capabilities   | NOTES | SPROCKET USE              | ALT FUNCS                                                                     | ADD'L FUNC                                    |
| :---       | :---             | :---      | :---              | :---  | :---                      | :---                                                                          | :---                                          |
| 1          | PC14-OSC32_IN    | I/O       | FT                | 1, 2  | BUTTON1                   | TIM1_BK2                                                                      | OSC32_IN, OSC_IN                              |
| 2          | PC15-OSC32_OUT   | I/O       | FT                | 1, 2  | BUTTON2                   | OSC32_EN, OSC_EN                                                              | OSC32_OUT                                     |
| 3          | VDD/VDDA         | S         |                   |       | Supply                    |                                                                               |                                               |
| 4          | VSS/VSSA         | S         |                   |       | Supply                    |                                                                               |                                               |
| 5          | PF2-NRST         | I/O       | -                 | -     | SWD                       | MCO                                                                           | NRST                                          |
| 6          | PA0              | I/O       | FT_a              | -     | LED1                      | SPI2_SCK, USART2_CTS, TIM2_CH1_ETR, LPTIM1_OUT                                | ADC_IN0, TAMP_IN2, WKUP1                      |
| 7          | PA1              | I/O       | FT_ea             | -     | GPIO1                     | SPI1_SCK, I2S1_CK, USART2_RTS_DE_CK, TIM2_CH2, I2C1_SMBA, EVENTOUT            | ADC_IN1                                       |
| 8          | PA2              | I/O       | FT_a              | -     | UART-TX                   | SPI1_MOSI, I2S1_SD, USART2_TX, TIM2_CH3, LPUART1_TX                           | ADC_IN2, WKUP4, LSCO                          |
| 9          | PA3              | I/O       | FT_ea             | -     | UART-RX                   | SPI2_MISO, USART2_RX, TIM2_CH4, LPUART1_RX, EVENTOUT                          | ADC_IN3                                       |
| 10         | PA4              | I/O       | FT_a              | -     | SmartLED (SPI2_MOSI)      | SPI1_NSS, I2S1_WS, SPI2_MOSI, TIM14_CH1, LPTIM2_OUT, EVENTOUT                 | ADC_IN4, TAMP_IN1, RTC_TS, RTC_OUT1, WKUP2    |
| 11         | PA5              | I/O       | FT_ea             | -     | GPIO2                     | SPI1_SCK, I2S1_CK, TIM2_CH1_ETR, LPTIM2_ETR, EVENTOUT                         | ADC_IN5                                       |
| 12         | PA6              | I/O       | FT_ea             | -     | GPIO3                     | SPI1_MISO, I2S1_MCK, TIM3_CH1, TIM1_BK, TIM16_CH1, LPUART1_CTS                | ADC_IN6                                       |
| 13         | PA7              | I/O       | FT_a              | -     | GPIO4                     | SPI1_MOSI, I2S1_SD, TIM3_CH2, TIM1_CH1N, TIM14_CH1, TIM17_CH1                 | ADC_IN7                                       |
| 14         | PB0              | I/O       | FT_ea             | -     | GPIO5                     | SPI1_NSS, I2S1_WS, TIM3_CH3, TIM1_CH2N, LPTIM1_OUT                            | ADC_IN8                                       |
| 15         | PB1              | I/O       | FT_ea             | -     | GPIO6                     | TIM14_CH1, TIM3_CH4, TIM1_CH3N, LPTIM2_IN1, LPUART1_RTS_DE, EVENTOUT          | ADC_IN9                                       |
| 16         | PA8              | I/O       | FT                | -     | GPIO7                     | MCO, SPI2_NSS, TIM1_CH1, LPTIM2_OUT, EVENTOUT                                 | -                                             |
| 17         | PC6              | I/O       | FT                | -     | GPIO                      | TIM3_CH1, TIM2_CH3                                                            | -                                             |
| 18         | PA11/PA9         | I/O       | FT_fa             | 3     | I2C2-SCL                  | SPI1_MISO, I2S1_MCK, USART1_CTS, TIM1_CH4, TIM1_BK2, I2C2_SCL                 | ADC_IN15                                      |
| 19         | PA12/PA10        | I/O       | FT_fa             | 3     | I2C2-SDA                  | SPI1_MOSI, I2S1_SD, USART1_RTS_DE_CK, TIM1_ETR, I2S_CKIN, I2C2_SDA            | ADC_IN16                                      |
| 20         | PA13             | I/O       | FT_ea             | 4     | SWDIO                     | SWDIO, IR_OUT, EVENTOUT                                                       | ADC_IN17                                      |
| 21         | PA14-BOOT0       | I/O       | FT_a              | 4     | SWCLK                     | SWCLK, USART2_TX, EVENTOUT                                                    | ADC_IN18, BOOT0                               |
| 22         | PA15             | I/O       | FT                | -     | SPI-CSn                   | SPI1_NSS, I2S1_WS, USART2_RX, TIM2_CH1_ETR, EVENTOUT                          | -                                             |
| 23         | PB3              | I/O       | FT                | -     | SPI-SCK                   | SPI1_SCK, I2S1_CK, TIM1_CH2, TIM2_CH2, USART1_RTS_DE_CK, EVENTOUT             | -                                             |
| 24         | PB4              | I/O       | FT                | -     | SPI-CIPO                  | SPI1_MISO, I2S1_MCK, TIM3_CH1, USART1_CTS, TIM17_BK, EVENTOUT                 | -                                             |
| 25         | PB5              | I/O       | FT                | -     | SPI-COPI                  | SPI1_MOSI, I2S1_SD, TIM3_CH2, TIM16_BK, LPTIM1_IN1, I2C1_SMBA                 | WKP6                                          |
| 26         | PB6              | I/O       | FT_f              | -     | I2C1-SCL                  | USART1_TX, TIM1_CH3, TIM16_CH1N, SPI2_MISO, LPTIM1_ETR, I2C1_SCL, EVENTOUT    | -                                             |
| 27         | PB7              | I/O       | FT_fa             | -     | I2C1-SDA                  | USART1_RX, SPI2_MOSI, TIM17_CH1N, LPTIM1_IN2, I2C1_SDA, EVENTOUT              | ADC_IN11, PVD_IN                              |
| 28         | PB8              | I/O       | FT_f              | -     | LED2                      | SPI2_SCK, TIM16_CH1, I2C1_SCL, EVENTOUT                                       | -                                             |

| NOTES | Meaning                                                       |
| :---  | :---                                                          |
| 1     | <= 2MHz, max load 30pF, only sinks 3mA (collectively?)        |
| 2     | RTC domain relevant, see RM0444                               |
| 3     | pins are remappable to swap between IOs using SYSCFG_CFGR1    |
| 4     | SWD on reset, PA13 Pull-Up, PA14 Pull-down internally         |
| FT    | 5V tolerant I/O                                               |
| _f    | Fm+ capable                                                   |
| _a    | analog switch function                                        |
| _e    | switchable diode to Vdd                                       |
| PVD   | Programmable Voltage Detector                                 |
| MCO   | Microcontroller Clock Output                                  |
| LSCO  | Low Speed Clock Output                                        |

</details>

## Board Pinout

|       | | Col 1 | Col 2 | | Col 3 | | Col 4 | Col 5 |
| :---: | | :---: | :---: | | :---: | | :---: | :---: |
| Row 1 | | GPIO1 | GPIO5 | | SWDIO | | COPI  | CSn   |
| Row 2 | | GPIO2 | GPIO6 | | GND   | | SCK   | CIPO  |
| Row 3 | | GPIO3 | GPIO7 | | SWCLK | | SDA   | SCL   |
| Row 4 | | GPIO4 | GPIO8 | | 3v3   | | 3v3   | 5v0   |
| Row 5 | | GND   | GND   | | NRST  | | GND   | GND   |
| Row 6 | | 3v3   | 5v0   | | 5v0+  | | RX    | TX    |

"5v0" pins are post-protection diode. "5v0+" are pre-protection diode.
