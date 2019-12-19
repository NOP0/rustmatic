use rustmatic_core::{AccessType, Process, System, Transition};
use rustmatic_runtime::{Fault, Runtime};
use rustmatic_gpio::GpioPin;
use std::sync::Arc;
use sysfs_gpio::Pin;

struct PlcMain {
    cycle_counter: u64,
    my_bool: bool,
}

impl PlcMain {
    pub fn new(runtime: &mut Runtime) -> Self {

        let my_gpio = GpioPin::input(Pin::new(17));

        // Register this input at offset %I4.0 in input Process Image
        runtime.inputs.register_input_device(
            4,
            0,
            AccessType::Bit,
            runtime.devices.register(Arc::new(my_gpio)),
        );

        PlcMain {
            cycle_counter: 0,
            my_bool: false,
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

            self.my_bool = inputs.read_bit(4, 0);
        }

        // Do something with them.
        println!("The value of my_bool is: {}", self.my_bool);

        Transition::StillRunning
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
