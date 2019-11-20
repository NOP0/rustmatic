use rustmatic_runtime::Runtime;
use rustmatic_core::{Device, InputChannel};


fn main() {

    #[derive(Copy, Clone)]
    struct StandardDigitalInput{
        state: bool,
    }

    impl StandardDigitalInput{
        pub fn new() -> Self{
            StandardDigitalInput{
                state: false,
            }
        }
    }
    struct Mydevice{
        channels: Vec<StandardDigitalInput>,
    }

    impl Mydevice{
        pub fn new() -> Self{
            Mydevice{
            channels: vec![StandardDigitalInput::new();8]
            }
                    
    }

  
}

let my_device = Mydevice::new();



}