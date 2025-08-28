#![no_main]
#![no_std]

use core::i16;
use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use microbit::{hal::gpio, Board};
use nrf52833_hal::{saadc::SaadcConfig, Saadc};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Starting up..");
    let board = Board::take().unwrap();
    // P0
    let mut pot_in = board.edge.e00.into_floating_input();
    // P8
    let mut gpio1 = board.edge.e08.into_push_pull_output(gpio::Level::Low);
    // P13
    let mut gpio2 = board.pins.p0_17.into_push_pull_output(gpio::Level::Low);
    // P14
    let mut gpio3 = board.pins.p0_01.into_push_pull_output(gpio::Level::Low);
    // P15
    let mut gpio4 = board.pins.p0_13.into_push_pull_output(gpio::Level::Low);
    // P16
    let mut gpio5 = board.edge.e16.into_push_pull_output(gpio::Level::Low);

    rprintln!("Configure ADC");
    let mut adc = Saadc::new(board.ADC, SaadcConfig::default());

    loop {
        if let Ok(value) = adc.read_channel(&mut pot_in) {
            let percent = value as f32 / (i16::MAX as f32 / 2.0);
            rprintln!("{}", percent);
            match percent {
                0.05..0.2 => {
                    // 5 - 20 %
                    // the first to 1, the rest to 0
                    gpio1.set_high().unwrap();
                    gpio2.set_low().unwrap();
                    gpio3.set_low().unwrap();
                    gpio4.set_low().unwrap();
                    gpio5.set_low().unwrap();
                }
                0.2..0.4 => {
                    // 20 - 40 %
                    // the first 2 to 1, the rest to 0
                    gpio1.set_high().unwrap();
                    gpio2.set_high().unwrap();
                    gpio3.set_low().unwrap();
                    gpio4.set_low().unwrap();
                    gpio5.set_low().unwrap();
                }
                0.4..0.6 => {
                    // 40 - 60 %
                    // the first three to 1, the rest to 0
                    gpio1.set_high().unwrap();
                    gpio2.set_high().unwrap();
                    gpio3.set_high().unwrap();
                    gpio4.set_low().unwrap();
                    gpio5.set_low().unwrap();
                }
                0.6..0.8 => {
                    // 60 -80 %
                    // the first 4 to 1, the rest to 0
                    gpio1.set_high().unwrap();
                    gpio2.set_high().unwrap();
                    gpio3.set_high().unwrap();
                    gpio4.set_high().unwrap();
                    gpio5.set_low().unwrap();
                }
                0.8..=1.0 => {
                    // 80 - 100 %
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
