use crate::Endianess;
use std::io::{BufRead, Error, ErrorKind, Read, Result as IoResult};

pub(crate) trait ReadExt: Read {
    fn read_u8(&mut self) -> IoResult<u8> {
        let mut buffer = [0u8; 1];
        self.read_exact(&mut buffer)?;
        Ok(buffer[0])
    }

    fn read_bool(&mut self) -> IoResult<bool> {
        let byte = self.read_u8()?;
        Ok(match byte {
            0 => false,
            1 => true,
            x => panic!("Expected bool to be 0 or 1 but received {}", x),
        })
    }

    fn read_u16(&mut self, endianess: Endianess) -> IoResult<u16> {
        let mut buffer = [0u8; 2];
        self.read_exact(&mut buffer)?;
        Ok(match endianess {
            Endianess::Big => u16::from_be_bytes(buffer),
            Endianess::Little => u16::from_le_bytes(buffer),
        })
    }

    fn read_u32(&mut self, endianess: Endianess) -> IoResult<u32> {
        let mut buffer = [0u8; 4];
        self.read_exact(&mut buffer)?;
        Ok(match endianess {
            Endianess::Big => u32::from_be_bytes(buffer),
            Endianess::Little => u32::from_le_bytes(buffer),
        })
    }

    fn read_i32(&mut self, endianess: Endianess) -> IoResult<i32> {
        let mut buffer = [0u8; 4];
        self.read_exact(&mut buffer)?;
        Ok(match endianess {
            Endianess::Big => i32::from_be_bytes(buffer),
            Endianess::Little => i32::from_le_bytes(buffer),
        })
    }

    fn read_u64(&mut self, endianess: Endianess) -> IoResult<u64> {
        let mut buffer = [0u8; 8];
        self.read_exact(&mut buffer)?;
        Ok(match endianess {
            Endianess::Big => u64::from_be_bytes(buffer),
            Endianess::Little => u64::from_le_bytes(buffer),
        })
    }
}

impl<T> ReadExt for T where T: Read {}

pub(crate) trait BufReadExt: BufRead {
    fn read_null_terminated_string(&mut self) -> Result<String, (Error, Vec<u8>)> {
        let mut buffer = Vec::new();

        let read_count = match self.read_until(0, &mut buffer) {
            Ok(read_count) => read_count,
            Err(error) => return Err((error, buffer)),
        };
        if read_count == 0 {
            return Err((Error::from(ErrorKind::UnexpectedEof), buffer));
        }

        // Drop null byte
        buffer.pop();

        String::from_utf8(buffer).map_err(|error| {
            (
                // TODO: Not a fan of this clone, we have to be doing something whacky
                // to end up needing this clone to carry this error across properly
                Error::new(ErrorKind::InvalidData, error.clone()),
                error.into_bytes(),
            )
        })
    }
}

impl<T> BufReadExt for T where T: BufRead {}
