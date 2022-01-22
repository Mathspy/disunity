use anyhow::{format_err, Context, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Endianess {
    Big,
    Little,
}

#[derive(Debug)]
struct Header {
    version: u32,
    endianess: Endianess,
    // reserved??
    metadata: u32,
    file_size: u64,
    data_offset: u64,
}

fn get_header<R: BufRead>(file: &mut R) -> Result<Header> {
    let mut buffer = [0u8; 8];

    // Ignore first 8 bytes
    file.read_exact(&mut buffer)?;

    file.read_exact(&mut buffer[0..4])?;
    let version = u32::from_be_bytes(buffer[0..4].try_into()?);

    // Ignore 4 bytes
    file.read_exact(&mut buffer[0..4])?;

    file.read_exact(&mut buffer[0..1])?;
    let endianess = if buffer[0] == 0 {
        Endianess::Little
    } else {
        Endianess::Big
    };

    // Throw away "reserved" for now
    file.read_exact(&mut buffer[0..3])?;

    file.read_exact(&mut buffer[0..4])?;
    let metadata = u32::from_be_bytes(buffer[0..4].try_into()?);

    file.read_exact(&mut buffer)?;
    let file_size = u64::from_be_bytes(buffer);

    file.read_exact(&mut buffer)?;
    let data_offset = u64::from_be_bytes(buffer);

    // Ignore 8 unknown bytes
    file.read_exact(&mut buffer)?;

    Ok(Header {
        version,
        endianess,
        metadata,
        file_size,
        data_offset,
    })
}

fn get_unity_version<R: BufRead>(file: &mut R) -> Result<String> {
    let mut buffer = Vec::new();

    if file.read_until(0, &mut buffer)? == 0 {
        return Err(format_err!(
            "Expected Unity version ending with a null byte"
        ));
    }

    // Drop null byte
    buffer.pop();

    String::from_utf8(buffer).context("Failed to parse unity version as valid utf-8")
}

#[derive(Debug)]
enum TargetPlatform {
    Unknown(i32),
    Windows64,
}

impl From<i32> for TargetPlatform {
    fn from(value: i32) -> Self {
        match value {
            19 => TargetPlatform::Windows64,
            value => TargetPlatform::Unknown(value),
        }
    }
}

fn get_target_platform<R: BufRead>(file: &mut R, endianess: Endianess) -> Result<TargetPlatform> {
    let mut buffer = [0u8; 4];

    file.read_exact(&mut buffer)?;
    let target_platform = match endianess {
        Endianess::Big => i32::from_be_bytes(buffer),
        Endianess::Little => i32::from_le_bytes(buffer),
    };

    Ok(target_platform.into())
}

fn main() -> Result<()> {
    let file = File::open("/Users/mathspy/Downloads/resources.assets")?;
    let mut file = BufReader::new(file);

    let header = dbg!(get_header(&mut file)?);
    dbg!(get_unity_version(&mut file)?);
    dbg!(get_target_platform(&mut file, header.endianess)?);

    Ok(())
}
