
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Fmt(#[from] std::fmt::Error),

    #[error("build graph error")]
    BuildGroup(String),

    #[error("{msg} >> {err}")]
    Context { msg: String, err: Box<dyn std::error::Error> }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait ErrorContext<T> {
    fn context<S: Into<String>>(self, msg: S) -> Result<T>;
    fn with_context<S: Into<String>>(self, f: impl Fn() -> S) -> Result<T>;
}

impl<T, E: std::error::Error + 'static> ErrorContext<T> for std::result::Result<T, E> {
    fn context<S: Into<String>>(self, msg: S) -> Result<T> {
        self.map_err(|e| Error::Context { msg: msg.into(), err: Box::new(e) }) 
    }

    fn with_context<S: Into<String>>(self, f: impl Fn() -> S) -> Result<T> {
        let msg = f();
        self.context(msg)
    }
}