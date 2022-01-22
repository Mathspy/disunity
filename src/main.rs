mod utils;

use anyhow::{format_err, Context, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use utils::ReadExt;

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

fn parse_header(file: &mut BufReader<File>) -> Result<Header> {
    // Ignore first 8 bytes
    file.seek_relative(8)?;

    let version = file.read_u32(Endianess::Big)?;

    // Ignore 4 bytes
    file.seek_relative(4)?;

    let endianess = file.read_bool()?;
    let endianess = if endianess {
        Endianess::Big
    } else {
        Endianess::Little
    };

    // Throw away "reserved" for now
    file.seek_relative(3)?;

    let metadata = file.read_u32(Endianess::Big)?;
    let file_size = file.read_u64(Endianess::Big)?;
    let data_offset = file.read_u64(Endianess::Big)?;

    // Ignore 8 unknown bytes
    file.seek_relative(8)?;

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

fn get_boolean<R: BufRead>(file: &mut R) -> Result<bool> {
    let mut buffer = [0u8; 0];
    file.read_exact(&mut buffer)?;
    Ok(buffer[0] == 0)
}

fn main() -> Result<()> {
    let file = File::open("/Users/mathspy/Downloads/resources.assets")?;
    let mut file = BufReader::new(file);

    let header = dbg!(parse_header(&mut file)?);
    dbg!(get_unity_version(&mut file)?);
    dbg!(get_target_platform(&mut file, header.endianess)?);
    let _type_tree_enabled = get_boolean(&mut file)?;

    Ok(())
}
