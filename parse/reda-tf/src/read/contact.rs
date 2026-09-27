use std::collections::HashMap;
use crate::{read::ast::{parse_attr, parse_float, parse_integer, parse_string}, TfContact, TfContactBuilder};
use super::{ast::Rule, TfReadResult};

pub fn parse_contact_code(pair: pest::iterators::Pair<Rule>) -> TfReadResult<TfContact> {
    let mut builder = TfContactBuilder::default();
    let mut inner_rules = pair.into_inner();
    let mut attrs = HashMap::new();

    let name_pair = inner_rules.next().unwrap();
    builder.name(parse_string(name_pair));

    macro_rules! set_attr {
        ($inner:expr, $method:ident, $parser:ident) => {
            { builder.$method($parser($inner.into_inner().next().unwrap())); }
        };
    }

    for inner in inner_rules {
        match inner.as_rule() {
            Rule::contact_id          => set_attr!(inner, contact_id, parse_integer),
            Rule::contact_cut_layer   => set_attr!(inner, cut_layer, parse_string),
            Rule::contact_lower_layer => set_attr!(inner, lower_layer, parse_string),
            Rule::contact_upper_layer => set_attr!(inner, upper_layer, parse_string),
            Rule::contact_cut_w       => set_attr!(inner, cut_width, parse_float),
            Rule::contact_cut_h       => set_attr!(inner, cut_height, parse_float),
            Rule::contact_up_enc_w    => set_attr!(inner, upper_layer_enc_width, parse_float),
            Rule::contact_up_enc_h    => set_attr!(inner, upper_layer_enc_height, parse_float),
            Rule::contact_lo_enc_w    => set_attr!(inner, lower_layer_enc_width, parse_float),
            Rule::contact_lo_enc_h    => set_attr!(inner, lower_layer_enc_height, parse_float),
            Rule::contact_min_spacing => set_attr!(inner, min_cut_spacing, parse_float),
            Rule::attr => {
                let (key, value) = parse_attr(inner);
                attrs.insert(key, value);
            }
            _ => {} 
        }
    }
    
    builder.attrs(attrs);
    let value = builder.build()?;
    Ok(value)
}