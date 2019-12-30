use crate::{DeviceID, DeviceManager};
use byteorder::{ByteOrder, LittleEndian};

const PI_LENGTH: usize = 128;

#[derive(Clone, Copy)]
pub struct Address {
    pub byte: usize,
    pub bit: usize,
    pub type_of: AccessType,
}

#[derive(Clone, Copy, Debug)]
pub enum AccessType {
    Bit,
    Byte,
    Word,
    DoubleWord,
}

pub enum Direction {
    In,
    Out,
}

pub struct ProcessImage {
    pub image: Vec<u8>,
    pub devices: Vec<(DeviceID, Address)>,
    direction: Direction,
}

impl ProcessImage {
    pub fn new(direction: Direction) -> Self {
        ProcessImage {
            image: vec![0; PI_LENGTH],
            devices: Vec::new(),
            direction,
        }
    }

    pub fn with_capacity(capacity: usize, direction: Direction) -> Self {
        ProcessImage {
            image: vec![0; capacity],
            devices: Vec::new(),
            direction,
        }
    }

    pub fn read_bit(&self, byte: usize, bit: usize) -> bool {
        let byte_prev = self.image[byte];
        let mask = 1 << bit;
        (byte_prev & mask) != 0
    }

    pub fn read_byte(&self, byte: usize) -> u8 { self.image[byte] }

    pub fn read_word(&self, word: usize) -> u16 {
        LittleEndian::read_u16(&self.image[word..word + 2])
    }

    pub fn read_double_word(&self, dword: usize) -> u32 {
        LittleEndian::read_u32(&self.image[dword..dword + 4])
    }

    pub fn write_bit(&mut self, byte: usize, bit: usize, state: bool) {
        if state {
            self.image[byte] |= 1 << bit;
        } else {
            self.image[byte] &= !(1 << bit);
        }
    }

    pub fn write_byte(&mut self, byte: usize, state: u8) {
        self.image[byte] = state;
    }

    pub fn write_word(&mut self, word: usize, state: u16) {
        LittleEndian::write_u16(&mut self.image[word..word + 2], state);
    }

    pub fn write_double_word(&mut self, dword: usize, state: u32) {
        LittleEndian::write_u32(&mut self.image[dword..dword + 4], state);
    }

    pub fn register_device(
        &mut self,
        byte: usize,
        bit: usize,
        type_of: AccessType,
        device: DeviceID,
    ) {
        self.devices.push((device, Address { byte, bit, type_of }));
    }

    pub fn update(&mut self, devices: &DeviceManager) {
        let devices_vec: Vec<(DeviceID, Address)> = self.devices.clone();

        match self.direction {
            Direction::In => {
                for device in devices_vec {
                    match device.1.type_of {
                        AccessType::Bit => self.write_bit(
                            device.1.byte,
                            device.1.bit,
                            devices.read::<bool>(device.0).unwrap(),
                        ),
                        AccessType::Byte => self.write_byte(
                            device.1.byte,
                            devices.read::<u8>(device.0).unwrap(),
                        ),
                        AccessType::Word => self.write_word(
                            device.1.byte,
                            devices.read::<u16>(device.0).unwrap(),
                        ),
                        AccessType::DoubleWord => self.write_double_word(
                            device.1.byte,
                            devices.read::<u32>(device.0).unwrap(),
                        ),
                    }
                }
            },
            Direction::Out => {
                for device in devices_vec {
                    match device.1.type_of {
                        AccessType::Bit => {
                            (devices
                                .write::<bool>(
                                    device.0,
                                    self.read_bit(device.1.byte, device.1.bit),
                                )
                                .unwrap())
                        },
                        AccessType::Byte => {
                            (devices
                                .write::<u8>(
                                    device.0,
                                    self.read_byte(device.1.byte),
                                )
                                .unwrap())
                        },
                        AccessType::Word => {
                            (devices
                                .write::<u16>(
                                    device.0,
                                    self.read_word(device.1.byte),
                                )
                                .unwrap())
                        },
                        AccessType::DoubleWord => {
                            (devices
                                .write::<u32>(
                                    device.0,
                                    self.read_double_word(device.1.byte),
                                )
                                .unwrap())
                        },
                    }
                }
            },
        }
    }
}
