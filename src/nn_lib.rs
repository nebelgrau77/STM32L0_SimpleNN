//pub type Float = f32;   //this way I can easily replace it with double precision f64 if necessary

pub mod nn {


    /*

    fn square(num: Float) -> Float {        // just a test of the type alias
        return num * num;
    }

    */

    fn weighted_sum(input: [f32;3], weight: [f32; 3], input_len: usize) -> f32 {

        let mut output: f32 = 0.0;

        for i in 0..input_len {
            output += input[i] * weight[i];
        }

        return output; 

    }

    pub fn multiple_inputs_single_output_nn(input: [f32;3], weight: [f32; 3], input_len: usize) -> f32 {

        let mut predicted_value: f32 = 0.0;

        predicted_value = weighted_sum(input, weight, input_len);

        return predicted_value;

    }

    fn elementwise_multiply(input_scalar: f32, weight_vector: &[f32], output_vector: &mut [f32], vector_len: usize) {

        for i in 0..vector_len {
            output_vector[i] = input_scalar * weight_vector[i];
        }

    }

    pub fn single_in_multiple_out_nn(input_scalar: f32, weight_vector: &[f32], output_vector: &mut [f32], vector_len: usize) {

        elementwise_multiply(input_scalar, &weight_vector, output_vector, vector_len);

    } 


    fn matrix_vector_multiplication(input_vector: &[f32], 
                                    input_len: usize, 
                                    output_vector: &mut [f32], 
                                    output_len: usize, 
                                    weights_matrix: &[f32]) {
        
        for k in 0..output_len {
            for i in 0..input_len {
                output_vector[k] += input_vector[i] * weights_matrix[k*3+i]
            }
        }
    }



    pub fn multiple_in_multiple_out(input_vector: &[f32], 
        input_len: usize, 
        output_vector: &mut [f32], 
        output_len: usize, 
        weights_matrix: &[f32]) {
    
        matrix_vector_multiplication(&input_vector, input_len, output_vector, output_len, &weights_matrix);

    }

    
    /*
    pub fn hidden_nn (input_vector: &[f32],
        input_len: usize,
        hidden_len: usize,
        input_to_hidden_weights: &[f32],
        output_len: usize,
        hidden_to_output_weights: &[f32],
        output_vector: &mut [f32]){

                    
        let mut hidden_predicted = [0f32; 3]; //this needs to be fixed; must use vector and not array
        
        matrix_vector_multiplication(&input_vector, input_len, &mut hidden_predicted, output_len, input_to_hidden_weights);
        matrix_vector_multiplication(&hidden_predicted, hidden_len, output_vector, output_len, hidden_to_output_weights);

        }
    */

    pub fn hidden_nn (input_vector: &[f32],
        input_len: usize,
        hidden_len: usize,
        input_to_hidden_weights: &[f32],
        output_len: usize,
        hidden_to_output_weights: &[f32],
        mut buffer: &mut [f32], // empty buffer of the same size as hidden 
        output_vector: &mut [f32]){


        matrix_vector_multiplication(&input_vector, input_len, &mut buffer, output_len, input_to_hidden_weights);
        matrix_vector_multiplication(&buffer, hidden_len, output_vector, output_len, hidden_to_output_weights);

        }

    



    pub fn find_error_simple(yhat: f32, y: f32) -> f32 {

        let temp: f32 = yhat-y;
        return temp * temp; //x.powf(y) not available in no_std
        
    }

    pub fn find_error(input: f32, weight: f32, expected_value: f32) -> f32 {

        let temp: f32 = (input * weight) - expected_value; 
        return temp * temp; //x.powf(y) not available
        
    }

    
    pub fn brute_force_learning(input: f32,
                                mut weight: f32,
                                expected_value: f32,
                                step_amount: f32,
                                epochs: u32, 
                                tx: &mut stm32l0xx_hal::serial::Tx<stm32l0::stm32l0x1::LPUART1> ) {

        let mut prediction: f32 = 0.0;
        let mut error: f32 = 0.0;
        let mut up_prediction: f32 = 0.0;
        let mut up_error: f32 = 0.0;
        let mut down_prediction: f32 = 0.0;
        let mut down_error: f32 = 0.0;

        for _ in 0..epochs {
            prediction = input * weight;
            error = find_error_simple(prediction, expected_value);
            use core::fmt::Write;
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

        

    }


    pub fn normalize_data(input_vector: &[f32], output_vector: &mut [f32], len: usize) {
        //find max
        let mut max: f32 = input_vector[0];
        for i in 0..len {
            if(input_vector[i]) > max {
                max = input_vector[i];
            }
        }

        // normalize: divide each item by max
        for i in 0..len {
            output_vector[i] = input_vector[i] / max;
        }

    }


    use rand::prelude::*;

    const U32_MAX: f32 = 4294967295.0; // u32::MAX

    pub fn weights_random_initialization(hidden_len: usize, input_len: usize, weights_matrix: &mut [f32]) {
        
        //weights_matrix[hidden_len][input_len]

        let mut rng = SmallRng::seed_from_u64(0xdead_beef_cafe_d00d);

        for i in 0..hidden_len {
            for j in 0..input_len {

                // generate random numbers between 0 and 1

                let mut rand_val = rng.next_u32() as f32;

                rand_val = rand_val / U32_MAX;

                weights_matrix[i*input_len + j] = rand_val;
                

            }

        }

    }


    use micromath::F32Ext;      // used for exp function

    fn sigmoid(x: f32) -> f32 {
        let result: f32 = 1.0 / (1.0 + F32Ext::exp(-x));

        result
    }

    pub fn vector_sigmoid(input_vector: &[f32], output_vector: &mut [f32], len: usize) {
        for i in 0..len {
            output_vector[i] = sigmoid(input_vector[i]);
        }
    }

}
