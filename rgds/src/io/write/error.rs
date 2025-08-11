
#[derive(Debug, thiserror::Error)]
pub enum GdsWriteError {
    #[error("Io error '{0}'")]
    Io(#[from] std::io::Error),
}

pub type GdsWriteResult<T> = Result<T, GdsWriteError>;