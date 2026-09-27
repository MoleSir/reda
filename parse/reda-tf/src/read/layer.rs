use std::collections::HashMap;

use crate::read::ast::{parse_boolean, parse_color};
use crate::{TfCutLayer, TfCutLayerBuilder, TfLayer, TfLayerBase, TfLayerBaseBuilder, TfRouteLayer, TfRouteLayerBuilder, TfMastersliceLayer, TfMastersliceLayerBuilder};
use super::{ast::Rule, TfReadResult};
use super::ast::{parse_attr, parse_float, parse_integer, parse_string};

pub fn parse_layer(pair: pest::iterators::Pair<Rule>) -> TfReadResult<TfLayer> {
    let mut builder = TfLayerBaseBuilder::default();
    let mut inner_rules = pair.into_inner();

    // 1. Layer Name string
    let layer_name = parse_string(inner_rules.next().unwrap());

    // 2. Properties (Base and Rules are mixed in the iterator stream, though grammar enforces order)
    let base_rules = inner_rules.next().unwrap().into_inner();
    for inner in base_rules {
        match inner.as_rule() {
            Rule::layer_mask_name => builder.mask_name(parse_string(inner.into_inner().next().unwrap())),
            Rule::layer_number => builder.layer_number(parse_integer(inner.into_inner().next().unwrap())),
            Rule::layer_visible => builder.visible(parse_boolean(inner.into_inner().next().unwrap())),
            Rule::layer_selectable => builder.selectable(parse_boolean(inner.into_inner().next().unwrap())),
            Rule::layer_blink => builder.blink(parse_boolean(inner.into_inner().next().unwrap())),
            Rule::layer_is_default => builder.is_defaultlayer(parse_boolean(inner.into_inner().next().unwrap())),
            Rule::layer_line_style => builder.line_style(parse_string(inner.into_inner().next().unwrap())),
            Rule::layer_pattern => builder.pattern(parse_string(inner.into_inner().next().unwrap())),
            Rule::layer_color => builder.color(parse_color(inner.into_inner().next().unwrap())),
            _ => panic!("Error: {:?}, {}", inner.as_rule(), parse_string(inner.into_inner().next().unwrap())),
        };
    }

    // 3. Rule
    let base = builder.build()?;
    let rule_pair = inner_rules.next().unwrap();
    if base.mask_name.is_empty() || base.mask_name == "implant" || base.mask_name.contains("poly") {
        let layer = parse_masterslice_layer(layer_name, base, rule_pair)?;
        Ok(TfLayer::Masterslice(layer))
    } else if base.mask_name.contains("metal") {
        let layer = parse_route_layer(layer_name, base, rule_pair)?;
        Ok(TfLayer::Route(layer))
    } else if base.mask_name.contains("via") {
        let layer = parse_cut_layer(layer_name, base, rule_pair)?;
        Ok(TfLayer::Cut(layer))
    } else {
        panic!();
    }
}

fn parse_masterslice_layer(layer_name: String, base: TfLayerBase, pair: pest::iterators::Pair<Rule>) -> TfReadResult<TfMastersliceLayer> {
    let mut builder = TfMastersliceLayerBuilder::default();
    builder.name(layer_name);
    builder.base(base);
    let mut attrs =  HashMap::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::layer_pitch => { builder.pitch(parse_float(inner.into_inner().next().unwrap())); },
            Rule::layer_def_width => { builder.default_width(parse_float(inner.into_inner().next().unwrap())); },
            Rule::layer_min_width => { builder.min_width(parse_float(inner.into_inner().next().unwrap())); },
            Rule::layer_min_spacing => { builder.min_spacing(parse_float(inner.into_inner().next().unwrap())); },
            Rule::attr => {
                let (key, value) = parse_attr(inner);
                attrs.insert(key, value);
            }
            _ => {},
        };
    }
    
    builder.attrs(attrs);
    let layer = builder.build()?;
    Ok(layer)
}

fn parse_route_layer(layer_name: String, base: TfLayerBase, pair: pest::iterators::Pair<Rule>) -> TfReadResult<TfRouteLayer> {
    let mut builder = TfRouteLayerBuilder::default();
    builder.name(layer_name);
    builder.base(base);
    let mut attrs =  HashMap::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::layer_pitch => { builder.pitch(parse_float(inner.into_inner().next().unwrap())); },
            Rule::layer_def_width => { builder.default_width(parse_float(inner.into_inner().next().unwrap())); },
            Rule::layer_min_width => { builder.min_width(parse_float(inner.into_inner().next().unwrap())); },
            Rule::layer_min_spacing => { builder.min_spacing(parse_float(inner.into_inner().next().unwrap())); },
            Rule::attr => {
                let (key, value) = parse_attr(inner);
                attrs.insert(key, value);
            }
            _ => {},
        };
    }
    
    builder.attrs(attrs);
    let layer = builder.build()?;
    Ok(layer)
}

fn parse_cut_layer(layer_name: String, base: TfLayerBase, pair: pest::iterators::Pair<Rule>) -> TfReadResult<TfCutLayer> {
    let mut builder = TfCutLayerBuilder::default();
    builder.name(layer_name);
    builder.base(base);
    let mut attrs =  HashMap::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::layer_pitch => { builder.pitch(parse_float(inner.into_inner().next().unwrap())); },
            Rule::layer_def_width => { builder.default_width(parse_float(inner.into_inner().next().unwrap())); },
            Rule::layer_min_width => { builder.min_width(parse_float(inner.into_inner().next().unwrap())); },
            Rule::layer_min_spacing => { builder.min_spacing(parse_float(inner.into_inner().next().unwrap())); },
            Rule::attr => {
                let (key, value) = parse_attr(inner);
                attrs.insert(key, value);
            }
            _ => {}
        };
    }
    
    builder.attrs(attrs);
    let layer = builder.build()?;
    Ok(layer)
}