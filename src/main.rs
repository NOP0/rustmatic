use rustmatic_core::{Process, ProcessImage, System, Transition};
use rustmatic_runtime::{Fault, Runtime};

    struct PlcMain;

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

            Transition::StillRunning
        }
    }
fn main() {
    let mut runtime = Runtime::new();

    runtime.add_process(PlcMain);

    let _ = runtime.poll();
}
