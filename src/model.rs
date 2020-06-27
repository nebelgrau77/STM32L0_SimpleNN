pub mod model {

    // neural network size

    pub const IN_LEN: usize = 3;                          // input nodes
    pub const OUT_LEN: usize = 3;                         // output nodes  
    pub const HID_LEN: usize = 3;                         // hidden layer nodes  

    // indexes of the vector elements

    pub const TEMPERATURE_PREDICTION_IDX: usize = 0;      //indexes in the prediction array
    pub const HUMIDITY_PREDICTION_IDX: usize = 1;
    pub const AIRQUALITY_PREDICTION_IDX: usize = 2;

    pub const  SAD_PREDICTION_IDX: usize = 0; 
    pub const  SICK_PREDCITION_IDX: usize = 1;
    pub const  ACTIVE_PREDICTION_IDX: usize = 2;
       
    // weights

    pub const INPUT_TO_HIDDEN_WEIGHTS: [[f64;3];3] = [[-2.0,9.5,2.01], // hid[0]
                                                     [-0.8,7.2,6.3],   // hid[1]
                                                     [-0.5,0.45,0.9]]; // hid[2]
    
    pub const HIDDEN_TO_OUTPUT_WEIGHTS: [[f64;3];3] = [[-1.0,1.15,0.11], // sad or happy
                                                       [-0.18,0.15,-0.01], // sick or healthy
                                                       [0.25,-0.25,-0.1]]; // active or inactive


}

