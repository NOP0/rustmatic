//! Abstractions and data types for hardware configuration

use rustmatic_core::{
    Device, InputNumber, OutputNumber, Process, System, Transition, Value,
    VariableIndex,};


struct StandardDigitalInput{
    state : bool
}

impl InputChannel for StandardDigitalInput{
    fn update(&self) -> Option<bool>{
        self.state
    }
}


