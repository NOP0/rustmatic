use rustmatic_core::{AccessType, Address, Process, System, Transition};
use rustmatic_dummy_input::{DummyBool, DummyU32};
use rustmatic_runtime::{Fault, Runtime};
use std::sync::Arc;

struct PlcMain {
    cycle_counter: u64,
    my_bool_1: bool, 
    my_int_2: u32,   
    my_int_3: u32,  
}

impl PlcMain {
    pub fn new(runtime: &mut Runtime) -> Self {

        // Register first input
        let my_input_1 = DummyBool::new();
        runtime.inputs.register_input_device(
            4,
            0,
            AccessType::Bit,
            runtime.devices.register(Arc::new(my_input_1)),
        );

        // Register second input
        let my_input_2 = DummyU32::new();
        runtime.inputs.register_input_device(
            8,
            0,
            AccessType::DoubleWord,
            runtime.devices.register(Arc::new(my_input_2)),
        );

        // Register third input
        let my_input_3 = DummyU32::new();
        runtime.inputs.register_input_device(
            12,
            0,
            AccessType::DoubleWord,
            runtime.devices.register(Arc::new(my_input_3)),
        );

        PlcMain {
            cycle_counter: 0,
            my_bool_1: false,
            my_int_2: 0,
            my_int_3: 0,
        }
    }
}

impl Process for PlcMain {
    type Fault = Fault;

    fn init(&mut self, system: &mut dyn System) -> Result<(), Self::Fault> {
        // Do some initializing
        println!("PlcMain was initialized");
        Ok(())
    }

    fn poll(&mut self, system: &mut dyn System) -> Transition<Self::Fault> {
        println!("PlcMain running!");

        let inputs = system.inputs(); // Get handle to ProcessImage Inputs

        self.my_bool_1 = inputs.read_bit(Address {
            byte_offset: 4,
            bit_offset: 0,
            type_of: AccessType::Bit,
        });

        self.my_int_2 = inputs.read_double_word(Address {
            byte_offset: 8,
            bit_offset: 0,
            type_of: AccessType::DoubleWord,
        });

        self.my_int_3 = inputs.read_double_word(Address {
            byte_offset: 12,
            bit_offset: 0,
            type_of: AccessType::DoubleWord,
        });


        println!("The value of my_bool is: {}", self.my_bool_1);
        println!("The value of my_int is: {}", self.my_int_2);
        println!("The value of my_int is: {}", self.my_int_3);

        self.cycle_counter += 1;

        if self.cycle_counter < 64000 {
            Transition::StillRunning
        } else {
            Transition::Completed
        }
    }
    // Transition::StillRunning
    //}
}
fn main() {
    let mut runtime = Runtime::new();

    let plc_main = PlcMain::new(&mut runtime);

    runtime.add_process(plc_main);

    runtime.init();

    while runtime.iter_processes().count() > 0 {
        let _ = runtime.poll();
    }
}
