
use pest_derive::Parser;

use crate::{TfColor, TfValue};

#[derive(Parser)]
#[grammar_inline = r#"
COMMENT = _{ "/*" ~ (!"*/" ~ ANY)* ~ "*/" }
WHITESPACE = _{ " " | "\t" | "\r" | "\n" }

// ... (这里粘贴你完整的 grammar 内容) ...
// 为了节省篇幅，假设这里包含了你上面提供的完整 grammar
// 重点修正：你的 grammar 中 tile 对应 Tile，我会照此处理
// ====================================================================//
//                Base
// ====================================================================//
string_literal = @{ "\"" ~ (!"\"" ~ ANY)* ~ "\"" }
ident          = @{ ("_" | ASCII_ALPHA) ~ ("_" | ASCII_ALPHA | ASCII_DIGIT)* }
number         = @{ "-"? ~ ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT+)? ~ (^"e" ~ ("+" | "-")? ~ ASCII_DIGIT+)? }
integer        = @{ ASCII_DIGIT+ }
array_number   = { "(" ~ number ~ ("," ~ number)* ~ ")" }
value          = { string_literal | number | array_number }
attr           = { ident ~ "=" ~ value }

// ====================================================================//
//                Technology
// ====================================================================//
technology_name              = { "name" ~ "=" ~ string_literal }
technology_dielectric        = { "dielectric" ~ "=" ~ number }
technology_grid_res          = { "gridResolution" ~ "=" ~ number }
technology_unit_time         = { "unitTimeName" ~ "=" ~ string_literal }
technology_time_prec         = { "timePrecision" ~ "=" ~ number }
technology_unit_len          = { "unitLengthName" ~ "=" ~ string_literal }
technology_len_prec          = { "lengthPrecision" ~ "=" ~ number }
technology_unit_volt         = { "unitVoltageName" ~ "=" ~ string_literal }
technology_volt_prec         = { "voltagePrecision" ~ "=" ~ number }
technology_unit_curr         = { "unitCurrentName" ~ "=" ~ string_literal }
technology_curr_prec         = { "currentPrecision" ~ "=" ~ number }
technology_unit_pwr          = { "unitPowerName" ~ "=" ~ string_literal }
technology_pwr_prec          = { "powerPrecision" ~ "=" ~ number }
technology_unit_res          = { "unitResistanceName" ~ "=" ~ string_literal }
technology_res_prec          = { "resistancePrecision" ~ "=" ~ number }
technology_unit_cap          = { "unitCapacitanceName" ~ "=" ~ string_literal }
technology_cap_prec          = { "capacitancePrecision" ~ "=" ~ number }
technology_unit_ind          = { "unitInductanceName" ~ "=" ~ string_literal }
technology_ind_prec          = { "inductancePrecision" ~ "=" ~ number }
technology_min_temp          = { "minBaselineTemperature" ~ "=" ~ number }
technology_nom_temp          = { "nomBaselineTemperature" ~ "=" ~ number }
technology_max_temp          = { "maxBaselineTemperature" ~ "=" ~ number }

technology_property = _{ 
    technology_name | technology_dielectric | technology_grid_res |
    technology_unit_time | technology_time_prec |
    technology_unit_len | technology_len_prec |
    technology_unit_volt | technology_volt_prec |
    technology_unit_curr | technology_curr_prec |
    technology_unit_pwr | technology_pwr_prec |
    technology_unit_res | technology_res_prec |
    technology_unit_cap | technology_cap_prec |
    technology_unit_ind | technology_ind_prec |
    technology_min_temp | technology_nom_temp | technology_max_temp |
    attr
}

technology = { "Technology" ~ "{" ~ technology_property* ~ "}" }

// ====================================================================//
//                Tile 
// ====================================================================//
tile_width = { "width" ~ "=" ~ number }
tile_height = { "height" ~ "=" ~ number }

tile_property = _{
    tile_width | tile_height
}

tile = { "Tile" ~ string_literal ~ "{" ~ tile_property*  ~ "}" }

// ====================================================================//
//                Color
// ====================================================================//
color_name = { "name" ~ "=" ~ string_literal }
color_rgb_def    = { "rgbDefined" ~ "=" ~ integer } 
color_red        = { "redIntensity" ~ "=" ~ integer }
color_green      = { "greenIntensity" ~ "=" ~ integer }
color_blue       = { "blueIntensity" ~ "=" ~ integer }

color_property = _{ 
    color_name | color_rgb_def | color_red | color_green | color_blue 
}

color = { "Color" ~ integer ~ "{" ~ color_property* ~ "}" }

// ====================================================================//
//                Layer
// ====================================================================//
layer_number          = { "layerNumber"     ~ "=" ~ integer }
layer_mask_name       = { "maskName"        ~ "=" ~ string_literal }
layer_is_default      = { "isDefaultLayer"  ~ "=" ~ integer }
layer_visible         = { "visible"         ~ "=" ~ integer }
layer_selectable      = { "selectable"      ~ "=" ~ integer }
layer_blink           = { "blink"           ~ "=" ~ integer }
layer_line_style      = { "lineStyle"       ~ "=" ~ string_literal }
layer_pattern         = { "pattern"         ~ "=" ~ string_literal }
layer_color           = { "color"           ~ "=" ~ string_literal } 

layer_base_property = _{
      layer_mask_name | layer_number | layer_is_default | layer_visible | layer_selectable
    | layer_blink | layer_line_style | layer_pattern | layer_color
}

layer_pitch        = { "pitch"        ~ "=" ~ number }
layer_def_width    = { "defaultWidth" ~ "=" ~ number }
layer_min_width    = { "minWidth"     ~ "=" ~ number }
layer_min_spacing  = { "minSpacing"   ~ "=" ~ number }

layer_rule_property = _{
      layer_pitch | layer_def_width | layer_min_width | layer_min_spacing
    | attr
}

layer_base_propertys = { layer_base_property* }
layer_rule_propertys = { layer_rule_property* }

layer = { "Layer" ~ string_literal ~ "{" ~ layer_base_propertys ~ layer_rule_propertys ~ "}" }


// ====================================================================//
//                ContactCode (Cut)
// ====================================================================//
contact_id          = { "contactCodeNumber"   ~ "=" ~ integer }
contact_cut_layer   = { "cutLayer"            ~ "=" ~ string_literal }
contact_lower_layer = { "lowerLayer"          ~ "=" ~ string_literal }
contact_upper_layer = { "upperLayer"          ~ "=" ~ string_literal }
contact_cut_w       = { "cutWidth"            ~ "=" ~ number }
contact_cut_h       = { "cutHeight"           ~ "=" ~ number }
contact_up_enc_w    = { "upperLayerEncWidth"  ~ "=" ~ number }
contact_up_enc_h    = { "upperLayerEncHeight" ~ "=" ~ number }
contact_lo_enc_w    = { "lowerLayerEncWidth"  ~ "=" ~ number }
contact_lo_enc_h    = { "lowerLayerEncHeight" ~ "=" ~ number }
contact_min_spacing = { "minCutSpacing"       ~ "=" ~ number }

// 属性集合：注意把具体的放在前面，通用的 attr 放在最后
contact_property = _{
    contact_id |
    contact_cut_layer |
    contact_lower_layer |
    contact_upper_layer |
    contact_cut_w |
    contact_cut_h |
    contact_up_enc_w |
    contact_up_enc_h |
    contact_lo_enc_w |
    contact_lo_enc_h |
    contact_min_spacing |
    attr 
}

contact_code = { "ContactCode" ~ string_literal ~ "{" ~ contact_property* ~ "}" }


// ====================================================================//
//                ContactCode (Cut)
// ====================================================================//

design_rule_layer  = { "layer"            ~ "=" ~ string_literal }
design_rule_layer1 = { "layer1"           ~ "=" ~ string_literal }
design_rule_layer2 = { "layer2"           ~ "=" ~ string_literal }

design_rule_property = _{
    design_rule_layer | design_rule_layer1 | design_rule_layer2 | attr
}

design_rule = { "DesignRule" ~ "{" ~ design_rule_property*  ~ "}" }


// ====================================================================//
//                Top 
// ====================================================================//
file = { SOI ~ technology ~ color* ~ tile* ~ layer* ~ contact_code* ~ design_rule* ~ EOI }

"#]
pub struct TfPestParser;

pub fn parse_string(pair: pest::iterators::Pair<Rule>) -> String {
    let s = pair.as_str();
    s[1..s.len() - 1].to_string()
}

pub fn parse_float(pair: pest::iterators::Pair<Rule>) -> f64 {
    pair.as_str().parse().unwrap_or(0.0)
}

pub fn parse_floats(pair: pest::iterators::Pair<Rule>) -> Vec<f64> {
    pair.into_inner().map(|p| parse_float(p)).collect()
}

pub fn parse_integer(pair: pest::iterators::Pair<Rule>) -> u32 {
    pair.as_str().parse().unwrap()
}

pub fn parse_boolean(pair: pest::iterators::Pair<Rule>) -> bool {
    let value = parse_integer(pair);
    value == 1
}

pub fn parse_color(pair: pest::iterators::Pair<Rule>) -> TfColor {
    let color_str = parse_string(pair);
    match color_str.parse::<u32>() {
        Ok(value) => TfColor::SelfDef(value),
        Err(_) => TfColor::Builtin(color_str)
    }
}

pub fn parse_value(pair: pest::iterators::Pair<Rule>) -> TfValue {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::string_literal => TfValue::String(parse_string(pair)),
        Rule::number => TfValue::Float(parse_float(pair)),
        Rule::array_number => TfValue::FloatValue(parse_floats(pair)),
        _ => panic!(),
    }
}

pub fn parse_attr(pair: pest::iterators::Pair<Rule>) -> (String, TfValue) {
    let mut inners = pair.into_inner();
    let key = parse_string(inners.next().unwrap());
    let value = parse_value(inners.next().unwrap());
    (key, value)
}