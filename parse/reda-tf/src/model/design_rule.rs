use std::collections::HashMap;
use super::TfValue;

#[derive(Debug, Clone)]
pub enum TfDesignRule {
    LayerPair(TfLayerPairDesignRule),
    Layer(TfLayerDesignRule)
}

#[derive(Debug, Clone)]
pub struct TfLayerPairDesignRule {
    pub layer1: String,
    pub layer2: String,
    pub attrs: HashMap<String, TfValue>,    
}

#[derive(Debug, Clone)]
pub struct TfLayerDesignRule {
    pub layer: String,
    pub attrs: HashMap<String, TfValue>,
}
