use rustmatic_core::{Process, ProcessImage, System, Transition};
use rustmatic_runtime::{Fault, Runtime};

    struct PlcMain{
        cycle_counter : i32,
    }

    impl Process for PlcMain {
        type Fault = Fault;

        fn poll(
            &mut self,
            _system: &dyn System,
            pi: &mut ProcessImage,
        ) -> Transition<Self::Fault> {
            println!("PlcMain running!");

            let my_bool = pi.register_input(false);

            println!("my_bool is:{}", pi.read::<bool>(my_bool));

            pi.write(my_bool, true);

            println!("my_bool is:{}", pi.read::<bool>(my_bool));

            let my_float = pi.register_input(0.0);

            println!("my_float is:{}", pi.read::<f64>(my_float));

            pi.write(my_float, 3.14);

            println!("my_float is:{}", pi.read::<f64>(my_float));
            
            self.cycle_counter += 1;

            if self.cycle_counter < 1000 {
                Transition::StillRunning
            }
            else {
                Transition::Completed
            }

        }
    }
fn main() {
    let mut runtime = Runtime::new();

    runtime.add_process(PlcMain{cycle_counter: 0});

    while runtime.iter_processes().count() > 0 {
    let _ = runtime.poll();
    }
}
