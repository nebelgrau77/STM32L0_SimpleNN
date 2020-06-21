pub mod math {

    pub fn square(value: i32) -> i32 {
        return value * value
    }

    pub fn cube(value: i32) -> i32 {
        return value * value * value
    }
}

pub mod nn {

    fn weighted_sum(input: [f64;3], weight: [f64; 3], input_len: u32) -> f64 {

        let mut output: f64 = 0.0;

        for i in 0..input_len as usize {
            output += input[i] * weight[i];
        }

        return output; 

    }

    pub fn multiple_inputs_single_output_nn(input: [f64;3], weight: [f64; 3], input_len: u32) -> f64 {

        let mut predicted_value: f64 = 0.0;

        predicted_value = weighted_sum(input, weight, input_len);

        return predicted_value;

    }


    
    /*
    fn elementwise_multiply(input_scalar: f64, weight_vector: [f64; 3], output_vector: &mut [f64;3], vector_len: usize) {

        for i in 0..vector_len {
            output_vector[i] = input_scalar * weight_vector[i];
        }


    }
    */

    fn elementwise_multiply(input_scalar: f64, weight_vector: &[f64], output_vector: &mut [f64], vector_len: usize) {

        for i in 0..vector_len {
            output_vector[i] = input_scalar * weight_vector[i];
        }


    }

    pub fn single_in_multiple_out_nn(input_scalar: f64, weight_vector: &[f64], output_vector: &mut [f64], vector_len: usize) {

        elementwise_multiply(input_scalar, &weight_vector, output_vector, vector_len);

    } 


    fn matrix_vector_multiplication(input_vector: &[f64], 
                                    INPUT_LEN: usize, 
                                    output_vector: &mut [f64], 
                                    OUTPUT_LEN: usize, 
                                    weights_matrix: [[f64; 3];3]) {
                                    
        for k in 0..OUTPUT_LEN {
            for i in 0..INPUT_LEN {
                output_vector[k] += input_vector[i] * weights_matrix[k][i]
            }
        }
    }

    pub fn multiple_in_multiple_out(input_vector: &[f64], 
                                    INPUT_LEN: usize, 
                                    output_vector: &mut [f64], 
                                    OUTPUT_LEN: usize, 
                                    weights_matrix: [[f64; 3];3]) {
                                    
        matrix_vector_multiplication(&input_vector, INPUT_LEN, output_vector, OUTPUT_LEN, weights_matrix);
    }

    
    pub fn hidden_nn (input_vector: &[f64],
                      INPUT_LEN: usize,
                      HIDDEN_LEN: usize,
                      input_to_hidden_weights: [[f64;3];3],
                      OUTPUT_LEN: usize,
                      hidden_to_output_weights: [[f64;3];3],
                      output_vector: &mut [f64]){
        
        let mut hidden_predicted: [f64; 3] = [0_f64; 3];
        matrix_vector_multiplication(&input_vector, INPUT_LEN, &mut hidden_predicted, OUTPUT_LEN, input_to_hidden_weights);
        matrix_vector_multiplication(&hidden_predicted, HIDDEN_LEN, output_vector, OUTPUT_LEN, hidden_to_output_weights);
        
    }
    
}


/*
//hidden layer

weights: [[f64; x];2] = [[input_to_hidden], [hidden_to_output]];

hidden_layer(input, weights, predicted values)  {
    vector_matrix_multiply(input, weights[0], hidden_pred)
    vector_matrix_multiply(hidden_pred, weights[1], predicted_values)
}



*/

