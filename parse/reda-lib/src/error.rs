use std::num::ParseFloatError;


#[derive(Debug, thiserror::Error)]
pub enum LibError {
    #[error("except group")]
    ExceptLibGroup,

    #[error("library param should only a name")]
    LibraryParamLenUnmatch,

    #[error("expect attr {0} but not found")]
    MissAttr(String),

    #[error("unsupport direction lliteral {0}")]
    UnsupportDirection(String),

    #[error("unsupport antenna diode {0}")]
    UnsupportAntennaDiodeType(String),

    #[error("unsupport driver type {0}")]
    UnsupportDriverType(String),

    #[error("unsupport dont fault {0}")]
    UnsupportDontFault(String),

    #[error("unsupport next state type {0}")]
    UnsupportNextStateType(String),

    #[error("unsupport enum {1} for {0}")]
    UnsupportEnumValue(&'static str, String),

    #[error("not only ff group")]
    NotOnlyFF,

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    ParseFloat(ParseFloatError),

    #[error("parse expr rror: {0}")]
    ParseExpr(String),

    #[error("parse error: {0}")]
    Parse(String),

    #[error("value kind unmatch, expect {0} but got {1}")]
    ValueKindUnmatch(&'static str, &'static str),

    #[error("{0} >> {1}")]
    Context(String, Box<LibError>),
}

pub type LibResult<T> = Result<T, LibError>;

pub trait LibErrorContext {
    fn context<S: Into<String>>(self, f: impl Fn() -> S) -> Self;
}

impl<T> LibErrorContext for Result<T, LibError> {
    fn context<S: Into<String>>(self, f: impl Fn() -> S) -> Self {
        self.map_err(|e| LibError::Context(f().into(), Box::new(e)))
    }
}