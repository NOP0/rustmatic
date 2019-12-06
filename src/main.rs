use rustmatic_core::{Input, Output, Process, System, Transition};
use rustmatic_dummy_input::DummyInput;
use rustmatic_runtime::{Fault, Runtime};
use std::sync::Arc;

struct PlcMain {
    cycle_counter: i32,
    my_bool: Input<bool>, // Define an process image input of type bool
    my_float: Input<f64>, // Define an process image input of type float
}

impl PlcMain {
    pub fn new(runtime: &mut Runtime) -> Self {
        let my_bool = runtime.process_image.register_input::<bool>(false);
        let my_float = runtime.process_image.register_input::<f64>(0.0);

        let my_dummy_input = DummyInput::new(my_bool.get_number());
        let handle_to_dummy_input =
            runtime.devices.register(Arc::new(my_dummy_input));

        runtime
            .process_image
            .register_input_device(my_bool, handle_to_dummy_input);

        PlcMain {
            cycle_counter: 0,
            my_bool,
            my_float,
        }
    }
}

impl Process for PlcMain {
    type Fault = Fault;

    fn init(&mut self, system: &dyn System) -> Result<(), Self::Fault> {
        // Do some initializing
        println!("PlcMain was initialized");
        Ok(())
    }

    fn poll(&mut self, system: &dyn System) -> Transition<Self::Fault> {
        println!("PlcMain running!");

        let pi = system.process_image(); // Get handle to ProcessImage

        println!("The value of my_bool is: {}", pi.read(self.my_bool));

        self.cycle_counter += 1;

        //     if self.cycle_counter < 100000 {
        //         Transition::StillRunning
        //     } else {
        //         Transition::Completed
        //     }
        // }
        Transition::StillRunning
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
