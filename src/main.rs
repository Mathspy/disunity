mod error;
mod utils;

use error::{ParseResult, ParserContext};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
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

#[derive(Debug, FromPrimitive)]
enum KnownAssetClass {
    GameObject = 1,
    Transform = 4,
    Camera = 20,
    Material = 21,
    MeshRenderer = 23,
    Texture2D = 28,
    MeshFilter = 33,
    Mesh = 43,
    Shader = 48,
    TextAsset = 49,
    RigidBody2D = 50,
    CircleCollider2D = 58,
    PolygonCollider2D = 60,
    BoxCollider2D = 61,
    PhysicsMaterial2D = 62,
    BoxCollider = 65,
    CompositeCollider2D = 66,
    EdgeCollider2D = 68,
    CapsuleCollider2D = 70,
    ComputeShader = 72,
    AnimationClip = 74,
    AudioListener = 81,
    AudioSource = 82,
    AnimatorController = 91,
    Animator = 95,
    MonoBehavior = 114,
    LineRenderer = 120,
    Font = 128,
    ParticleSystem = 198,
    ParticleSystemRenderer = 199,
    SortingGroup = 210,
    SpriteRenderer = 212,
    Sprite = 213,
    AnimatorOverrideController = 221,
    CanvasRenderer = 222,
    Canvas = 223,
    RectTransform = 224,
    CanvasGroup = 225,
    PlayableDirector = 320,
    VideoPlayer = 328,
    SpriteMask = 331,
    TilemapCollider2D = 19719996,
    Grid = 156049354,
    TilemapRenderer = 483693784,
    SpriteAtlas = 687078895,
    Tilemap = 1839735485,
}

#[derive(Debug)]
enum AssetClass {
    Unknown(u32),
    GameObject,
    Transform,
    Camera,
    Material,
    MeshRenderer,
    Texture2D,
    MeshFilter,
    Mesh,
    Shader,
    TextAsset,
    RigidBody2D,
    CircleCollider2D,
    PolygonCollider2D,
    BoxCollider2D,
    PhysicsMaterial2D,
    BoxCollider,
    CompositeCollider2D,
    EdgeCollider2D,
    CapsuleCollider2D,
    ComputeShader,
    AnimationClip,
    AudioListener,
    AudioSource,
    AnimatorController,
    Animator,
    LineRenderer,
    Font,
    ParticleSystem,
    ParticleSystemRenderer,
    SortingGroup,
    SpriteRenderer,
    Sprite,
    AnimatorOverrideController,
    CanvasRenderer,
    Canvas,
    RectTransform,
    CanvasGroup,
    PlayableDirector,
    VideoPlayer,
    SpriteMask,
    TilemapCollider2D,
    Grid,
    TilemapRenderer,
    SpriteAtlas,
    Tilemap,
    MonoBehavior { script_id: [u8; 16] },
}

impl From<KnownAssetClass> for AssetClass {
    fn from(value: KnownAssetClass) -> Self {
        match value {
            KnownAssetClass::GameObject => AssetClass::GameObject,
            KnownAssetClass::Transform => AssetClass::Transform,
            KnownAssetClass::Camera => AssetClass::Camera,
            KnownAssetClass::Material => AssetClass::Material,
            KnownAssetClass::MeshRenderer => AssetClass::MeshRenderer,
            KnownAssetClass::Texture2D => AssetClass::Texture2D,
            KnownAssetClass::MeshFilter => AssetClass::MeshFilter,
            KnownAssetClass::Mesh => AssetClass::Mesh,
            KnownAssetClass::Shader => AssetClass::Shader,
            KnownAssetClass::TextAsset => AssetClass::TextAsset,
            KnownAssetClass::RigidBody2D => AssetClass::RigidBody2D,
            KnownAssetClass::CircleCollider2D => AssetClass::CircleCollider2D,
            KnownAssetClass::PolygonCollider2D => AssetClass::PolygonCollider2D,
            KnownAssetClass::BoxCollider2D => AssetClass::BoxCollider2D,
            KnownAssetClass::PhysicsMaterial2D => AssetClass::PhysicsMaterial2D,
            KnownAssetClass::BoxCollider => AssetClass::BoxCollider,
            KnownAssetClass::CompositeCollider2D => AssetClass::CompositeCollider2D,
            KnownAssetClass::EdgeCollider2D => AssetClass::EdgeCollider2D,
            KnownAssetClass::CapsuleCollider2D => AssetClass::CapsuleCollider2D,
            KnownAssetClass::ComputeShader => AssetClass::ComputeShader,
            KnownAssetClass::AnimationClip => AssetClass::AnimationClip,
            KnownAssetClass::AudioListener => AssetClass::AudioListener,
            KnownAssetClass::AudioSource => AssetClass::AudioSource,
            KnownAssetClass::AnimatorController => AssetClass::AnimatorController,
            KnownAssetClass::Animator => AssetClass::Animator,
            KnownAssetClass::LineRenderer => AssetClass::LineRenderer,
            KnownAssetClass::Font => AssetClass::Font,
            KnownAssetClass::ParticleSystem => AssetClass::ParticleSystem,
            KnownAssetClass::ParticleSystemRenderer => AssetClass::ParticleSystemRenderer,
            KnownAssetClass::SortingGroup => AssetClass::SortingGroup,
            KnownAssetClass::SpriteRenderer => AssetClass::SpriteRenderer,
            KnownAssetClass::Sprite => AssetClass::Sprite,
            KnownAssetClass::AnimatorOverrideController => AssetClass::AnimatorOverrideController,
            KnownAssetClass::CanvasRenderer => AssetClass::CanvasRenderer,
            KnownAssetClass::Canvas => AssetClass::Canvas,
            KnownAssetClass::RectTransform => AssetClass::RectTransform,
            KnownAssetClass::CanvasGroup => AssetClass::CanvasGroup,
            KnownAssetClass::PlayableDirector => AssetClass::PlayableDirector,
            KnownAssetClass::VideoPlayer => AssetClass::VideoPlayer,
            KnownAssetClass::SpriteMask => AssetClass::SpriteMask,
            KnownAssetClass::TilemapCollider2D => AssetClass::TilemapCollider2D,
            KnownAssetClass::Grid => AssetClass::Grid,
            KnownAssetClass::TilemapRenderer => AssetClass::TilemapRenderer,
            KnownAssetClass::SpriteAtlas => AssetClass::SpriteAtlas,
            KnownAssetClass::Tilemap => AssetClass::Tilemap,
            KnownAssetClass::MonoBehavior => {
                panic!("MonoBehavior needs to be handled separately from other asset classes")
            }
        }
    }
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
            let class = KnownAssetClass::from_u32(class_id);

            let stripped = file.read_bool().context("reading asset type is_stripped")?;
            let script_type_index = file
                .read_u16(endianess)
                .context("reading asset type script type index")?;

            let class = match class {
                Some(KnownAssetClass::MonoBehavior) => {
                    let mut script_id = [0u8; 16];
                    file.read_exact(&mut script_id)
                        .context("reading old type hash")?;
                    AssetClass::MonoBehavior { script_id }
                }
                Some(known_class) => AssetClass::from(known_class),
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
