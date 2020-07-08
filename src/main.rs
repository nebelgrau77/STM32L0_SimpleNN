#![no_main]
#![no_std]

extern crate panic_halt;

mod nn_lib;

mod model;

pub use crate::nn_lib::nn::*;

pub use crate::model::model::*;

use cortex_m_rt::entry;
use stm32l0xx_hal::{
    pac,
    prelude::*,
    rcc::{Config,MSIRange},
    serial,
};

use core::fmt::Write;

#[entry]
fn main() -> ! {
    
    // neural network setup

    //let input_vector: [f64; 3] = [30.0,87.0,110.0];  //[temperature, humidity, air_quality]
    //let mut predicted_results:[f64; 3] = [0_f64; 3]; //initial array for predicted results

    //let expected_values: [f64; 3] = [600.0, 10.0, -90.0]; //i.e. y values

    let mut weight: f64 = 0.5;
    let input: f64 = 0.5;
    let expected_value: f64 = 0.8;
    let step_amount: f64 = 0.001;
    let epochs: u32 = 1500;

    // peripherals setup

    let dp = pac::Peripherals::take().unwrap();
    let _cp = cortex_m::Peripherals::take().unwrap();

    //configure the clock
    let mut rcc = dp.RCC.freeze(Config::msi(MSIRange::Range2)); //Range4 means around 1.048 MHz
   
    //acquire GPIOA
    let gpioa = dp.GPIOA.split(&mut rcc);

    //choose RX/TX pins
    let tx_pin = gpioa.pa2;
    let rx_pin = gpioa.pa3;

    // configure serial with LowPowerUART, default config is 9600 bps;
    let lpserial = dp.LPUART1.usart(tx_pin, rx_pin, serial::Config::default(), &mut rcc).unwrap();

    let (mut tx, mut _rx) = lpserial.split();

    // 'AI' part comes here:


    // error calculation done here: I want to move it to nn_lib.rs instead
    
    let mut prediction: f64 = 0.0;
    let mut error: f64 = 0.0;
    let mut up_prediction: f64 = 0.0;
    let mut up_error: f64 = 0.0;
    let mut down_prediction: f64 = 0.0;
    let mut down_error: f64 = 0.0;

    for _ in 0..epochs {
        prediction = input * weight;
        error = find_error_simple(prediction, expected_value);

        writeln!(tx, "Error: {:.3}      Prediction: {:.3}\r\n", error, prediction).unwrap();

        up_prediction = input * (weight + step_amount);
        up_error = find_error_simple(up_prediction, expected_value);

        down_prediction = input * (weight - step_amount);
        down_error = find_error_simple(down_prediction, expected_value);

        if down_error < up_error {
            weight = weight - step_amount;
        }

        if down_error > up_error {
            weight = weight + step_amount;
        }

    }


    loop {}

}