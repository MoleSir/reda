use std::num::ParseIntError;


#[derive(Debug, thiserror::Error)]
pub enum DefReadError {
    #[error(transparent)]
    ParseIntError(ParseIntError),

    #[error("{0}")]
    Si2(String),

    #[error("Unsupport fixed VIARULE {0}")]
    UnsupportFixedViaRule(String),

    #[error("Invalid Literal {0}")]
    InvalidLiteral(String),

    #[error("{0}")]
    Msg(String),
}

pub type DefReadResult<T> = Result<T, DefReadError>; 