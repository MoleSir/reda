use reda_unit::Time;
use crate::probe::DrawerError;

#[derive(Debug, thiserror::Error)]
pub enum AnalysisError {
    #[error("No node {0} exits")]
    NoExitNode(String),

    #[error("No node {0} exits")]
    NoExitBranch(String),

    #[error("Time '{0} 'out of range")]
    TimeOutOfRange(Time),

    #[error("Inner error: {0}")]
    InnerError(String),

    #[error("Plot error: {0}")]
    PlotError(DrawerError),
}
