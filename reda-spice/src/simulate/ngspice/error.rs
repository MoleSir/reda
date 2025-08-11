use thiserror::Error;

#[derive(Error, Debug)]
pub enum NgSpiceError {
    #[error("unsupported platform")]
    Platform,

    #[error("load library failed for '{0}'")]
    Library(String),

    #[error("execute command '{command}' failed for '{reason}'")]
    Command { command: String, reason: String },

    #[error("load circuit '{circuit}' failed for '{reason}'")]
    Circuit { circuit: String, reason: String },

    #[error("result '{0}' not found!")]
    ResultNotFound(String),

    #[error("io error '{0}'")]
    Io(#[from] std::io::Error),

    #[error("miss point")]
    MissingPoints,

    #[error("no time in transient analysis")]
    NoTimeInTranAnalysis,

    #[error("no frequency in ac analysis")]
    NoFrequencyInAcAnalysis,

    #[error("no sweep in dc analysis")]
    NoSweepInDCAnalysis,

    #[error("unexpect complex value")]
    UnexpectComplexValue,

    #[error("parse .raw error: '{0}'")]
    ParseRawFile(String),

    #[error("noexit ffi '{0}'")]
    FFi(#[from] std::ffi::NulError),

    #[error("no exit v-sweep in .dc voltage analysis")]
    NoVSweepInDcVolatgeAnalysis,

    #[error("no exit i-sweep in .dc current analysis")]
    NoISweepInDcCurrentAnalysis,

    #[error("no exit t-sweep in .dc time analysis")]
    NoTSweepInDcTimeAnalysis,
}

pub type NgSpiceResult<R> = Result<R, NgSpiceError>; 

impl NgSpiceError {
    pub fn command(command: String, reason: String) -> Self {
        Self::Command { command, reason }
    }

    pub fn circuit(circuit: String, reason: String) -> Self {
        Self::Circuit { circuit, reason }
    }
}

impl From<libloading::Error> for NgSpiceError {
    fn from(value: libloading::Error) -> Self {
        Self::Library(format!("{}", value))
    }
}