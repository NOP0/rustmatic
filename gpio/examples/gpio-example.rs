use gpio_cdev::Chip;
use rustmatic_core::{PiAccess, Process, System, Transition};
use rustmatic_gpio::GpioPin;
use rustmatic_runtime::{Fault, Runtime};
use std::{sync::Arc, time::Instant};

struct PlcMain {
    cycle_counter: u64,
    my_bool: bool,
    created: Instant,
}

impl PlcMain {
    pub fn new(runtime: &mut Runtime) -> Self {
        // This example uses GPIO 21 on a Rasberry PI 2B. Adjust the GPIO number
        // for your application.
        let my_gpio = GpioPin::input(Chip::new("/dev/gpiochip0").unwrap(), 21);

        // Register this input at offset %I4.0 in input Process Image
        runtime.inputs.register_device::<bool>(
            (4, 0),
            runtime.devices.register(Arc::new(my_gpio)),
        );

        // This example uses GPIO 20 on a Rasberry PI 2B. Adjust the GPIO number
        // for your application.
        let my_gpio_2 =
            GpioPin::output(Chip::new("/dev/gpiochip0").unwrap(), 20);

        // Register this output at offset %Q4.0 in input Process Image
        runtime.outputs.register_device::<bool>(
            (4, 0),
            runtime.devices.register(Arc::new(my_gpio_2)),
        );

        PlcMain {
            cycle_counter: 0,
            my_bool: false,
            created: Instant::now(),
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

        {
            let inputs = system.inputs();

            self.my_bool = <bool>::read(inputs, (4, 0));
        }

        {
            let outputs = system.outputs();

            let elapsed = self.created.elapsed().as_secs();

            if elapsed % 5 != 0 {
                <bool>::write(outputs, (4, 0), false);
            } else {
                <bool>::write(outputs, (4, 0), true);
            }
        }

        println!("The value of my_bool is: {}", self.my_bool);

        self.cycle_counter += 1;

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
