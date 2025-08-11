
/// Contains two bytes of bit flags for Sref, Aref, and text transforrnation. 
/// - Bit 0 (the leftmost bit) specifies reflection. 
///   If bit 0 is set, the element is reflected about the X-axis before angular rotation. 
///   For an Aref, the entire array is reflected, with the individual array members rigidly attached. 
/// - Bit 13 flags absolute magnification. 
/// - Bit 14 flags absolute angle. 
/// - Bit 15 (the rightmost bit) and all remaining bits are reserved for future use and must be cleared. 
///   If this record is omitted, the element is assumed to have no reflection, non-absolute magnification, and non- absolute angle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GdsTransformFlag {
    /// Reflect the element about the X-axis before rotation.
    pub reflect: bool,              // Bit 0 (leftmost bit)
    /// Use absolute magnification.
    pub absolute_magnification: bool, // Bit 13
    /// Use absolute angle.
    pub absolute_angle: bool,       // Bit 14
}

impl GdsTransformFlag {
    /// Parse flags from a u16 bitfield.
    pub fn from_u16(bits: u16) -> Self {
        Self {
            reflect:                (bits & (1 << 15)) != 0, // Bit 0 (leftmost) == bit 15
            absolute_magnification: (bits & (1 << 2)) != 0,  // Bit 13 == bit 2 from MSB
            absolute_angle:         (bits & (1 << 1)) != 0,  // Bit 14 == bit 1 from MSB
        }
    }

    /// Convert the flags into a u16 bitfield.
    pub fn to_u16(&self) -> u16 {
        let mut bits = 0u16;
        if self.reflect {
            bits |= 1 << 15; // Bit 0 (leftmost)
        }
        if self.absolute_magnification {
            bits |= 1 << 2;  // Bit 13
        }
        if self.absolute_angle {
            bits |= 1 << 1;  // Bit 14
        }
        bits
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GdsTransform {
    /// STRANS record flags
    pub flag: GdsTransformFlag,

    /// MAG record 
    /// Eight-Byte Real Contains a double-precision real number (8 bytes), which is the magnification factor. 
    /// If this record is omitted, a magnification factor of one is assumed.
    pub magnification: f64,

    /// ANGLE record 
    /// Eight-Byte Real Contains a double-precision real number (8 bytes), which is the angular rotation factor. 
    pub angle: f64,
}

impl GdsTransform {
    pub fn with_flag(bits: u16) -> Self {
        Self {
            flag: GdsTransformFlag::from_u16(bits),
            magnification: 1.0,
            angle: 0.0,
        }
    }
}