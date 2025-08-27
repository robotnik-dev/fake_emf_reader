#![no_main]
#![no_std]

use core::i16;

use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use microbit::{hal::gpio, Board};
use nrf52833_hal::{saadc::SaadcConfig, Saadc};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut pot_in = board.edge.e00.into_floating_input();
    let mut gpio1 = board.edge.e08.into_push_pull_output(gpio::Level::Low);
    let mut gpio2 = board.pins.p0_17.into_push_pull_output(gpio::Level::Low);
    let mut gpio3 = board.pins.p0_01.into_push_pull_output(gpio::Level::Low);
    let mut gpio4 = board.pins.p0_13.into_push_pull_output(gpio::Level::Low);
    let mut gpio5 = board.edge.e16.into_push_pull_output(gpio::Level::Low);

    let mut adc = Saadc::new(board.ADC, SaadcConfig::default());

    loop {
        if let Ok(value) = adc.read_channel(&mut pot_in) {
            let percent = i16::MAX / value;
            // TODO: rtt printout
            match percent {
                5..20 => {
                    // the first to 1, the rest to 0
                    gpio1.set_high().unwrap();
                    gpio2.set_low().unwrap();
                    gpio3.set_low().unwrap();
                    gpio4.set_low().unwrap();
                    gpio5.set_low().unwrap();
                }
                20..40 => {
                    // the first 2 to 1, the rest to 0
                    gpio1.set_high().unwrap();
                    gpio2.set_high().unwrap();
                    gpio3.set_low().unwrap();
                    gpio4.set_low().unwrap();
                    gpio5.set_low().unwrap();
                }
                40..60 => {
                    // the first three to 1, the rest to 0
                    gpio1.set_high().unwrap();
                    gpio2.set_high().unwrap();
                    gpio3.set_high().unwrap();
                    gpio4.set_low().unwrap();
                    gpio5.set_low().unwrap();
                }
                60..80 => {
                    // the first 4 to 1, the rest to 0
                    gpio1.set_high().unwrap();
                    gpio2.set_high().unwrap();
                    gpio3.set_high().unwrap();
                    gpio4.set_high().unwrap();
                    gpio5.set_low().unwrap();
                }
                80..=100 => {
                    // all 5 to 1
                    gpio1.set_high().unwrap();
                    gpio2.set_high().unwrap();
                    gpio3.set_high().unwrap();
                    gpio4.set_high().unwrap();
                    gpio5.set_high().unwrap();
                }
                _ => {
                    // all to 0
                    gpio1.set_low().unwrap();
                    gpio2.set_low().unwrap();
                    gpio3.set_low().unwrap();
                    gpio4.set_low().unwrap();
                    gpio5.set_low().unwrap();
                }
            }
        }
    }
}
