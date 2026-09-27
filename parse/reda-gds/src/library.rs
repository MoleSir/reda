
use std::collections::HashMap;
use std::io::Read;
use std::io::Seek;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;
use derive_builder::Builder;

pub use crate::models::*;
pub use crate::io::*;

#[derive(Clone, Builder)]
#[builder(setter(strip_option, into))]
pub struct GdsLibrary {
    #[builder(default)]
    pub version: i16,
    pub create_date: GdsDateTime,
    pub modify_date: GdsDateTime,
    pub name: String,

    /// The size of a database unit in user units. 
    /// 1 database unit = xx user unit
    /// default = 0.001
    pub usrunits_per_dbunit: f64,
    /// The size of a database unit in meters.
    /// 1 database unit = xx meter
    /// default = 1e-9 
    pub meters_per_dbunit: f64,

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

    #[builder(default)]
    pub structures: HashMap<String, Arc<RwLock<GdsStructure>>>,
}

impl GdsLibrary {
    pub fn new(name: impl Into<String>) -> Self {
        GdsLibraryBuilder::default()
            .name(name)
            .create_date(GdsDateTime::now())
            .modify_date(GdsDateTime::now())
            .version(0i16)
            .usrunits_per_dbunit(0.001)
            .meters_per_dbunit(1e-9)
            .build()
            .unwrap()
    }

    pub fn with_version(&mut self, version: i16) {
        self.version = version;
    }

    pub fn usrunits_per_dbunit(&mut self, value: f64) {
        self.usrunits_per_dbunit = value;
    }
    
    pub fn meters_per_dbunit(&mut self, value: f64) {
        self.meters_per_dbunit = value;
    }
    
    pub fn new_structure(&mut self, name: impl Into<String>) -> Arc<RwLock<GdsStructure>> {
        let name = name.into();
        let structure = Arc::new(RwLock::new(GdsStructure::new(name.clone())));
        self.structures.insert(name, structure.clone());
        structure
    }

    pub fn add_structure(&mut self, structure: Arc<RwLock<GdsStructure>>) {
        let name = structure.read().unwrap().name.clone();
        self.structures.insert(name, structure);
    }

    pub fn dbuint_to_micro(&self, uint: i32) -> f64 {
        let ratio = 1. / (self.meters_per_dbunit * 1e6);
        uint as f64 / ratio
    }

    pub fn micro_to_dbunit(&self, micro: f64) -> i32 {
        let ratio = 1. / (self.meters_per_dbunit * 1e6);
        f64::round(micro * ratio) as i32
    }

    pub fn structure_size(&self) -> usize {
        self.structures.len()
    }
}

impl GdsLibrary {
    pub fn load_file<P: AsRef<Path>>(path: P) -> GdsReadResult<Self> {
        let mut reader = GdsReader::open(path)?;
        reader.read()
    }

    pub fn save_gds_file<P: AsRef<Path>>(&self, path: P) -> GdsWriteResult<()> {
        let mut writer = GdsWriter::open(path)?;
        writer.write(self)
    }

    pub fn save_text_file<P: AsRef<Path>>(&self, path: P) -> GdsWriteResult<()> {
        let mut writer = TextWriter::open(path)?;
        writer.write(self)
    }

    pub fn read<R: Read + Seek>(reader: R) -> GdsReadResult<Self> {
        let mut reader = GdsReader::new(reader)?;
        reader.read()
    }
    
    pub fn write_gds<W: std::io::Write>(&self, writer: W) -> GdsWriteResult<()> {
        let mut writer = GdsWriter::new(writer);
        writer.write(self)
    }

    pub fn write_text<W: std::io::Write>(&self, writer: W) -> GdsWriteResult<()> {
        let mut writer = TextWriter::new(writer);
        writer.write(self)
    }
}
