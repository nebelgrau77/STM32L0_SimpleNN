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

const NUM_FEATURES: usize = 2;
const NUM_EXAMPLES: usize = 3;
const NUM_HIDDEN_NODES: usize = 3;
const NUM_OUTPUT_NODES: usize = 1;

pub type Float = f32;   //this way I can easily replace it with double precision f64 if necessary

#[entry]
fn main() -> ! {
    
    // neural network setup

    /*
    let num_of_features = 2;
    let num_of_examples = 3;

    let num_of_hidden_nodes = 3;
    let num_of_output_nodes = 1;
    */
    

    /*
    train X:

        2 5 1
        8 5 8

        dim = nx X m

    */

    let raw_x: [Float;NUM_FEATURES * NUM_EXAMPLES] = [2.0,5.0,1.0,
                                                    8.0,5.0,8.0]; // raw_x[num_of_features][num_of examples]
    

    /*
    train y:

        200 90 190
        
        dim = 1 X m

    */



    let raw_y: [Float;NUM_EXAMPLES] = [200.0, 90.0, 190.0]; //raw_y[1][num]


    let mut train_x = [0.0; NUM_FEATURES*NUM_EXAMPLES];
    let mut train_y = [0.0;NUM_EXAMPLES];

    let mut train_x_eg1 = [0.0; NUM_FEATURES];
    let mut train_y_eg1 = 0.0;

    let mut z1_eg1 = [0.0; NUM_HIDDEN_NODES];
    let mut a1_eg1 = [0.0; NUM_HIDDEN_NODES];
    let mut z2_eg1 = 0.0;
    let mut yhat_eg1 = 0.0;

    /*

    // hours of workout data
    let x1: [Float;3] = [2.0,5.0,1.0];

    let mut x1_norm: [Float;3] = [0.0;3]; // normalized

    // hours of rest data
    let x2: [Float;3] = [8.0,5.0,8.0];

    let mut x2_norm: [Float;3] = [0.0;3]; // normalized

    // muscle gain (output)
    let y: [Float;3] = [200.0, 90.0, 190.0];
    
    let mut y_norm: [Float;3] = [0.0;3]; // normalized
    
    */

    // input layer to hidden layer weights matrix

    let mut syn0 = [0.0; NUM_HIDDEN_NODES*NUM_FEATURES]; // number of hidden nodes * number of features, ideally [[f64;3];2]

    // hidden layer to output layer weights matrix

    let mut syn1 = [0.0; NUM_OUTPUT_NODES * NUM_HIDDEN_NODES]; // number of output nodes * number of hidden nodes




    /*
    let input_vector: [f64; 3] = [30.0,87.0,110.0];  //[temperature, humidity, air_quality]
    let mut predicted_results:[f64; 3] = [0_f64; 3]; //initial array for predicted results

    let expected_values: [f64; 3] = [600.0, 10.0, -90.0]; //i.e. y values
    */
    /*
    let mut weight: f64 = 0.5;
    let input: f64 = 0.5;
    let expected_value: f64 = 0.8;
    let step_amount: f64 = 0.001;
    let epochs: u32 = 1500;
    */
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


    
    //let mut rng = SmallRng::seed_from_u64(0xdead_beef_cafe_d00d);

    //let mut choice = rng.next_u32();
    

    // 'AI' part comes here:

    normalize_data(&raw_x, &mut train_x, NUM_EXAMPLES);

    // brute_force_learning(input, weight, expected_value, step_amount, 1500, &mut tx);

    /*
    normalize_data(&x1, &mut x1_norm, num_of_examples);
    normalize_data(&x2, &mut x2_norm, num_of_examples);
    normalize_data(&y, &mut y_norm, num_of_examples);

    writeln!(tx, "\nRaw x1 data:\t").unwrap();

    for i in 0..num_of_examples {
        write!(tx, "{}\t", x1[i]).unwrap();
    }

    writeln!(tx, "\nNormalized x1 data:\t").unwrap();

    for i in 0..num_of_examples {
        write!(tx, "{}\t", x1_norm[i]).unwrap();
    }

    // repeat for x2 and y

    */

    /*

    weights_random_initialization(NUM_HIDDEN_NODES, NUM_FEATURES, &mut syn0);
    weights_random_initialization(NUM_OUTPUT_NODES, NUM_HIDDEN_NODES, &mut syn1);

    // synapse 0 weights
    
    writeln!(tx, "\nSynapse 0 weights (input to hidden):").unwrap();
    
    for i in 0..NUM_HIDDEN_NODES {
        for j in 0..NUM_FEATURES {
            writeln!(tx, "{}\t", syn0[i*NUM_FEATURES + j]).unwrap();
        }
    }

    writeln!(tx, "\nSynapse 1 weights (hidden to output):").unwrap();

    // synapse 1 weights
    for i in 0..NUM_OUTPUT_NODES {
        for j in 0..NUM_HIDDEN_NODES {
            writeln!(tx, "{}\t", syn0[i*NUM_HIDDEN_NODES + j]).unwrap();
        }
    }

    */

    loop {}

}