use reda_lib_macro::model;
use crate::{lib_enum, model::LibCell};

use super::{LibCapacitiveLoadUnit, LibCurrentUnit, LibLeakagePowerUnit, LibPullingResistanceUnit, LibTimeUnit, LibVoltageUnit};

#[model]
#[derive(Debug, Clone, Default)]
pub struct LibLibrary {
    pub name: String,

    // documentation attributes
    pub date: String,
    pub revision: Option<String>,
    pub comment: Option<String>,

    // general attributes
    pub delay_model: LibDelayModel,

    // units attributes
    pub time_unit: LibTimeUnit,
    pub voltage_unit: LibVoltageUnit,
    pub capacitive_load_unit: Option<LibCapacitiveLoadUnit>,
    pub current_unit: Option<LibCurrentUnit>,
    pub pulling_resistance_unit: Option<LibPullingResistanceUnit>,
    pub leakage_power_unit: Option<LibLeakagePowerUnit>,

    // operation conditions
    pub nom_process: Option<f64>,
    pub nom_temperature: Option<f64>,
    pub nom_voltage: Option<f64>,
    pub default_operating_conditions: Option<String>,

    //  threshold definitions
    pub slew_lower_threshold_pct_fall: f64,
    pub slew_lower_threshold_pct_rise: f64,
    pub slew_upper_threshold_pct_fall: f64,
    pub slew_upper_threshold_pct_rise: f64,
    pub slew_derate_from_library: f64,
    pub input_threshold_pct_fall: f64,
    pub input_threshold_pct_rise: f64,
    pub output_threshold_pct_fall: f64,
    pub output_threshold_pct_rise: f64,
    pub default_leakage_power_density: f64,
    pub default_cell_leakage_power: f64,

    // default pin attributes
    pub default_inout_pin_cap: Option<f64>,
    pub default_input_pin_cap: Option<f64>,
    pub default_output_pin_cap: Option<f64>,
    pub default_fanout_load: Option<f64>,
    pub default_max_transition: Option<f64>,

    // groups
    pub cells: Vec<LibCell>,
    pub operating_conditions: Vec<LibOperatingConditions>,
    pub lu_table_templates: Vec<LibLuTableTemplate>,
}

#[model]
#[derive(Debug, Clone, Default)]
pub struct LibLuTableTemplate {
    pub name: String,
    pub variable_1: Option<LibLuTableTemplateVariable>,
    pub variable_2: Option<LibLuTableTemplateVariable>,
    pub variable_3: Option<LibLuTableTemplateVariable>,
    pub index_1: Option<Vec<f64>>,
    pub index_2: Option<Vec<f64>>,
    pub index_3: Option<Vec<f64>>,
}

impl LibLibrary {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { 
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn cell(&self, name: &str) -> Option<&LibCell> {
        self.cells.iter().find(|c| c.name == name)
    }
}

#[model]
#[derive(Debug, Clone)]
pub struct LibOperatingConditions {
    pub name: String,
    pub process: f64,
    pub voltage: f64,
    pub temperature: f64,
    pub calc_mode: Option<String>,
    pub parameteri: Option<f64>,
    pub process_label: Option<String>,
    pub tree_type: Option<LibTreeType>,
}

lib_enum!(LibTechnology, coms, fpga);
lib_enum!(LibDelayModel, table_lookup, polynomial);
lib_enum!(LibTreeType, best_case_tree, balanced_tree, worst_case_tree);
lib_enum!(LibLuTableTemplateVariable, 
    input_net_transition, 
    total_output_net_capacitance,
    output_net_length, 
    output_net_wire_cap,
    output_net_pin_cap,
    related_out_total_output_net_capacitance,
    related_out_output_net_length,
    related_out_output_net_wire_cap,
    related_out_output_net_pin_cap,
    constrained_pin_transition,
    related_pin_transition
);