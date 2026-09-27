use crate::error::{LibErrorContext, LibError};
use crate::model::{LibAst, LibCapacitiveLoadUnit, LibCell, LibGroup, LibLibrary, LibLuTableTemplate, LibOperatingConditions, LibValue, Model};

use super::FromLibGroup;

impl LibLibrary {
    pub fn from_ast(ast: LibAst) -> Result<Self, LibError> {
        let group = LibGroup::from_ast(ast).context(|| "semantic root ast to group")?;
        Self::from_group(group)
    }
}

impl FromLibGroup for LibLibrary {
    type Error = LibError;
    fn  from_group(group: LibGroup) -> Result<Self, LibError> {
        let mut group = group;
        let name = group.expect_name().context(|| "group name")?;
        let err_context = |msg: &str| format!("{} in `{}` library", msg, &name);

        // helpers that attach context when calling LibValue accessors
        let get_float_opt = 
            |key: &str| group.attr_float_opt(key, || err_context(&format!("semantic `{}` attr", key)));
        let get_float = 
            |key: &str| group.attr_float(key, || err_context(&format!("semantic `{}` attr", key)));
        let get_string_opt = 
            |key: &str| group.attr_string_opt(key, || err_context(&format!("semantic `{}` attr", key))).map(|v| v.map(String::from));
        let get_string = 
            |key: &str| group.attr_string(key, || err_context(&format!("semantic `{}` attr", key))).map(String::from);
        let get_ident_opt = 
            |key: &str| group.attr_ident_opt(key, || err_context(&format!("semantic `{}` attr", key))).map(|v| v.map(String::from));

        let date = get_string("date")?;
        let revision = get_string_opt("revision")?;
        let comment = get_string_opt("comment")?;
        
        let delay_model = group.attr_parse("delay_model", &|| err_context("semantic `delay_model` attr"))?;
        
        let nom_process = get_float_opt("nom_process")?;
        let nom_temperature = get_float_opt("nom_temperature")?;
        let nom_voltage = get_float_opt("nom_voltage")?;

        let default_operating_conditions = get_ident_opt("default_operating_conditions")?;

        let slew_lower_threshold_pct_fall = get_float("slew_lower_threshold_pct_fall")?;
        let slew_lower_threshold_pct_rise = get_float("slew_lower_threshold_pct_rise")?;
        let slew_upper_threshold_pct_fall = get_float("slew_upper_threshold_pct_fall")?;
        let slew_upper_threshold_pct_rise = get_float("slew_upper_threshold_pct_rise")?;
        let slew_derate_from_library = get_float("slew_derate_from_library")?;
        let input_threshold_pct_fall = get_float("input_threshold_pct_fall")?;
        let input_threshold_pct_rise = get_float("input_threshold_pct_rise")?;
        let output_threshold_pct_fall = get_float("output_threshold_pct_fall")?;
        let output_threshold_pct_rise = get_float("output_threshold_pct_rise")?;
        let default_leakage_power_density = get_float("default_leakage_power_density")?;
        let default_cell_leakage_power = get_float("default_cell_leakage_power")?;

        let time_unit = group.attr_parse_string("time_unit", &|| err_context("semantic `time_unit` attr"))?;
        let voltage_unit = group.attr_parse_string("voltage_unit", &|| err_context("semantic `voltage_unit` attr"))?;
        let current_unit = group.attr_parse_string_opt("current_unit", &|| err_context("semantic `current_unit` attr"))?;
        let pulling_resistance_unit = group.attr_parse_string_opt("pulling_resistance_unit", &|| err_context("semantic `pulling_resistance_unit` attr"))?;
        let leakage_power_unit = group.attr_parse_string_opt("leakage_power_unit", &|| err_context("semantic `leakage_power_unit` attr"))?;

        let capacitive_load_unit = match group.attr("capacitive_load_unit") {
            Some(value) => {
                if let LibValue::Vec(vec) = value {
                    if vec.len() != 2 {
                        Err(LibError::Parse("capacitive_load_unit attr should be a (float, ident)".into()))?
                    }
                    let f = vec[0].float().context(|| err_context("semantic `capacitive_load_unit` attr"))?;
                    let i = vec[1].ident().context(|| err_context("semantic `capacitive_load_unit` attr"))?;
                    match i {
                        "ff" => Some(LibCapacitiveLoadUnit::FF(f)),
                        "pf" => Some(LibCapacitiveLoadUnit::PF(f)),
                        _ => Err(LibError::UnsupportEnumValue("capacitive_load_unit", i.into()))?
                    }

                } else {
                    Err(LibError::Parse("capacitive_load_unit attr should be a (float, ident)".into()))?
                }
            }
            None => None,
        };

        let default_inout_pin_cap = get_float_opt("default_inout_pin_cap")?;
        let default_input_pin_cap = get_float_opt("default_input_pin_cap")?; 
        let default_output_pin_cap = get_float_opt("default_output_pin_cap")?; 
        let default_fanout_load = get_float_opt("default_fanout_load")?; 
        let default_max_transition = get_float_opt("default_max_transition")?; 

        let mut cells = vec![];
        let mut operating_conditions = vec![];
        let mut lu_table_templates = vec![];
        let mut groups = vec![];

        for group in group.groups {
            match group.kind.as_str() {
                "cell" => cells.push(LibCell::from_group(group).context(|| err_context("semantic cell"))?),
                "operating_conditions" => operating_conditions.push(LibOperatingConditions::from_group(group).context(|| err_context("semantic operating_conditions"))?),
                "lu_table_template" => lu_table_templates.push(LibLuTableTemplate::from_group(group).context(|| err_context("semantic lu_table_templates"))?),
                _ => groups.push(group),
            }
        }

        Ok(Self {
            name,

            date,
            revision,
            comment,

            delay_model,
            
            time_unit,
            voltage_unit,
            capacitive_load_unit,
            current_unit,
            pulling_resistance_unit,
            leakage_power_unit,

            nom_process,
            nom_temperature,
            nom_voltage,

            slew_lower_threshold_pct_fall,
            slew_lower_threshold_pct_rise,
            slew_upper_threshold_pct_fall,
            slew_upper_threshold_pct_rise,
            slew_derate_from_library,
            input_threshold_pct_fall,
            input_threshold_pct_rise,
            output_threshold_pct_fall,
            output_threshold_pct_rise,
            default_leakage_power_density,
            default_cell_leakage_power,

            default_operating_conditions,

            default_inout_pin_cap,
            default_input_pin_cap,
            default_output_pin_cap,
            default_fanout_load,
            default_max_transition,
                    
            attrs: group.attrs,
            groups,
            cells,
            lu_table_templates,
            operating_conditions,
        })
    }
}

impl FromLibGroup for LibOperatingConditions {
    type Error = LibError;

    fn from_group(group: LibGroup) -> Result<Self, Self::Error> {
        let mut group = group;
        let name = group.expect_name().context(|| "group name")?;
        let err_context = |msg: &str| format!("{} in `{}` operating_conditions", msg, &name);

        // helpers that attach context when calling LibValue accessors
        let get_float_opt = 
            |key: &str| group.attr_float_opt(key, || err_context(&format!("semantic `{}` attr", key)));
        let get_float = 
            |key: &str| group.attr_float(key, || err_context(&format!("semantic `{}` attr", key)));
        let get_string_opt = 
            |key: &str| group.attr_string_opt(key, || err_context(&format!("semantic `{}` attr", key))).map(|v| v.map(String::from));

        let process = get_float("process")?;
        let voltage = get_float("voltage")?;
        let temperature = get_float("temperature")?;
        let calc_mode = get_string_opt("calc_mode")?;
        let parameteri = get_float_opt("parameteri")?;
        let process_label = get_string_opt("process_label")?;
        let tree_type = group.attr_parse_opt("tree_type", &|| err_context("semantic `tree_type` attr"))?;

        Ok(Self {
            name,
            process,
            voltage,
            temperature,
            calc_mode,
            parameteri,
            process_label,
            tree_type,

            attrs: group.attrs,
            groups: group.groups,
        })

    }
}


impl FromLibGroup for LibLuTableTemplate {
    type Error = LibError;

    fn from_group(group: LibGroup) -> Result<Self, Self::Error> {
        let mut group = group;
        let name = group.expect_name().context(|| "group name")?;
        let err_context = |msg: &str| format!("{} in `{}` lu_table_template", msg, &name);

        let variable_1 = group.attr_parse_opt("variable_1", &|| err_context("semantic `variable_1` attr"))?;
        let variable_2 = group.attr_parse_opt("variable_2", &|| err_context("semantic `variable_2` attr"))?;
        let variable_3 = group.attr_parse_opt("variable_3", &|| err_context("semantic `variable_3` attr"))?;

        let index_1 = group.attr_floats_opt("index_1", &|| err_context("semantic `index_1` attr"))?;
        let index_2 = group.attr_floats_opt("index_2", &|| err_context("semantic `index_2` attr"))?;
        let index_3 = group.attr_floats_opt("index_3", &|| err_context("semantic `index_3` attr"))?;

        Ok(Self {
            name, 
            variable_1,
            variable_2,
            variable_3,
            index_1,
            index_2,
            index_3,

            attrs: group.attrs,
            groups: group.groups,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cell_from_group() {
        let mut group = LibGroup::new("cell", vec!["AND2".to_string()]);
        group.groups.push(LibGroup::new("gtype", vec!["gname".to_string()]));
        group.groups.push(LibGroup::new("pin", vec!["a".to_string()]));
        group.groups.push(LibGroup::new("pin", vec!["b".to_string()]));
        let cell = LibCell::from_group(group).unwrap();
        assert_eq!(cell.name, "AND2");
        assert_eq!(cell.groups.len(), 1);
        assert_eq!(cell.pins.len(), 2);
    }

    #[test]
    fn test_library_into_group() {
        let mut lib = LibLibrary::new("my_lib");
        lib.groups.push(LibGroup::new("gtype", vec!["gname".to_string()]));
        lib.cells.push(LibCell::new("AND2"));
        lib.cells.push(LibCell::new("NAND2"));
        // let group = lib.into_group();
        // assert_eq!(group.type_, "library");
        // assert_eq!(group.name, "my_lib");
        // assert_eq!(group.groups.len(), 3);
    }

    // #[test]
    // fn test_lib_from_group() {
    //     let mut group = LibGroup::new("library", vec!["mylib".to_string()]);
    //     group.groups.push(LibGroup::new("gtype", vec!["gname".to_string()]));
    //     let mut cell = LibGroup::new("cell", vec!["AND2".to_string()]);
    //     cell.groups.push(LibGroup::new("pin", vec!["a".to_string()]));
    //     cell.groups.push(LibGroup::new("pin", vec!["b".to_string()]));
    //     group.groups.push(cell);
    //     let lib = LibLibrary::from_group(group).unwrap();
    //     assert_eq!(lib.name, "mylib");
    //     assert_eq!(lib.groups.len(), 1);
    //     assert_eq!(lib.cells.len(), 1);
    //     let converted_cell = lib.cell("AND2").unwrap();
    //     assert_eq!(converted_cell.name, "AND2");
    //     assert_eq!(converted_cell.groups.len(), 0);
    //     assert_eq!(converted_cell.pins.len(), 2);
    // }
}