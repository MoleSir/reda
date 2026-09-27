use std::collections::HashMap;

use crate::read::ast::parse_attr;
use crate::{TfTechnology, TfTechnologyBuilder};
use super::{ast::Rule, TfReadResult};
use super::ast::{parse_string, parse_float, parse_integer};

pub fn parse_technology(pair: pest::iterators::Pair<Rule>) -> TfReadResult<TfTechnology> {
    let mut builder = TfTechnologyBuilder::default();
    let mut attrs = HashMap::new();

    macro_rules! set_attr {
        ($inner:expr, $method:ident, $parser:ident) => {
            { builder.$method($parser($inner.into_inner().next().unwrap())); }
        };
    }

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::technology_name => set_attr!(inner, name, parse_string),
            Rule::technology_dielectric => set_attr!(inner, dielectric, parse_float),
            Rule::technology_grid_res => set_attr!(inner, grid_resolution, parse_integer),
            Rule::technology_unit_time => set_attr!(inner, time_unit, parse_string),
            Rule::technology_time_prec => set_attr!(inner, time_precision, parse_integer),
            Rule::technology_unit_len => set_attr!(inner, length_unit, parse_string),
            Rule::technology_len_prec => set_attr!(inner, length_precision, parse_integer),
            Rule::technology_unit_volt => set_attr!(inner, voltage_unit, parse_string),
            Rule::technology_volt_prec => set_attr!(inner, voltage_precision, parse_integer),
            Rule::technology_unit_curr => set_attr!(inner, current_unit, parse_string),
            Rule::technology_curr_prec => set_attr!(inner, current_precision, parse_integer),
            Rule::technology_unit_pwr => set_attr!(inner, power_unit, parse_string),
            Rule::technology_pwr_prec => set_attr!(inner, power_precision, parse_integer),
            Rule::technology_unit_res => set_attr!(inner, resistance_unit, parse_string),
            Rule::technology_res_prec => set_attr!(inner, resistance_precision, parse_integer),
            Rule::technology_unit_cap => set_attr!(inner, capacitance_unit, parse_string),
            Rule::technology_cap_prec => set_attr!(inner, capacitance_precision, parse_integer),
            Rule::technology_unit_ind => set_attr!(inner, inductance_unit, parse_string),
            Rule::technology_ind_prec => set_attr!(inner, inductance_precision, parse_integer),
            Rule::technology_min_temp => set_attr!(inner, min_baseline_temperature, parse_integer),
            Rule::technology_nom_temp => set_attr!(inner, nom_baseline_temperature, parse_integer),
            Rule::technology_max_temp => set_attr!(inner, max_baseline_temperature, parse_integer),
            Rule::attr => {
                let (key, value) = parse_attr(inner);
                attrs.insert(key, value);
            }
            _ => panic!("Error: {:?}, {}", inner.as_rule(), parse_string(inner.into_inner().next().unwrap())),
        };
    }

    builder.attrs(attrs);
    let value = builder.build()?; 
    Ok(value)
}