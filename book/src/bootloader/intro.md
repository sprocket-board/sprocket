# Bootloader

The bootloader for the sprocket is [`sprocket-boot`]. It is written in Rust.

[`sprocket-boot`]: https://github.com/sprocket-board/sprocket-boot

## Installing

Build [`sprocket-boot`] with a nightly compiler, and flash with `probe-run` or however you flash binaries using an SWD adapter.

## Memory Layout

NOTE: Both the application and the bootloader must respect these regions.

| Region        | Device    | Start Addr    | Size       | Usage                             |
| :---          | :---      | :---          | :---       | :---                              |
| Application   | FLASH     | 0x0800_0000   | 58KiB      | User Application and Vector Table |
| Settings      | FLASH     | 0x0800_E800   | 2KiB       | Bootloader/Application Settings   |
| Bootloader    | FLASH     | 0x0800_F000   | 4KiB       | Bootloader Code                   |
| RAM Flags     | RAM       | 0x2000_0000   | 128 Bytes  | Message passing between BL/APP    |
| User Memory   | RAM       | 0x2000_0080   | 8064 Bytes | General Purpose RAM               |

## How it works

Sprocket boot doesn't actually relocate the vector table, or own the vector table itself. It depends on the reset vector and MSP of the application to contain the reset address and stack address of the bootloader.

This is achieved by hot-patching the application image when bootloading with the proper bootloader information. It then stores the Application's reset vector and MSP in the Settings page.

This means that if you use SWD (or some other means, like DFU), you will likely break the bootloader, unless you manually apply these changes.

## The boot sequence

This is a distilled version of how the boot sequence works:

* The system boots directly to the bootloader
* The bootloader checks:
    * The Settings Page
    * The RAM Flags Segment
    * The Reset Vector/MSP of the Application
    * The state of the buttons
* If the buttons are pressed, the system stays in bootloader
* If the "stay in bootloader" RAM flag has been set, the system stays in bootloader
* If the Settings or Application data looks bad, the system stays in bootloader
* Otherwise the bootloader jumps to the application
* If the system is stayig in bootloader, it starts listening as an I2C Peripheral

## Pages and Subpages

The FLASH is broken up into two main chunks:

* PAGES, which are always 2KiB, and are the minimum erasable size on the STM32G031.
* SUBPAGES, which are currently defined as 256 bytes. This number was chosen arbitrarily
* Typically, a PAGE is erased, and then each SUBPAGE is written with new data.

## Bootloader I2C Commands

<!--
TODO:

Move this to the I2C section
-->

The Bootloader acts as an I2C peripheral, and can be clocked (theoretically) up to 1mbps
with appropriate pullups and cable length, though clock rates higher than 400kHz are not
currently reliable. A **clock rate of 100kHz-400kHz is recommended**. Clock stretching is
probably required at the moment, but I am open to removing that requirement.

There are two kinds of I2C communications from a Controller perspective:

* Writes
* Writes-then-Reads

* For Writes:
    * Send the I2C address - Write
    * then a 1-byte register ID
    * then write the contents of the register
    * then a STOP
* For Writes-then-Reads:
    * Send the I2C Address - Write
    * Then a 1-byte register ID
    * Then a STOP
    * Send the I2C Address - Read
    * Then read the contents of the register
    * Then a stop

| Register  | Direction | Length | Usage                    |
| :---      | :---      | :---   | :---                     |
| 0x10      | WR-TH-RD  | 16     | Bootloader Image Name    |
| 0x40      | WRITE     | 5      | Start Bootload           |
| 0x41      | WRITE     | 261    | Write Subpage            |
| 0x42      | WRITE     | 0      | Complete and Reboot      |
| 0x45      | WRITE     | 1      | Set I2C Address          |

At the moment, any other read/write will cause the bootloader to panic.
Also Note: The Length in this table does not count the 1 byte register address.

### `0x10` - Bootloader Image Name

At the moment, the device responds with a constant `b"sprocket boot!!!"`.
This register can be used to verify bootloader communications.

### `0x40` - Start Bootload

Before writing subpages, a bootload must be started. This message should contain:

* 4 bytes - total checksum (later crc32), little endian
* 1 byte - total subpages to write

### `0x41` - Write Subpage

Before writing subpages, a bootload must be started. On the first write of each page,
the page will be erased.

For now, Page 0 + Subpage 0 must be the LAST subpage written. When writing this subpage,
the reset vector and stack pointer will be overwritten and stored to the Settings page.

Write Subpage messages contain:

* 1 byte: Page/Subpage - 0bPPPPP_SSS
   * max 32 pages
   * 8 sub-pages per page
* 256: subpage contents
* 4 byte: For now: 32-bit checksum. Later, CRC32 or similar

### `0x42` - Complete and Reboot

This command will reboot to the application if:

* No subpages have been written, or a bootload was never started
* ALL subpages have been written, after a bootload was started

The device will wait for the I2C STOP command, then reboot.

### `0x45` - Set I2C address

After starting a bootload, a new I2C address can be set. This address will always be used
by the bootloader, and may be used by the application as well.

This I2C address will only be used after a reboot, and is written to the settings page
when a "Complete and Reboot" command is sent.
