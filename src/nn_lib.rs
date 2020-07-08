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

    fn elementwise_multiply(input_scalar: f64, weight_vector: &[f64], output_vector: &mut [f64], vector_len: usize) {

        for i in 0..vector_len {
            output_vector[i] = input_scalar * weight_vector[i];
        }

    }

    pub fn single_in_multiple_out_nn(input_scalar: f64, weight_vector: &[f64], output_vector: &mut [f64], vector_len: usize) {

        elementwise_multiply(input_scalar, &weight_vector, output_vector, vector_len);

    } 


    fn matrix_vector_multiplication(input_vector: &[f64], 
                                    input_len: usize, 
                                    output_vector: &mut [f64], 
                                    output_len: usize, 
                                    weights_matrix: &[f64]) {
        
        for k in 0..output_len {
            for i in 0..input_len {
                output_vector[k] += input_vector[i] * weights_matrix[k*3+i]
            }
        }
    }



    pub fn multiple_in_multiple_out(input_vector: &[f64], 
        input_len: usize, 
        output_vector: &mut [f64], 
        output_len: usize, 
        weights_matrix: &[f64]) {
    
        matrix_vector_multiplication(&input_vector, input_len, output_vector, output_len, &weights_matrix);

    }

    

    pub fn hidden_nn (input_vector: &[f64],
        input_len: usize,
        hidden_len: usize,
        input_to_hidden_weights: &[f64],
        output_len: usize,
        hidden_to_output_weights: &[f64],
        output_vector: &mut [f64]){

        let mut hidden_predicted: [f64; 3] = [0_f64; 3];
        matrix_vector_multiplication(&input_vector, input_len, &mut hidden_predicted, output_len, input_to_hidden_weights);
        matrix_vector_multiplication(&hidden_predicted, hidden_len, output_vector, output_len, hidden_to_output_weights);

        }


    pub fn find_error_simple(yhat: f64, y: f64) -> f64 {

        let temp: f64 = yhat-y;
        return temp * temp; //x.powf(y) not available in no_std
        
    }

    pub fn find_error(input: f64, weight: f64, expected_value: f64) -> f64 {

        let temp: f64 = (input * weight) - expected_value; 
        return temp * temp; //x.powf(y) not available
        
    }


    pub fn brute_force_learning(input: f64, mut weight: f64, expected_value: f64, step_amount: f64, epochs: u32) -> (f64, f64) {
        let mut prediction: f64 = 0.0;
        let mut error: f64 = 0.0;
        let mut up_prediction: f64 = 0.0;
        let mut up_error: f64 = 0.0;
        let mut down_prediction: f64 = 0.0;
        let mut down_error: f64 = 0.0;

        for _ in 0..epochs {
            prediction = input * weight;
            error = find_error_simple(prediction, expected_value);

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

        return (prediction, error);

    }


    }




/*

// AN ALTERNATIVE APPROACH AS SUGGESTED BY A MORE EXPERIENCED RUSTACEAN:

fn foo<'a, T, U>(
    iter: T
) 
    where T: IntoIterator<Item = U>,
    U: IntoIterator<Item = &'a f32>
{

    for (x, t) in iter.into_iter().enumerate() {
        for (y, u) in t.into_iter().enumerate() {
            println!("{}/{}: {}", x, y, u);
        }
    }
}

fn main(){
    let values: [[f32; 3]; 3] = [
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
        [7.0, 8.0, 9.0]
    ];
    
    foo(&values)
}

// Basically it takes any type T that implements IntoIterator, which yields U
// Then U can be any type that implements IntoIterator which yields &f32
// &[U] implements IntoIterator<Item = &U> and &[f32] implements IntoIterator<Item = &f32>

*/