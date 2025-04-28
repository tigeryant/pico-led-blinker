#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use rp_pico::hal::{pac, clocks::init_clocks_and_plls, watchdog::Watchdog, sio::Sio, gpio::Pins};
// use rp_pico::hal::{pac, clocks::init_clocks_and_plls, watchdog::Watchdog, sio::Sio, gpio::Function, gpio::Pins};
// use rp_pico::hal;
use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(peripherals.WATCHDOG);

    // Configure clocks
    let _clocks = init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        peripherals.XOSC,
        peripherals.CLOCKS,
        peripherals.PLL_SYS,
        peripherals.PLL_USB,
        &mut peripherals.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = Sio::new(peripherals.SIO);

    let pins = Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS,
    );

    // The LED is connected to pin 29
    let mut led_pin = pins.gpio29.into_push_pull_output();

    loop {
        led_pin.set_high().unwrap();
        cortex_m::asm::delay(500_000);
        led_pin.set_low().unwrap();
        cortex_m::asm::delay(500_000);
    }
}
