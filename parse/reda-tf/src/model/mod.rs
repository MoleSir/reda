mod layer;

use std::path::Path;

use derive_builder::Builder;
pub use layer::*;
mod design_rule;
pub use design_rule::*;
mod contact;
pub use contact::*;
mod technology;
pub use technology::*;
mod color;
pub use color::*;
mod value;
pub use value::*;
mod tile;
pub use tile::*;
use crate::read::{self, TfReadResult};

#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct Techfile {
    pub technology: TfTechnology,
    pub tiles: Vec<TfTile>,
    pub colors: Vec<TfColorDefine>,
    pub layers: Vec<TfLayer>,
    pub contacts: Vec<TfContact>,
    pub design_rules: Vec<TfDesignRule>,
}

impl Techfile {
    pub fn load_file<P: AsRef<Path>>(path: P) -> TfReadResult<Self> {
        let input = std::fs::read_to_string(path)?;
        read::parse_techfile(&input)
    }
}