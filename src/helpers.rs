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


    

    fn elementwise_multiply(input_scalar: f64, weight_vector: [f64; 3], output_vector: &mut [f64;3], vector_len: usize) {

        for i in 0..vector_len {
            output_vector[i] = input_scalar * weight_vector[i];
        }


    }


    pub fn single_in_multiple_out_nn(input_scalar: f64, weight_vector: [f64; 3], output_vector: &mut [f64;3], vector_len: usize) {

        elementwise_multiply(input_scalar, weight_vector, output_vector, vector_len);

    } 


    fn matrix_vector_multiplication(input_vector: [f64;3], 
                                    INPUT_LEN: usize, 
                                    output_vector: &mut [f64; 3], 
                                    OUTPUT_LEN: usize, 
                                    weights_matrix: [[f64; 3];3]) {
        for k in 0..OUTPUT_LEN {
            for i in 0..INPUT_LEN {
                output_vector[k] += input_vector[i] * weights_matrix[k][i]
            }
        }
    }

    pub fn multiple_in_multiple_out(input_vector: [f64;3], 
                                    INPUT_LEN: usize, 
                                    output_vector: &mut [f64; 3], 
                                    OUTPUT_LEN: usize, 
                                    weights_matrix: [[f64; 3];3]) {
        matrix_vector_multiplication(input_vector, INPUT_LEN, output_vector, OUTPUT_LEN, weights_matrix);
    }

}



