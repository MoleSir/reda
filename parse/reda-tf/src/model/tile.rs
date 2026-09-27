use derive_builder::Builder;

#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct TfTile {
    pub name: String,
    pub width: f64,
    pub height: f64,
}