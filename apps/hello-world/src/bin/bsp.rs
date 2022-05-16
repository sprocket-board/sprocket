#![no_main]
#![no_std]

use hello_world as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    hello_world::exit()
}
