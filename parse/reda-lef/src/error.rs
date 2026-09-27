#[derive(Debug, thiserror::Error)]
pub enum LefError {
    #[error("{0}")]
    Si2(String),

    #[error("Unsupport fixed VIARULE {0}")]
    UnsupportFixedViaRule(String),

    #[error("{0}")]
    Msg(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Nul(#[from] std::ffi::NulError),

    #[error("Writer error: code {0}")]
    Write(i32),
}

pub type LefResult<T> = Result<T, LefError>;
