use crate::{InputNumber, OutputNumber};
use slotmap::{DenseSlotMap};
use anymap::AnyMap;

#[derive(Clone)]
pub struct InputChannels<T>(DenseSlotMap<InputNumber, T>);

impl<T> Default for InputChannels<T> {
    fn default() -> InputChannels<T> { InputChannels(DenseSlotMap::with_key()) }
}
// TODO: Implement for OutputChannels
pub struct OutputChannels<T>(DenseSlotMap<OutputNumber, T>);

pub struct ProcessImage {
    input_channels : AnyMap,
    output_channels : AnyMap,
}

impl ProcessImage {
    pub fn new() -> Self {
        ProcessImage{
        input_channels : AnyMap::new(),
        output_channels : AnyMap::new(),
        }
    }

    pub fn register_input<T: 'static>(
        &mut self,
        input: T,
    ) -> InputNumber {
        let channels = self
            .input_channels
            .entry::<InputChannels<T>>()
            .or_insert_with(InputChannels::<T>::default);

        let id = channels.0.insert(input);

        id
    }

    pub fn read<T: 'static+Copy>(&self, input: InputNumber) -> T{

        // Get handle to slotmap of correct type
        let input_ref =
        self
        .input_channels.get::<InputChannels<T>>()
        .and_then(|input_channels|input_channels.0.get(input));

        *(input_ref.unwrap())

    
    }

    pub fn write<T: 'static+Copy>(&mut self, input: InputNumber, state: T){
        let input_ref = 
        self
        .input_channels.get_mut::<InputChannels<T>>()
        .and_then(|input_channels|input_channels.0.get_mut(input));
        *(input_ref.unwrap()) = state;
    }
   
}





