use crate::{TfColorDefine, TfColorDefineBuilder};
use super::{ast::Rule, TfReadResult};
use super::ast::{parse_boolean, parse_string, parse_integer};

pub fn parse_color_define(pair: pest::iterators::Pair<Rule>) -> TfReadResult<TfColorDefine> {
    let mut builder = TfColorDefineBuilder::default();
    let mut inner_rules = pair.into_inner();

    builder.color_id(parse_integer(inner_rules.next().unwrap()));

    // 2. Properties
    for inner in inner_rules {
        match inner.as_rule() {
            Rule::color_name => builder.name(parse_string(inner.into_inner().next().unwrap())),
            Rule::color_rgb_def => builder.rgb_defined(parse_boolean(inner.into_inner().next().unwrap())),
            Rule::color_red => builder.red(parse_integer(inner.into_inner().next().unwrap())),
            Rule::color_green => builder.green(parse_integer(inner.into_inner().next().unwrap())),
            Rule::color_blue => builder.blue(parse_integer(inner.into_inner().next().unwrap())),
            _ => panic!(),
        };
    }

    let value = builder.build()?; 
    Ok(value)
}
