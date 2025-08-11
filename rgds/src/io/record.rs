#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GdsRecordType {
    Header = 0x0002,
    BgnLib = 0x0102,
    LibName = 0x0206,
    RefLibs = 0x1F06,
    Fonts = 0x2006,
    AttrTable = 0x2306,
    Generations = 0x2202,
    Format = 0x3602,
    Mask = 0x3706,
    EndMasks = 0x3800,
    Uints = 0x0305,

    EndLib = 0x0400,

    BgnStr = 0x0502,
    StrName = 0x0606,

    EndStr = 0x0700,
    EndEle = 0x1100,

    Boundary = 0x0800,
    Path = 0x0900,
    SRef = 0x0A00,
    ARef = 0x0B00,
    Text = 0x0C00,
    Node = 0x1500,
    Box = 0x2D00,

    ElFlags = 0x2601,
    Plex = 0x2F03,
    Layer = 0x0D02,
    DataType = 0x0E02,
    Xy = 0x1003,
    PathType = 0x2102,
    Width = 0x0F03,
    SName = 0x1206,
    STrans = 0x1A01,
    Mag = 0x1B05,
    Angle = 0x1C05,
    ColRow = 0x1302,
    TextType = 0x1602,
    Presentation = 0x1701,
    String = 0x1906,
    NodeType = 0x2A02,
    BoxType = 0x2E02,

    BgnExtn = 0x3003,
    EndExtn = 0x3103,
}

impl GdsRecordType {
    pub fn from_u16(value: u16) -> Option<Self> {
        use GdsRecordType::*;
        match value {
            0x0002 => Some(Header),
            0x0102 => Some(BgnLib),
            0x0206 => Some(LibName),
            0x1F06 => Some(RefLibs),
            0x2006 => Some(Fonts),
            0x2306 => Some(AttrTable),
            0x2202 => Some(Generations),
            0x3602 => Some(Format),
            0x3706 => Some(Mask),
            0x3800 => Some(EndMasks),
            0x0305 => Some(Uints),
            0x0400 => Some(EndLib),
            0x0502 => Some(BgnStr),
            0x0606 => Some(StrName),
            0x0700 => Some(EndStr),
            0x1100 => Some(EndEle),
            0x0800 => Some(Boundary),
            0x0900 => Some(Path),
            0x0A00 => Some(SRef),
            0x0B00 => Some(ARef),
            0x0C00 => Some(Text),
            0x1500 => Some(Node),
            0x2D00 => Some(Box),
            0x2601 => Some(ElFlags),
            0x2F03 => Some(Plex),
            0x0D02 => Some(Layer),
            0x0E02 => Some(DataType),
            0x1003 => Some(Xy),
            0x2102 => Some(PathType),
            0x0F03 => Some(Width),
            0x1206 => Some(SName),
            0x1A01 => Some(STrans),
            0x1B05 => Some(Mag),
            0x1C05 => Some(Angle),
            0x1302 => Some(ColRow),
            0x1602 => Some(TextType),
            0x1701 => Some(Presentation),
            0x1906 => Some(String),
            0x2A02 => Some(NodeType),
            0x2E02 => Some(BoxType),
            0x3003 => Some(BgnExtn),
            0x3103 => Some(EndExtn),
            _ => None,
        }
    }
}

impl From<GdsRecordType> for u16 {
    fn from(value: GdsRecordType) -> Self {
        value as u16
    }
}

impl std::fmt::Display for GdsRecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use GdsRecordType::*;
        let name = match self {
            Header => "Header",
            BgnLib => "BgnLib",
            LibName => "LibName",
            RefLibs => "RefLibs",
            Fonts => "Fonts",
            AttrTable => "AttrTable",
            Generations => "Generations",
            Format => "Format",
            Mask => "Mask",
            EndMasks => "EndMasks",
            Uints => "Uints",

            EndLib => "EndLib",

            BgnStr => "BgnStr",
            StrName => "StrName",

            EndStr => "EndStr",
            EndEle => "EndEle",

            Boundary => "Boundary",
            Path => "Path",
            SRef => "SRef",
            ARef => "ARef",
            Text => "Text",
            Node => "Node",
            Box => "Box",

            ElFlags => "ElFlags",
            Plex => "Plex",
            Layer => "Layer",
            DataType => "DataType",
            Xy => "Xy",
            PathType => "PathType",
            Width => "Width",
            SName => "SName",
            STrans => "STrans",
            Mag => "Mag",
            Angle => "Angle",
            ColRow => "ColRow",
            TextType => "TextType",
            Presentation => "Presentation",
            String => "String",
            NodeType => "NodeType",
            BoxType => "BoxType",

            BgnExtn => "BgnExtn",
            EndExtn => "EndExtn",
        };
        write!(f, "{}", name)
    }
}