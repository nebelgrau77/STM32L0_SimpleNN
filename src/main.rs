#![no_main]
#![no_std]

extern crate panic_halt;


mod helpers;

pub use crate::helpers::math::*;
pub use crate::helpers::nn::*;

use cortex_m_rt::entry;
//use stm32l0::stm32l0x1;
use stm32l0xx_hal::{
    pac,
    prelude::*,
    rcc::{Config,MSIRange},
    serial,
};

use core::fmt::Write;

use core::ptr::write_volatile;


const NUM_OF_INPUTS: u32 = 3;
const OUT_LEN: usize = 3;
const IN_LEN: usize = 3;


const TEMPERATURE_PREDICTION_IDX: usize = 0;      //indexes in the prediction array
const HUMIDITY_PREDICTION_IDX: usize = 1;
const AIRQUALITY_PREDICTION_IDX: usize = 2;


const  SAD_PREDICTION_IDX: usize = 0; 
const  SICK_PREDCITION_IDX: usize = 1;
const  ACTIVE_PREDICTION_IDX: usize = 2;


#[entry]
fn main() -> ! {

    //let temperature: [f64; 5] = [12.0,23.0,50.0,-10.0,16.0];
    //let humidity: [f64; 5] = [60.0,67.0,50.0,65.0,63.0];
    //let air_quality: [f64; 5] = [60.0,47.0,167.0,187.0,94.0];
    //let weights: [f64; 3] = [-2.0,2.0,1.0];

    //let weights: [f64; 3] = [-20.0, 95.0, 201.0]; // weights for [temperature, humidity, air_quality]


    let weights: [[f64;3];3] = [[-2.0,9.5,2.01],  //sad or happy?          
                                [-0.8,7.2,6.3],   //sick or healthy?
                                [-0.5,0.45,0.9]]; //active or inactive?

    
    // double input_vector[IN_LEN] = { 30.0,87.0,110.0};

    let input_vector: [f64; 3] = [30.0,87.0,110.0];  //[temperature, humidity, air_quality]


    // let sad: f64 = 0.9;         // single input data

    
    let mut predicted_results:[f64; 3] = [0_f64; 3]; //initial array for predicted results

    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
   

    let mut status = "not ready";

    dp.RCC.csr.modify(|_,w| w.lsedrv().low()); //explicitely set LSE drive capacity to low

    dp.RCC.csr.modify(|_,w| w.lseon().on()); // turn Low Speed External oscillator on
    while dp.RCC.csr.read().lserdy().is_not_ready() {      
        // wait until LSE is ready
    }

    /*
    if dp.RCC.csr.read().lserdy().is_ready() {
        status = "ready!";
    }
    */

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

    let mut value: i32 = 0;



    // single_input_multiple_output_nn(Sad,weights,predicted_results,OUT_LEN);

    /*
    single_in_multiple_out_nn(sad, weights, &mut predicted_results, OUT_LEN);

    writeln!(tx, "Predicted temperature is: {:.3}\r", predicted_results[TEMPERATURE_PREDICTION_IDX]).unwrap();
    writeln!(tx, "Predicted humidity is: {:.3}\r", predicted_results[HUMIDITY_PREDICTION_IDX]).unwrap();
    writeln!(tx, "Predicted air quality is: {:.3}\r", predicted_results[AIRQUALITY_PREDICTION_IDX]).unwrap();

    */

    multiple_in_multiple_out(input_vector, IN_LEN, &mut predicted_results, OUT_LEN, weights);

    writeln!(tx, "Sad/happy prediction: {:.3}\r", predicted_results[SAD_PREDICTION_IDX]).unwrap();
    writeln!(tx, "Sick/healthy prediction: {:.3}\r", predicted_results[SICK_PREDCITION_IDX]).unwrap();
    writeln!(tx, "Active/inactive prediction: {:.3}\r", predicted_results[ACTIVE_PREDICTION_IDX]).unwrap();

    /*

    for i in 0..temperature.len() as usize {

        let training: [f64; 3] = [temperature[i], humidity[i], air_quality[i]];

        let prediction: f64 = multiple_inputs_single_output_nn(training, weights, NUM_OF_INPUTS);

        writeln!(tx, "Prediction on training example no. {}: {}\r", i, prediction).unwrap();

    }
    
    */

    loop {

        // writeln!(tx, "Status: {}\r", status).unwrap();

        /*
        writeln!(tx, "Current value is: {}, squared: {}, cubed: {}\r", value, square(value), cube(value)).unwrap();
        value += 1;
        */


    }


}