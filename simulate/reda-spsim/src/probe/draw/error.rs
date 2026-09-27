
#[derive(Debug, thiserror::Error)]
pub enum DrawerError {
    #[error("fill background error: {0}")]
    FillBackground(String),

    #[error("draw chart error: {0}")]
    DrawChart(String),

    #[error("build cartesian error: {0}")]
    BuildCartesian(String),

    #[error("draw line {0} error: {1}")]
    DrawLine(String, String),
}