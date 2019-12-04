use rustmatic_core::{Input, Output, Process, System, Transition};
use rustmatic_dummy_input::{DummyInput};
use rustmatic_runtime::{Fault, Runtime};
use std::sync::Arc;

struct PlcMain {
    cycle_counter: i32,
    my_bool: Input<bool>, // Define an process image input of type bool
    my_float: Input<f64>, // Define an process image input of type float
    my_dummy_input : DummyInput, // Define an dummy input
}

impl PlcMain {
    pub fn new(runtime : &mut Runtime) -> Self {

        let my_bool = runtime.process_image.register_input::<bool>(false);
        let my_float =  runtime.process_image.register_input::<f64>(0.0);

        let my_dummy_input = DummyInput::new(my_bool.get_number());

      
        PlcMain {
            cycle_counter: 0,
            my_bool: my_bool,
            my_float: my_float,
            my_dummy_input : my_dummy_input,
        }
        

    }
}

impl Process for PlcMain {
    type Fault = Fault;

    fn init(&mut self, system: &dyn System) -> Result<(), Self::Fault> {

        // Do some initializing
        println!("PlcMain was initialized");

        let devices = system.devices();

        // FIXME: How to register devices, devicemanager is &
        self.my_dummy_input.register(devices);
        let _ = devices.register(Arc::new(self.my_dummy_input));

        Ok(())
    }

    fn poll(&mut self, system: &dyn System) -> Transition<Self::Fault> {
        println!("PlcMain running!");

        let pi = system.process_image(); // Get handle to ProcessImage

        println!("{}",pi.read(self.my_bool));

        self.cycle_counter += 1;

        if self.cycle_counter < 2 {
            Transition::StillRunning
        } else {
            Transition::Completed
        }
    }
}
fn main() {
    let mut runtime = Runtime::new();

    let plc_main = PlcMain::new(&mut runtime);

    runtime.add_process(plc_main);

    runtime.init(); // TODO: handle errors

    while runtime.iter_processes().count() > 0 {
        let _ = runtime.poll();
    }
}
