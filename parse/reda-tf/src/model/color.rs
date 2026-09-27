use derive_builder::Builder;

#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct TfColorDefine {
    pub color_id: u32,
    pub name: String,
    pub rgb_defined: bool,
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

#[derive(Debug, Clone)]
pub enum TfColor {
    SelfDef(u32),
    Builtin(String),
}
