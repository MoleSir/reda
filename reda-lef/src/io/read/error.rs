use nom::{error::VerboseError, IResult};

pub type LefReadRes<'a, U> = IResult<&'a str, U, VerboseError<&'a str>>;

#[derive(Debug, thiserror::Error)]
pub enum LefReadError {
    #[error("IO error '{0}'")]
    Io(#[from] std::io::Error),

    #[error("Parse error '{0}'")]
    Parse(String),
}   
