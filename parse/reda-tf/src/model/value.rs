#[derive(Debug, Clone)]
pub enum TfValue {
    Float(f64),
    Integer(u32),
    String(String),
    FloatValue(Vec<f64>),
}