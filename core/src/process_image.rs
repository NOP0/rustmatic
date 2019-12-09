use crate::{DeviceID, DeviceManager};
use byteorder::{ByteOrder, LittleEndian};

const PI_LENGTH: usize = 128;

#[derive(Clone, Copy)]
pub struct Address {
    pub byte_offset: usize,
    pub bit_offset: usize,
    pub type_of: AccessType,
}

#[derive(Clone, Copy, Debug)]
pub enum AccessType {
    Bit,
    Byte,
    Word,
    DoubleWord,
}

pub struct ProcessImage {
    pub image: [u8; PI_LENGTH],
    pub devices: Vec<(DeviceID, Address)>,
}

impl ProcessImage {
    pub fn new() -> Self {
        ProcessImage {
            image: [0; PI_LENGTH],
            devices: Vec::new(),
        }
    }

    pub fn read_bit(&self, address: Address) -> bool {

        let byte = self.image[address.byte_offset];
        let mask = 1 << address.bit_offset;
        (byte & mask) != 0
    }

    pub fn read_byte(&self, address: Address) -> u8 {
        self.image[address.byte_offset]
    }

    pub fn read_word(&self, address: Address) -> u16 {
        LittleEndian::read_u16(&self.image[address.byte_offset..address.byte_offset+4])
    }

    pub fn read_double_word(&self, address: Address) -> u32 {
        LittleEndian::read_u32(&self.image[address.byte_offset..address.byte_offset+4])
    }

    pub fn write_bit(&mut self, address: Address, state: bool) {
        if state {
            self.image[address.byte_offset] |= 1 << address.bit_offset;
          } else {
            self.image[address.byte_offset] &= !(1 << address.bit_offset);
          }
    }

    pub fn write_byte(&mut self, address: Address, state: u8) {
        self.image[address.byte_offset] = state;
    }

    pub fn write_word(&mut self, address: Address, state: u16) {
        LittleEndian::write_u16(&mut self.image[address.byte_offset..address.byte_offset+2], state);
    }

    pub fn write_double_word(&mut self, address: Address, state: u32) {
        LittleEndian::write_u32(&mut self.image[address.byte_offset..address.byte_offset+4], state);
    }

    pub fn register_input_device(
        &mut self,
        byte_offset: usize,
        bit_offset: usize,
        type_of: AccessType,
        device: DeviceID,
    ) {
        self.devices.push((
            device,
            Address {
                byte_offset,
                bit_offset,
                type_of,
            },
        ));
    }

    pub fn update_inputs(&mut self, devices: &DeviceManager) {
        let devices_vec: Vec<(DeviceID, Address)> = self.devices.clone();

        for device in devices_vec {
            match device.1.type_of {
                AccessType::Bit => self.write_bit(
                    device.1,
                    devices.read::<bool>(device.0).unwrap(),
                ),
                AccessType::Byte => self.write_byte(
                    device.1,
                    devices.read::<u8>(device.0).unwrap(),
                ),
                AccessType::Word => self.write_word(
                    device.1,
                    devices.read::<u16>(device.0).unwrap(),
                ),
                AccessType::DoubleWord => self.write_double_word(
                    device.1,
                    devices.read::<u32>(device.0).unwrap(),
                ),
                _ => {},
            }
        }
    }
}
