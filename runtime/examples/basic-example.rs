use rustmatic_core::{PiAccess, Process, System, Transition};
use rustmatic_dummy_input::{DummyBool, DummyDoubleWord};
use rustmatic_runtime::{Fault, Runtime};
use std::sync::Arc;

struct PlcMain {
    cycle_counter: u64,
    my_bool: bool,
    my_double_word: u32,
}

impl PlcMain {
    pub fn new(runtime: &mut Runtime) -> Self {
        // DummyBool is a faked input device generating signal changes
        let my_bool_input = DummyBool::new();

        // Register this input at offset %I4.0 in input Process Image
        runtime.inputs.register_device::<bool>(
            (4, 0),
            runtime.devices.register(Arc::new(my_bool_input)),
        );

        // DummyDoubleWord is a faked input device generating signal changes
        let my_double_word_input = DummyDoubleWord::new();

        // Register this input at offset %ID8 in input Process Image
        runtime.inputs.register_device::<u32>(
            8,
            runtime.devices.register(Arc::new(my_double_word_input)),
        );

        PlcMain {
            cycle_counter: 0,
            my_bool: false,
            my_double_word: 0,
        }
    }
}

impl Process for PlcMain {
    type Fault = Fault;

    fn init(&mut self, _system: &mut dyn System) -> Result<(), Self::Fault> {
        // Do some initializing
        println!("PlcMain was initialized");
        Ok(())
    }

    fn poll(&mut self, system: &mut dyn System) -> Transition<Self::Fault> {
        println!("PlcMain running cycle #{}", self.cycle_counter);

        // Read all inputs
        {
            let inputs = system.inputs();

            self.my_bool = <bool>::read(inputs, (4, 0));
            self.my_double_word = <u32>::read(inputs, 8);
        }

        // Do something with them.
        println!("The value of my_bool is: {}", self.my_bool);
        println!("The value of my_double_word is: {}", self.my_double_word);

        // Write all outputs
        let outputs = system.outputs();

        // Write to offset %Q0.0 in output Process Image
        <bool>::write(outputs, (0, 0), self.cycle_counter % 2 != 0);

        println!("The value of output is: {}", <bool>::read(outputs, (0, 0)));

        self.cycle_counter += 1;

        if self.cycle_counter < 64000 {
            Transition::StillRunning
        } else {
            println! {"64000 cycles completed."}
            Transition::Completed
        }
    }
}
fn main() {
    let mut runtime = Runtime::new();

    let plc_main = PlcMain::new(&mut runtime);

    runtime.add_process(plc_main);

    runtime.init().expect("Could not init runtime");

    while runtime.iter_processes().count() > 0 {
        let _ = runtime.poll();
    }
}
