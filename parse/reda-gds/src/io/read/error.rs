use std::string::FromUtf8Error;

use crate::{
    GdsArefBuilderError, GdsBoundaryBuilderError, GdsBoxBuilderError, GdsLibraryBuilderError, GdsNodeBuilderError, GdsPathBuilderError, GdsSrefBuilderError, GdsTextBuilderError 
};
use crate::io::record::GdsRecordType;

#[derive(Debug, thiserror::Error)]
pub enum GdsReadError {
    #[error("Io error '{0}'")]
    Io(#[from] std::io::Error),

    #[error("Parse utf8 failed '{0}'")]
    Utf8(#[from] FromUtf8Error),

    #[error("Unsupport record type: '{0}'")]
    UnsupportRecordType(u16),

    #[error("Record size must >= 4, but got '{0}'")]
    InvalidRecordSize(usize),

    #[error("Expect record size '{0}', but got '{1}'")]
    UnexpectRecordSize(usize, usize),

    #[error("Expect record type '{0}', but got '{1}'")]
    UnexpectRecordType(GdsRecordType, GdsRecordType),

    #[error("Invalid format value '{0}'")]
    InvalidFormat(u16),

    #[error("Build boundary failed for '{0}'")]
    BuildBoundary(#[from] GdsBoundaryBuilderError),

    #[error("Build path failed for '{0}'")]
    BuildPath(#[from] GdsPathBuilderError),

    #[error("Build sref failed for '{0}'")]
    BuildSref(#[from] GdsSrefBuilderError),

    #[error("Build aref failed for '{0}'")]
    BuildAref(#[from] GdsArefBuilderError),

    #[error("Build text failed for '{0}'")]
    BuildText(#[from] GdsTextBuilderError),

    #[error("Build node failed for '{0}'")]
    BuildNode(#[from] GdsNodeBuilderError),

    #[error("Build box failed for '{0}'")]
    BuildBox(#[from] GdsBoxBuilderError),

    #[error("Build library failed for '{0}'")]
    BuildLibrary(#[from] GdsLibraryBuilderError),

    #[error("Except one coord in XY, but got {0}")]
    ExecptPosition(usize),

    #[error("When {0} >> {1}")]
    Wrap(String, Box<GdsReadError>)
}

impl GdsReadError {
    pub fn wrap<S: Into<String>>(self, context: S) -> Self {
        Self::Wrap(context.into(), Box::new(self))
    }
}

pub type GdsReadResult<T> = Result<T, GdsReadError>;
