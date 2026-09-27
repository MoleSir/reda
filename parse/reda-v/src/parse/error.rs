#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("IO error '{0}'")]
    Io(#[from] std::io::Error),

    #[error("Parse error '{0}'")]
    Parse(String),
}
