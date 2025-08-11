use std::fmt;

/// Contains one word (two bytes) of bit flags for text presentation. 
/// - Bits 10 and 11, taken together as a binary number, specify the font 
///   (00 means font 0, 01 rneans font 1, 10 means font 2, and 11 means font 3). 
/// - Bits 12 and 13 specify the vertical justification 
///   (00 means top, 01 means middle, and 10 means bottom). 
/// - Bits 14 and 15 specify the horizontal justification 
///   (00 means left, 01 means center, and 10 means right). Bits 0 through 9 are reserved for future use and must be cleared. If this record is omitted, then top-left justification and font 0 are assumed. The following shows a PRESENTATION record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GdsFontNumber {
    Font0,
    Font1,
    Font2,
    Font3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GdsVJustify {
    Top,
    Middle,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GdsHJustify {
    Left,
    Center,
    Right,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GdsPresentation {
    pub font_number: GdsFontNumber,
    pub v_justify: GdsVJustify,
    pub h_justify: GdsHJustify,
}


impl GdsFontNumber {
    pub fn from_bits(bits: u16) -> Option<Self> {
        match bits {
            0 => Some(Self::Font0),
            1 => Some(Self::Font1),
            2 => Some(Self::Font2),
            3 => Some(Self::Font3),
            _ => None,
        }
    }

    pub fn to_bits(self) -> u16 {
        match self {
            Self::Font0 => 0,
            Self::Font1 => 1,
            Self::Font2 => 2,
            Self::Font3 => 3,
        }
    }
}

impl std::fmt::Display for GdsFontNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Font0 => "Font0",
            Self::Font1 => "Font1",
            Self::Font2 => "Font2",
            Self::Font3 => "Font3",
        };
        write!(f, "{}", s)
    }
}

impl GdsVJustify {
    pub fn from_bits(bits: u16) -> Option<Self> {
        match bits {
            0 => Some(Self::Top),
            1 => Some(Self::Middle),
            2 => Some(Self::Bottom),
            _ => None,
        }
    }

    pub fn to_bits(self) -> u16 {
        match self {
            Self::Top => 0,
            Self::Middle => 1,
            Self::Bottom => 2,
        }
    }
}

impl GdsHJustify {
    pub fn from_bits(bits: u16) -> Option<Self> {
        match bits {
            0 => Some(Self::Left),
            1 => Some(Self::Center),
            2 => Some(Self::Right),
            _ => None,
        }
    }

    pub fn to_bits(self) -> u16 {
        match self {
            Self::Left => 0,
            Self::Center => 1,
            Self::Right => 2,
        }
    }
}

impl fmt::Display for GdsHJustify {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Left => "Left",
            Self::Center => "Center",
            Self::Right => "Right",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for GdsVJustify {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Top => "Top",
            Self::Middle => "Middle",
            Self::Bottom => "Bottom",
        };
        write!(f, "{}", s)
    }
}

impl GdsPresentation {
    pub fn from_u16(bits: u16) -> Result<Self, String> {
        let font_bits = (bits >> 4) & 0b11;       // bits 10–11
        let vj_bits = (bits >> 2) & 0b11;         // bits 12–13
        let hj_bits = (bits >> 0) & 0b11;         // bits 14–15

        let font_number = GdsFontNumber::from_bits(font_bits)
            .ok_or_else(|| format!("Invalid font '{font_bits}'"))?;
        let v_justify = GdsVJustify::from_bits(vj_bits)
            .ok_or_else(|| format!("Invalid vertical justify '{vj_bits}'"))?;
        let h_justify = GdsHJustify::from_bits(hj_bits)
            .ok_or_else(|| format!("Invalid hori justify '{hj_bits}'"))?;

        Ok(Self {
            font_number,
            v_justify,
            h_justify,
        })
    }

    pub fn to_u16(&self) -> u16 {
        let font_bits = self.font_number.to_bits() << 4;
        let vj_bits = self.v_justify.to_bits() << 2;
        let hj_bits = self.h_justify.to_bits() << 0;
        (font_bits | vj_bits | hj_bits) << 10
    }
}

impl fmt::Display for GdsPresentation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Font Number: {}, Vertical Justify: {}, Horizontal Justify: {}",
            self.font_number, self.v_justify, self.h_justify
        )
    }
}