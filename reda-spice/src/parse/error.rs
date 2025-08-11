
#[derive(Debug, thiserror::Error)]
pub enum SpiceReadError {
    #[error("IO error '{0}'")]
    Io(#[from] std::io::Error),

    #[error("Parse error '{0}'")]
    Parse(String),
}
