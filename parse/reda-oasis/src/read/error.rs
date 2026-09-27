use std::string::FromUtf8Error;
use num_bigint::BigUint;

#[derive(Debug, thiserror::Error)]
pub enum ReadError {
    #[error(transparent)]
    FromUtf8(#[from] FromUtf8Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("unsigned int out of u8's range")]
    UnsignedIntOutOfRangeU8,

    #[error("unsupport real type {0}")]
    UnsupportRealType(BigUint),

    #[error("unsupport real type {0}")]
    UnsupportRepetitionType(BigUint),

    #[error("unsupport real type {0}")]
    UnsupportPointListType(BigUint),

    #[error("unsupport offset_flag {0}, only 0 or 1")]
    UnsupportOffsetFlag(u8),

    #[error("unsupport strict mode {0}, only 0 or 1")]
    UnsupportStrictMode(u8),

    #[error("unsupport validation scheme {0}, only 0/1/2")]
    UnsupportValidationScheme(u8),

    #[error("string too long {0}")]
    StringTooLong(BigUint),

    #[error("string too long {0}")]
    DimensionTooLong(BigUint),

    #[error("repetition dimension == 0")]
    RepetitionDimensionZero,

    #[error("Invalid vertex count {0} for {1}")]
    InvalidVertexCount(BigUint, &'static str),

    #[error("invalid a-string byte {0}")]
    InvalidAStringByte(u8),

    #[error("invalid n-string byte {0}")]
    InvalidNStringByte(u8),

    #[error("empty n-string")]
    EmptyNString,

    #[error("when {0} > {1}")]
    Context(String, Box<ReadError>),
}

pub type ReadResult<T> = Result<T, ReadError>;

pub trait ErrorContext {
    fn context<S: Into<String>>(self, ctx: S) -> Self;
}

impl<T> ErrorContext for Result<T, ReadError> {
    fn context<S: Into<String>>(self, ctx: S) -> Self {
        self.map_err(|e| ReadError::Context(ctx.into(), Box::new(e)))
    }
}