use crate::{TechfileBuilderError, TfColorDefineBuilderError, TfContactBuilderError, TfCutLayerBuilderError, TfLayerBaseBuilderError, TfRouteLayerBuilderError, TfMastersliceLayerBuilderError, TfTechnologyBuilderError, TfTileBuilderError};

use super::ast::Rule;


#[derive(Debug, thiserror::Error)]
pub enum TfReadError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("parse: {0}")]
    Parse(#[from] pest::error::Error<Rule>),

    #[error("technology attr lack: {0}")]
    TechnologyBuildError(#[from] TfTechnologyBuilderError),

    #[error("tile attr lack: {0}")]
    TileBuildError(#[from] TfTileBuilderError),

    #[error("color define attr lack: {0}")]
    ColorDefBuildError(#[from] TfColorDefineBuilderError),

    #[error("layer base attr lack: {0}")]
    LayerBaseBuildError(#[from] TfLayerBaseBuilderError),

    #[error("poly layer attr lack: {0}")]
    MastersliceLayerBuildError(#[from] TfMastersliceLayerBuilderError),

    #[error("metal layer attr lack: {0}")]
    RouteLayerBuildError(#[from] TfRouteLayerBuilderError),

    #[error("cut layer attr lack: {0}")]
    CutLayerBuildError(#[from] TfCutLayerBuilderError),

    #[error("contact attr lack: {0}")]
    ContectBuildError(#[from] TfContactBuilderError),

    #[error("techfile {0}")]
    TechfileBuildError(#[from] TechfileBuilderError),

    #[error("invalid design rule: {0}")]
    DesignRuleInvalid(String),
}


pub type TfReadResult<T> = Result<T, TfReadError>;