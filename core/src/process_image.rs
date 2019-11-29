use crate::{InputNumber, OutputNumber};
use slotmap::{DenseSlotMap};
use anymap::AnyMap;
use std::cell::RefCell;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct InputChannels<T>(DenseSlotMap<InputNumber, T>);

impl<T> Default for InputChannels<T> {
    fn default() -> InputChannels<T> { InputChannels(DenseSlotMap::with_key()) }
}
// TODO: Implement for OutputChannels
pub struct OutputChannels<T>(DenseSlotMap<OutputNumber, T>);

pub struct ProcessImage {
    input_channels : RefCell<AnyMap>,
    output_channels : AnyMap,
}



#[derive(Copy, Clone)]
pub struct InputHandle<T>{
    input_number : InputNumber,
    of_type : PhantomData<T>,
}


impl ProcessImage {
    pub fn new() -> Self {
        ProcessImage{
        input_channels : RefCell::new(AnyMap::new()),
        output_channels : AnyMap::new(),
        }
    }

    pub fn register_input<T: 'static>(
        &self,
        input: T,
    ) -> InputHandle<T> {
        let mut channels = self
            .input_channels
            .borrow_mut();

        let id=
            channels
            .entry::<InputChannels<T>>()
            .or_insert_with(InputChannels::<T>::default)
            .0.insert(input);

        InputHandle{
            input_number: id,
            of_type: PhantomData,
        }
    }

    pub fn read<T: 'static+Copy>(&self, input: InputHandle<T>) -> T{

        // Get handle to slotmap of correct type
        let channels = self
            .input_channels
            .borrow_mut();

        let value =
        channels
        .get::<InputChannels<T>>()
        .and_then(|input_channels|input_channels.0.get(input.input_number));

        *value.unwrap()

    
    }

    pub fn write<T: 'static+Copy>(& self, input: InputHandle<T>, state: T){
        let mut channels = self
        .input_channels
        .borrow_mut();

        let value =
        channels
        .get_mut::<InputChannels<T>>()
        .and_then(|input_channels|input_channels.0.get_mut(input.input_number));

        *value.unwrap() = state;
    }
   
}





