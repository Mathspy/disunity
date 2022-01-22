mod error;
mod utils;

use error::{ParseResult, ParserContext};
use std::{
    fs::File,
    io::{BufRead, BufReader, ErrorKind},
};
use utils::{BufReadExt, ReadExt};

use crate::error::ParseError;

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

fn parse_header(file: &mut BufReader<File>) -> ParseResult<Header> {
    // Ignore first 8 bytes
    file.seek_relative(8).context("ignoring first 8 bytes")?;

    let version = file
        .read_u32(Endianess::Big)
        .context("reading header version")?;

    // Ignore 4 bytes
    file.seek_relative(4)
        .context("ignoring 4 bytes after header")?;

    let endianess = file.read_bool().context("reading endianess boolean")?;
    let endianess = if endianess {
        Endianess::Big
    } else {
        Endianess::Little
    };

    // Throw away "reserved" for now
    file.seek_relative(3).context("ignoring reserved bytes")?;

    let metadata = file
        .read_u32(Endianess::Big)
        .context("reading header metadata")?;
    let file_size = file
        .read_u64(Endianess::Big)
        .context("reading header file size")?;
    let data_offset = file
        .read_u64(Endianess::Big)
        .context("reading header data offset")?;

    // Ignore 8 unknown bytes
    file.seek_relative(8)
        .context("ignoring last 8 bytes of header")?;

    Ok(Header {
        version,
        endianess,
        metadata,
        file_size,
        data_offset,
    })
}

fn parse_unity_version(file: &mut BufReader<File>) -> ParseResult<String> {
    file.read_null_terminated_string()
        .map_err(|(error, bytes)| match error.kind() {
            ErrorKind::UnexpectedEof => {
                ParseError::expected("Unity version ending with a null byte", bytes, error)
            }
            ErrorKind::InvalidData => {
                ParseError::expected("valid utf-8 for Unity version", bytes, error)
            }
            _ => ParseError::unexpected("parsing Unity version string", error),
        })
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

fn get_target_platform<R: BufRead>(
    file: &mut R,
    endianess: Endianess,
) -> ParseResult<TargetPlatform> {
    let mut buffer = [0u8; 4];

    file.read_exact(&mut buffer)
        .context("reading target platform")?;
    let target_platform = match endianess {
        Endianess::Big => i32::from_be_bytes(buffer),
        Endianess::Little => i32::from_le_bytes(buffer),
    };

    Ok(target_platform.into())
}

fn get_boolean<R: BufRead>(file: &mut R) -> ParseResult<bool> {
    let mut buffer = [0u8; 0];
    file.read_exact(&mut buffer)
        .context("reading type tree status")?;
    Ok(buffer[0] == 0)
}

fn main() -> ParseResult<()> {
    let file = File::open("/Users/mathspy/Downloads/resources.assets").unwrap();
    let mut file = BufReader::new(file);

    let header = dbg!(parse_header(&mut file)?);
    dbg!(parse_unity_version(&mut file)?);
    dbg!(get_target_platform(&mut file, header.endianess)?);
    let _type_tree_enabled = get_boolean(&mut file)?;

    Ok(())
}
