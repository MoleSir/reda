use derive_builder::Builder;
use reda_geometry::shape::Rect;
use crate::GdsCoord;

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct GdsBoundary {
    #[builder(default)]
    pub elf_flags: Option<i16>,
    
    #[builder(default)]
    pub plex: Option<i32>,
    pub layer: i16,

    #[builder(default)]
    pub data_type: i16,
    pub xy: Vec<GdsCoord>,
}

impl GdsBoundary {
    pub fn rect(layer: i16, leftdown: impl Into<GdsCoord>, rightup: impl Into<GdsCoord>) -> Self {
        let leftdown = leftdown.into();
        let rightup = rightup.into();
        let leftup = (leftdown.x, rightup.y).into();
        let rightdown = (rightup.x, leftdown.y).into();
        let xy = vec![leftdown, leftup, rightup, rightdown, leftdown];
        GdsBoundary {
            elf_flags: None,
            plex: None,
            layer,
            data_type: 0,
            xy,
        }
    }
    
    pub fn from_rect(layer: i16, rect: impl Into<Rect<i32>>) -> Self {
        let rect: Rect<i32> = rect.into();
        let leftdown = rect.lower_left();
        let rightup = rect.upper_right();
        let leftup = rect.upper_left();
        let rightdown = rect.lower_right();
        let xy = vec![leftdown, leftup, rightup, rightdown, leftdown];
        GdsBoundary {
            elf_flags: None,
            plex: None,
            layer,
            data_type: 0,
            xy,
        }
    }

    pub fn new(layer: i16) -> Self {
        GdsBoundary {
            elf_flags: None,
            plex: None,
            layer,
            data_type: 0,
            xy: vec![],
        }
    }

    // plase ensure is a rect
    pub fn to_rect(&self) -> Rect<i32> {
        Rect::new(self.xy[0], self.xy[2])
    }
}