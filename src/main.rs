use anyhow::{format_err, Context, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Header {
    version: u32,
    endianess: u8,
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
    let endianess = buffer[0];

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

fn main() -> Result<()> {
    let file = File::open("/Users/mathspy/Downloads/resources.assets")?;
    let mut file = BufReader::new(file);

    dbg!(get_header(&mut file)?);
    dbg!(get_unity_version(&mut file)?);

    Ok(())
}
