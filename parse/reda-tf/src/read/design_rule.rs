use std::collections::HashMap;
use crate::read::TfReadError;
use crate::{TfDesignRule, TfLayerDesignRule, TfLayerPairDesignRule};
use super::{ast::Rule, TfReadResult};
use super::ast::{parse_attr, parse_string};

pub fn parse_design_rule(pair: pest::iterators::Pair<Rule>) -> TfReadResult<TfDesignRule> {
    let pairs = pair.into_inner();

    let mut layer: Option<String> = None;
    let mut layer1: Option<String> = None;
    let mut layer2: Option<String> = None;
    let mut attrs = HashMap::new();

    for p in pairs {
        match p.as_rule() {
            Rule::design_rule_layer => {
                let inner = p.into_inner().next().unwrap(); // 获取内部的 string_literal
                layer = Some(parse_string(inner));
            },
            Rule::design_rule_layer1 => {
                let inner = p.into_inner().next().unwrap();
                layer1 = Some(parse_string(inner));
            },
            Rule::design_rule_layer2 => {
                let inner = p.into_inner().next().unwrap();
                layer2 = Some(parse_string(inner));
            },
            Rule::attr => {
                let (key, value) = parse_attr(p);
                attrs.insert(key, value);
            },
            _ => {} 
        }
    }

    if let Some(l) = layer {
        let rule = TfLayerDesignRule {
            layer: l,
            attrs,
        };    
        Ok(TfDesignRule::Layer(rule))
    } else if let (Some(l1), Some(l2)) = (layer1, layer2) {
        let rule = TfLayerPairDesignRule {
            layer1: l1,
            layer2: l2,
            attrs,
        };
        Ok(TfDesignRule::LayerPair(rule))
    } else {
        Err(TfReadError::DesignRuleInvalid("not layer or (layer1 / layer2)".into()))
    }
}