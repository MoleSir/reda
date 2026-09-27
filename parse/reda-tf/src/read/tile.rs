use crate::{TfTile, TfTileBuilder};
use super::{ast::Rule, TfReadResult};
use super::ast::{parse_float, parse_string};

pub fn parse_tile(pair: pest::iterators::Pair<Rule>) -> TfReadResult<TfTile> {
    let mut builder = TfTileBuilder::default();
    let mut inner_rules = pair.into_inner();

    builder.name(parse_string(inner_rules.next().unwrap()));
    for inner in inner_rules {
        match inner.as_rule() {
            Rule::tile_width => builder.width(parse_float(inner.into_inner().next().unwrap())),
            Rule::tile_height => builder.height(parse_float(inner.into_inner().next().unwrap())),
            _ => panic!(), 
        };
    }

    let value = builder.build()?;
    Ok(value)
}