use crate::Endianess;
use std::io::{Read, Result};

pub(crate) trait ReadExt: Read {
    fn read_u8(&mut self) -> Result<u8> {
        let mut buffer = [0u8; 1];
        self.read_exact(&mut buffer)?;
        Ok(buffer[0])
    }

    fn read_bool(&mut self) -> Result<bool> {
        let byte = self.read_u8()?;
        Ok(match byte {
            0 => false,
            1 => true,
            x => panic!("Expected bool to be 0 or 1 but received {}", x),
        })
    }

    fn read_u32(&mut self, endianess: Endianess) -> Result<u32> {
        let mut buffer = [0u8; 4];
        self.read_exact(&mut buffer)?;
        Ok(match endianess {
            Endianess::Big => u32::from_be_bytes(buffer),
            Endianess::Little => u32::from_le_bytes(buffer),
        })
    }

    fn read_i32(&mut self, endianess: Endianess) -> Result<i32> {
        let mut buffer = [0u8; 4];
        self.read_exact(&mut buffer)?;
        Ok(match endianess {
            Endianess::Big => i32::from_be_bytes(buffer),
            Endianess::Little => i32::from_le_bytes(buffer),
        })
    }

    fn read_u64(&mut self, endianess: Endianess) -> Result<u64> {
        let mut buffer = [0u8; 8];
        self.read_exact(&mut buffer)?;
        Ok(match endianess {
            Endianess::Big => u64::from_be_bytes(buffer),
            Endianess::Little => u64::from_le_bytes(buffer),
        })
    }
}

impl<T> ReadExt for T where T: Read {}
