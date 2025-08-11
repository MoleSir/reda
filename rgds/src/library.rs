
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;
use derive_builder::Builder;

pub use crate::models::*;
pub use crate::io::*;

#[derive(Clone, Builder)]
#[builder(setter(strip_option))]
pub struct GdsLibrary {
    pub version: i16,
    pub create_date: GdsDateTime,
    pub modify_date: GdsDateTime,
    pub name: String,

    #[builder(default)]
    pub reflibs: Option<[String; 2]>,
    #[builder(default)]
    pub fonts: Option<[String; 4]>,
    #[builder(default)]
    pub attrtable: Option<String>,
    #[builder(default)]
    pub generations: Option<i16>,
    #[builder(default)]
    pub format: Option<GdsFormat>,

    pub usrunits_per_dbunit: f64,
    pub meters_per_dbunit: f64,

    pub structures: HashMap<String, Arc<RwLock<GdsStructure>>>,
}


impl GdsLibrary {
    pub fn read_gds<P: AsRef<Path>>(path: P) -> GdsReadResult<Self> {
        let mut reader = GdsReader::open(path)?;
        reader.read()
    }

    pub fn write_gds<P: AsRef<Path>>(&self, path: P) -> GdsWriteResult<()> {
        let mut writer = GdsWriter::open(path)?;
        writer.write(self)
    }

    pub fn write_text<P: AsRef<Path>>(&self, path: P) -> GdsWriteResult<()> {
        let mut writer = TextWriter::open(path)?;
        writer.write(self)
    }
}
