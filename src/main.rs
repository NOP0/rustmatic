pub mod plc;

fn main() {

    // bool example
    let mut pi_bool : plc::ProcessImageInputs<bool> = plc::ProcessImageInputs::new(1000);

    let my_bool = pi_bool.insert(true);

    println!("{:?}", pi_bool.get(my_bool).unwrap());

    *(pi_bool.get_mut(my_bool).unwrap()) = false; // TODO: Poor ergonomics!

    println!("{:?}", pi_bool.get(my_bool).unwrap());
    
    // float example
    let mut pi_float : plc::ProcessImageInputs<f64> = plc::ProcessImageInputs::new(1000);

    let my_float = pi_float.insert(10.0);

    println!("{:?}", pi_float.get(my_float).unwrap());

    *(pi_float.get_mut(my_float).unwrap()) = 13.0;

    println!("{:?}", pi_float.get(my_float).unwrap());
}

