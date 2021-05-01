#![no_main]
#![no_std]

use rtic_world as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    rtic_world::exit()
}
