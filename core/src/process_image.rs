use crate::{DeviceID, DeviceManager, AdressNumber};
use byteorder::{ByteOrder, LittleEndian};
use slotmap::DenseSlotMap;
use std::marker::PhantomData;
use std::collections::HashMap;

const PI_LENGTH: usize = 128;

trait Addr<T:PiAccess> {
    type Adress;
    fn register (config: &mut Config<T>, device: DeviceID, adress: Self::Adress);
    fn get_adress(config: &Config<T>, device: DeviceID) -> Self::Adress;
    fn get_device(config: &Config<T>, adress: Self::Adress) -> DeviceID;
    
}

impl Addr<bool> for bool {
    type Adress = (usize, usize);
    fn register (config: &mut Config<bool>, device: DeviceID, adress: Self::Adress){
        config.device_to_adress.insert(device, adress).expect("Could not register device");
    }

    fn get_adress(config: &Config<bool>, device: DeviceID) -> Self::Adress {
        *config.device_to_adress.get(&device).expect("Could not get adress")
    }

    fn get_device(config: &Config<bool>, adress: Self::Adress) -> DeviceID {
        *config.adress_to_device.get(&adress).expect("Could not get device")


    }


}

struct Config<T: PiAccess> {
    device_to_adress: HashMap<DeviceID, T::Adress>,
    adress_to_device: HashMap<T::Adress, DeviceID>,
    phantom: PhantomData<T>,
}


pub trait PiAccess {
    type Op;
    type Adress;
    fn read(pi:&ProcessImage, adress: Self::Adress) -> Self::Op;
    fn write(pi: &mut ProcessImage, adress: Self::Adress, state: Self::Op);
    fn store_adress(adress: Self::Adress) -> Adress;
}
#[derive(Clone, Debug)]
pub enum Adress {
    Bool((usize, usize)),
    U8(usize),
    U16(usize),
    U32(usize),
}

pub enum Direction{
    In,
    Out,
}

pub struct ProcessImage
{   
    direction : Direction,
    pub image: Vec<u8>,
    pub devices: Vec<(DeviceID, Adress)>,
}

impl PiAccess for bool {
    type Adress = (usize, usize);
    type Op = bool;

    fn read(pi:&ProcessImage, adress: (usize, usize)) -> bool {
        let (byte, bit) = (adress.0, adress.1);
        let byte_prev = pi.image[byte];
        let mask = 1 << bit;
        (byte_prev & mask) != 0
    }

    fn write(pi: &mut ProcessImage, adress: (usize, usize), state: bool){ 
        let (byte, bit) = (adress.0, adress.1);
        if state {
            pi.image[byte] |= 1 << bit;
        } else {
            pi.image[byte] &= !(1 << bit);
        }
    }

    fn store_adress(adress: Self::Adress) -> Adress {
        Adress::Bool(adress)
    }
}

impl PiAccess for u8 {
    type Adress = usize;
    type Op = u8;

    fn read(pi: &ProcessImage, adress: usize) -> u8 
     { pi.image[adress] }

    fn write(pi: &mut ProcessImage, adress: usize, state: u8) 
    {
            pi.image[adress] = state;
    }
    fn store_adress(adress: Self::Adress) -> Adress {
        Adress::U8(adress)
    }
}

impl PiAccess for u16 {
    type Adress = usize;
    type Op = u16;

    fn read(pi: &ProcessImage, adress: usize) -> u16 
           {LittleEndian::read_u16(&pi.image[adress..adress + 2])
    }

      fn write(pi: &mut ProcessImage, adress: usize, state: u16)
      {
        LittleEndian::write_u16(&mut pi.image[adress..adress + 2], state);
    }
    fn store_adress(adress: Self::Adress) -> Adress {
        Adress::U16(adress)
    }

}

impl PiAccess for u32 {
    type Adress = usize;
    type Op = u32;
    
    fn read(pi: &ProcessImage, adress: usize) -> u32
    {
            LittleEndian::read_u32(&pi.image[adress..adress + 4])
    }
    fn write(pi: &mut ProcessImage, adress: usize, state: u32) 
    {
        LittleEndian::write_u32(&mut pi.image[adress..adress + 4], state);
    }
    fn store_adress(adress: Self::Adress) -> Adress {
        Adress::U32(adress)
    }


}

    
impl ProcessImage
    {
    pub fn new(direction: Direction) -> Self {
        ProcessImage {
            direction:direction,
            image: vec![0; PI_LENGTH],
            devices: Vec::new(),
        }
    }

    pub fn with_capacity(direction: Direction, capacity: usize) -> Self {
        ProcessImage {
            direction: direction,
            image: vec![0; capacity],
            devices: Vec::new(),
        }
    }
    pub fn register_device<T:PiAccess>(
        &mut self,
        adress: <T as PiAccess>::Adress,
        device: DeviceID,
    ) {
        let adress = <T as PiAccess>::store_adress(adress);
        self.devices.push((device, adress));
    }

    pub fn update(&mut self, devices: &DeviceManager ){
        let devices_vec: Vec<(DeviceID, Adress)> = self.devices.clone();
        for device in devices_vec {
            let (device_id, adress) = device;
            match self.direction {
                Direction::In => {
                    match adress {
                        Adress::Bool((byte, bit)) => {
                            <bool>::write(self, (byte, bit), devices.read::<bool>(device_id).unwrap());
                        }
                        Adress::U8(offset) => {
                            <u8>::write(self, offset, devices.read::<u8>(device_id).unwrap());
                        }
                        Adress::U16(offset) => {
                            <u16>::write(self, offset, devices.read::<u16>(device_id).unwrap());
                        }
                        Adress::U32(offset) => {
                            <u32>::write(self, offset, devices.read::<u32>(device_id).unwrap());
                            }
                        }
                    }
                Direction::Out => {
                    match adress {
                        Adress::Bool((byte, bit)) => {
                            devices.write::<bool>(device_id, <bool>::read(self, (byte, bit)));
                        }
                        Adress::U8(offset) => {
                            devices.write::<u8>(device_id, <u8>::read(self, offset));
                        }
                        Adress::U16(offset) => {
                            devices.write::<u16>(device_id, <u16>::read(self, offset));
                        }
                        Adress::U32(offset) => {
                            devices.write::<u32>(device_id, <u32>::read(self, offset));
                        }
                    }
                }
            }
        }
    }
}
