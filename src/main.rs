mod error;
mod utils;

use error::{ParseResult, ParserContext};
use std::{
    fs::File,
    io::{BufReader, ErrorKind},
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

fn parse_target_platform(
    file: &mut BufReader<File>,
    endianess: Endianess,
) -> ParseResult<TargetPlatform> {
    let target_platform = file
        .read_i32(endianess)
        .context("reading target platform")?;

    Ok(target_platform.into())
}

fn parse_type_tree_presence(file: &mut BufReader<File>) -> ParseResult<bool> {
    file.read_bool().context("reading type tree status")
}

fn main() -> ParseResult<()> {
    let file = File::open("/Users/mathspy/Downloads/resources.assets").unwrap();
    let mut file = BufReader::new(file);

    let header = dbg!(parse_header(&mut file)?);
    dbg!(parse_unity_version(&mut file)?);
    dbg!(parse_target_platform(&mut file, header.endianess)?);
    let _has_type_tree = parse_type_tree_presence(&mut file)?;

    Ok(())
}
