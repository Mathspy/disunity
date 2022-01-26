mod error;
mod utils;

use disunity_derive::Variant;
use error::{ParseResult, ParserContext};
use std::{
    fs::File,
    io::{BufReader, ErrorKind, Read},
};
use utils::{BufReadExt, ReadExt};

use crate::error::ParseError;

#[derive(Clone, Copy, Debug)]
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

#[derive(Debug, Variant)]
#[disunity(discriminant = u32)]
enum AssetClass {
    Unknown(u32),
    #[disunity(discriminant = 1)]
    GameObject,
    #[disunity(discriminant = 4)]
    Transform,
    #[disunity(discriminant = 20)]
    Camera,
    #[disunity(discriminant = 21)]
    Material,
    #[disunity(discriminant = 23)]
    MeshRenderer,
    #[disunity(discriminant = 28)]
    Texture2D,
    #[disunity(discriminant = 33)]
    MeshFilter,
    #[disunity(discriminant = 43)]
    Mesh,
    #[disunity(discriminant = 48)]
    Shader,
    #[disunity(discriminant = 49)]
    TextAsset,
    #[disunity(discriminant = 50)]
    RigidBody2D,
    #[disunity(discriminant = 58)]
    CircleCollider2D,
    #[disunity(discriminant = 60)]
    PolygonCollider2D,
    #[disunity(discriminant = 61)]
    BoxCollider2D,
    #[disunity(discriminant = 62)]
    PhysicsMaterial2D,
    #[disunity(discriminant = 65)]
    BoxCollider,
    #[disunity(discriminant = 66)]
    CompositeCollider2D,
    #[disunity(discriminant = 68)]
    EdgeCollider2D,
    #[disunity(discriminant = 70)]
    CapsuleCollider2D,
    #[disunity(discriminant = 72)]
    ComputeShader,
    #[disunity(discriminant = 74)]
    AnimationClip,
    #[disunity(discriminant = 81)]
    AudioListener,
    #[disunity(discriminant = 82)]
    AudioSource,
    #[disunity(discriminant = 91)]
    AnimatorController,
    #[disunity(discriminant = 95)]
    Animator,
    #[disunity(discriminant = 114)]
    MonoBehavior {
        script_id: [u8; 16],
    },
    #[disunity(discriminant = 120)]
    LineRenderer,
    #[disunity(discriminant = 128)]
    Font,
    #[disunity(discriminant = 198)]
    ParticleSystem,
    #[disunity(discriminant = 199)]
    ParticleSystemRenderer,
    #[disunity(discriminant = 210)]
    SortingGroup,
    #[disunity(discriminant = 212)]
    SpriteRenderer,
    #[disunity(discriminant = 213)]
    Sprite,
    #[disunity(discriminant = 221)]
    AnimatorOverrideController,
    #[disunity(discriminant = 222)]
    CanvasRenderer,
    #[disunity(discriminant = 223)]
    Canvas,
    #[disunity(discriminant = 224)]
    RectTransform,
    #[disunity(discriminant = 225)]
    CanvasGroup,
    #[disunity(discriminant = 320)]
    PlayableDirector,
    #[disunity(discriminant = 328)]
    VideoPlayer,
    #[disunity(discriminant = 331)]
    SpriteMask,
    #[disunity(discriminant = 19719996)]
    TilemapCollider2D,
    #[disunity(discriminant = 156049354)]
    Grid,
    #[disunity(discriminant = 483693784)]
    TilemapRenderer,
    #[disunity(discriminant = 687078895)]
    SpriteAtlas,
    #[disunity(discriminant = 1839735485)]
    Tilemap,
}

#[derive(Debug)]
struct AssetType {
    class: AssetClass,
    stripped: bool,
    script_type_index: u16,
    old_type_hash: [u8; 16],
}

fn parse_asset_types(
    file: &mut BufReader<File>,
    endianess: Endianess,
) -> ParseResult<Vec<AssetType>> {
    let count = file
        .read_u32(endianess)
        .context("reading asset types count")?;

    (0..count)
        .map(|_| {
            let class_id = file
                .read_u32(endianess)
                .context("reading asset type class_id")?;
            let class = AssetClassVariant::from_int(class_id);

            let stripped = file.read_bool().context("reading asset type is_stripped")?;
            let script_type_index = file
                .read_u16(endianess)
                .context("reading asset type script type index")?;

            let class = match class {
                Some(AssetClassVariant::MonoBehavior) => {
                    let mut script_id = [0u8; 16];
                    file.read_exact(&mut script_id)
                        .context("reading old type hash")?;
                    AssetClass::MonoBehavior { script_id }
                }
                Some(known_class) => AssetClass::from_variant(known_class)
                    .expect("to have handled all variants with fields"),
                None => dbg!(AssetClass::Unknown(class_id)),
            };

            let mut old_type_hash = [0u8; 16];
            file.read_exact(&mut old_type_hash)
                .context("reading old type hash")?;

            Ok(AssetType {
                class,
                stripped,
                script_type_index,
                old_type_hash,
            })
        })
        .collect()
}

fn main() -> ParseResult<()> {
    let file = File::open("/Users/mathspy/Downloads/resources.assets").unwrap();
    let mut file = BufReader::new(file);

    let header = dbg!(parse_header(&mut file)?);
    dbg!(parse_unity_version(&mut file)?);
    dbg!(parse_target_platform(&mut file, header.endianess)?);
    let _has_type_tree = parse_type_tree_presence(&mut file)?;
    let _asset_types = parse_asset_types(&mut file, header.endianess)?;

    Ok(())
}
