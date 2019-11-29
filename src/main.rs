use rustmatic_core::{Process, System, Transition, InputHandle};
use rustmatic_runtime::{Fault, Runtime};
use rustmatic_gpio::{GpioPin};

    struct PlcMain{
        cycle_counter : i32,
        my_bool : InputHandle<bool>,
        my_float : InputHandle<f64>,
    }



    impl Process for PlcMain {
        type Fault = Fault;

        fn init(&mut self,
            system: &dyn System,
        ) -> Result<(), Self::Fault>{

            // Do some initializing

            println!("PlcMain was initialized");

            Ok(())
        }



        fn poll(
            &mut self,
            system: &dyn System,
        ) -> Transition<Self::Fault> {
            println!("PlcMain running!");

            let pi = system.process_image(); // Get handle to ProcessImage

            pi.write(self.my_bool, false);

            println!("my_bool is:{}", pi.read(self.my_bool));

            pi.write(self.my_bool, true);

            println!("my_bool is:{}", pi.read(self.my_bool));

            pi.write(self.my_float, 0.0);

            println!("my_float is:{}", pi.read(self.my_float));

            pi.write(self.my_float, 3.14);

            println!("my_float is:{}", pi.read(self.my_float));
            
            self.cycle_counter += 1;

            if self.cycle_counter < 2 {
                Transition::StillRunning
            }
            else {
                Transition::Completed
            }

        }
    }
fn main() {
    let mut runtime = Runtime::new();

    runtime.add_process( // FIXME: Ouch, there should be some cleaner way?
        PlcMain{
        cycle_counter: 0,
        my_bool: runtime.process_image.register_input::<bool>(false),
        my_float: runtime.process_image.register_input::<f64>(0.0),
    }
    );

    runtime.init(); // TODO: handle errors

    while runtime.iter_processes().count() > 0 {
    let _ = runtime.poll();
    }
}
