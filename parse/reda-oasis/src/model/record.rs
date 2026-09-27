use num_bigint::BigUint;
use num_traits::Zero;

use super::{AString, NString, OString, Real, Repetition};

#[derive(Debug, Clone)]
pub struct PadRecord;

/*
START Record ('1')
├── version_string : AString("1.0")
├── unit : Real
├── offset_flag : StartOffsetFlag
└── table_offsets : Option<TableOffsets>
*/
#[derive(Debug, Clone)]
pub struct StartRecord {
    pub version_string: AString, 
    pub unit: Real,
    pub offset_flag: StartOffsetFlag,
    pub table_offsets: Option<TableOffsets>,
}

/// '2' [ table-offsets ] padding-string validation-scheme [ validation-signature ]
#[derive(Debug, Clone)]
pub struct EndRecord {
    pub table_offsets: Option<TableOffsets>, // Only present if Start.offset_flag == InEnd
    pub padding_length: usize,               // Derived: number of bytes used for padding
    pub validation_scheme: ValidationScheme,
    pub validation_signature: Option<[u8; 4]>, // CRC32 or CHECKSUM32 result
}

#[derive(Debug, Clone)]
pub enum CellNameRecord {
    /// Implicit numbering, record ID = '3'
    Implicit(NString),
    /// Explicit numbering, record ID = '4'
    Explicit(NString, BigUint),
}

#[derive(Debug, Clone)]
pub enum TextStringRecord {
    /// Implicit numbering, record ID = '5'
    Implicit(AString),
    /// Explicit numbering, record ID = '6'
    Explicit(AString, BigUint),
}

#[derive(Debug, Clone)]
pub enum PropNameRecord {
    /// Implicit numbering, record ID = '7'
    Implicit(NString),
    /// Explicit numbering, record ID = '8'
    Explicit(NString, BigUint),
}

#[derive(Debug, Clone)]
pub enum PropStringRecord {
    /// Implicit numbering, record ID = '9'
    Implicit(OString),
    /// Explicit numbering, record ID = '10'
    Explicit(OString, BigUint),
}

#[derive(Debug, Clone)]
pub enum LayerNameRecord {
    Layer {
        name: NString,
        layer_interval: Interval,
        datatype_interval: Interval,
    },
    TextLayer {
        name: NString,
        textlayer_interval: Interval,
        texttype_interval: Interval,
    },
}

#[derive(Debug, Clone)]
pub enum CellRecord {
    Number(BigUint),
    Name(NString),
}

#[derive(Debug, Clone)]
pub struct XyAbsoluteRecord;

#[derive(Debug, Clone)]
pub struct XyRelativeRecord;

/// PLACEMENT record
///
/// Record type 17 (Discrete) or 18 (Continuous)
/// 描述当前 cell 中的子单元放置（cell instance）。
#[derive(Debug, Clone)]
pub enum PlacementRecord {
    /// Record type '17' — Discrete rotation, magnification = 1.0
    Discrete(Placement17),
    /// Record type '18' — Continuous rotation and magnification (reals)
    Continuous(Placement18),
}

#[derive(Debug, Clone)]
pub struct Placement17 {
    /// placement-info-byte: bit pattern CNXYRAAF
    pub info: PlacementInfo17,

    /// 引用的子单元名或编号
    pub reference: Option<CellReference>,

    /// 放置坐标
    pub x: Option<i64>,
    pub y: Option<i64>,

    /// 重复模式（若 R=1）
    pub repetition: Option<Repetition>,

    /// 是否镜像（F=1 表示沿 X 轴翻转）
    pub flip_x: bool,

    /// 离散旋转角度（AA 编码：0, 90, 180, 270）
    pub rotation: Rotation90,
}

#[derive(Debug, Clone)]
pub struct Placement18 {
    /// placement-info-byte: bit pattern CNXYRMAF
    pub info: PlacementInfo18,

    /// 引用的子单元名或编号
    pub reference: Option<CellReference>,

    /// 可选放大倍数（若 M=1）
    pub magnification: Option<f64>,

    /// 可选旋转角度（若 A=1）
    pub angle: Option<f64>,

    /// 放置坐标
    pub x: Option<i64>,
    pub y: Option<i64>,

    /// 重复模式（若 R=1）
    pub repetition: Option<Repetition>,

    /// 是否镜像（F=1 表示沿 X 轴翻转）
    pub flip_x: bool,
}

#[derive(Debug, Clone)]
pub enum CellReference {
    /// Reference by explicit numeric ID
    Number(BigUint),

    /// Reference by local cell name
    Name(NString),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rotation90 {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

#[derive(Debug, Clone, Copy)]
pub struct PlacementInfo17 {
    pub c: bool, // 是否显式指定 cell
    pub n: bool, // 使用 reference-number (若 c=1)
    pub x: bool, // 是否包含 x
    pub y: bool, // 是否包含 y
    pub r: bool, // 是否包含 repetition
    pub a: u8,   // 旋转角度编码（AA 两位）
    pub f: bool, // 是否镜像
}

#[derive(Debug, Clone, Copy)]
pub struct PlacementInfo18 {
    pub c: bool,
    pub n: bool,
    pub x: bool,
    pub y: bool,
    pub r: bool,
    pub m: bool, // magnification present
    pub a: bool, // angle present
    pub f: bool, // flip
}

/// 表示区间类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interval {
    Unbounded,              // 0: 0..∞
    Upper(u64),             // 1: 0..bound
    Lower(u64),             // 2: bound..∞
    Exact(u64),             // 3: bound..bound
    Range(u64, u64),        // 4: bound-a..bound-b
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationScheme {
    None,
    Crc32,
    Checksum32,
}

#[derive(Debug, Clone)]
pub struct TableOffsets {
    pub cell_name: TableOffsetEntry,
    pub text_string: TableOffsetEntry,
    pub prop_name: TableOffsetEntry,
    pub prop_string: TableOffsetEntry,
    pub layer_name: TableOffsetEntry,
    pub xname: TableOffsetEntry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartOffsetFlag {
    InStart,
    InEnd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrictMode {
    NonStrict,
    Strict,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableOffsetEntry {
    pub strict: StrictMode,  // flag (0 or 1)
    pub offset: BigUint,     // byte offset from file start
}

impl TableOffsetEntry {
    pub fn is_present(&self) -> bool {
        self.offset != BigUint::zero()
    }
}
