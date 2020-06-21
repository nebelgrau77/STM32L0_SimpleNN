#![no_main]
#![no_std]

extern crate panic_halt;


mod nn_lib;

pub use crate::nn_lib::nn::*;

use cortex_m_rt::entry;
use stm32l0xx_hal::{
    pac,
    prelude::*,
    rcc::{Config,MSIRange},
    serial,
};

use core::fmt::Write;

use core::ptr::write_volatile;

const IN_LEN: usize = 3;
const OUT_LEN: usize = 3;
const HID_LEN: usize = 3;



const TEMPERATURE_PREDICTION_IDX: usize = 0;      //indexes in the prediction array
const HUMIDITY_PREDICTION_IDX: usize = 1;
const AIRQUALITY_PREDICTION_IDX: usize = 2;


const  SAD_PREDICTION_IDX: usize = 0; 
const  SICK_PREDCITION_IDX: usize = 1;
const  ACTIVE_PREDICTION_IDX: usize = 2;


#[entry]
fn main() -> ! {
    
    let input_to_hidden_weights: [[f64;3];3] = [[-2.0,9.5,2.01], // hid[0]
                                               [-0.8,7.2,6.3],   // hid[1]
                                               [-0.5,0.45,0.9]]; // hid[2]

                                // input from [hid[0], hid[1], hid[2]
   let hidden_to_output_weights: [[f64;3];3] = [[-1.0,1.15,0.11], // sad or happy
                                              [-0.18,0.15,-0.01], // sick or healthy
                                              [0.25,-0.25,-0.1]]; // active or inactive
                            
    
    let input_vector: [f64; 3] = [30.0,87.0,110.0];  //[temperature, humidity, air_quality]

    let mut predicted_results:[f64; 3] = [0_f64; 3]; //initial array for predicted results

    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut status = "not ready";

    dp.RCC.csr.modify(|_,w| w.lsedrv().low()); //explicitely set LSE drive capacity to low

    dp.RCC.csr.modify(|_,w| w.lseon().on()); // turn Low Speed External oscillator on
    while dp.RCC.csr.read().lserdy().is_not_ready() {      
        // wait until LSE is ready
    }

    dp.RCC.ccipr.modify(|_,w| w.lpuart1sel().lse());   // set LSE as clock for LPUART1

    //configure the clock
    let mut rcc = dp.RCC.freeze(Config::msi(MSIRange::Range2)); //Range4 means around 1.048 MHz
   
    //acquire GPIOA
    let gpioa = dp.GPIOA.split(&mut rcc);

    //choose RX/TX pins
    let tx_pin = gpioa.pa2;
    let rx_pin = gpioa.pa3;

    //configure serial with LowPowerUART (default config is 9600 bps)    
    let mut lpserial = dp.LPUART1.usart(tx_pin, rx_pin, serial::Config::default(), &mut rcc).unwrap();
   
    const BBR: *mut u32 = (0x40004800 + 0x00C) as *mut u32; //pointer to LPUART_BBR register
 
    unsafe {
        write_volatile(BBR, 0x369)          // 0x369 is the LPUARTDIV value for 9600 bps, 0xda7 is 2400 bps etc. (page 731 in RM)
    }
    
    let (mut tx, mut _rx) = lpserial.split();

    // 'AI' part comes here: predicting multiple output with hidden layer 

    hidden_nn(&input_vector, IN_LEN, HID_LEN, input_to_hidden_weights, OUT_LEN, hidden_to_output_weights, &mut predicted_results);

    writeln!(tx, "Sad/happy prediction: {:.3}\r", predicted_results[SAD_PREDICTION_IDX]).unwrap();
    writeln!(tx, "Sick/healthy prediction: {:.3}\r", predicted_results[SICK_PREDCITION_IDX]).unwrap();
    writeln!(tx, "Active/inactive prediction: {:.3}\r", predicted_results[ACTIVE_PREDICTION_IDX]).unwrap();

    loop {
    }

}