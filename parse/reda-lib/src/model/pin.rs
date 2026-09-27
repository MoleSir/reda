use reda_lib_macro::model;
use crate::lib_enum;

use super::{LibDontFault, LibExpr};

#[model]
#[derive(Debug, Clone, Default)]
pub struct LibPin {
    pub name: String,

    // Simple attr
    pub alive_during_power_up: Option<bool>,
    pub always_on: Option<bool>,

    pub antenna_diode_type: Option<LibAntennaDiodeType>,
    pub antenna_diode_related_ground_pins: Option<String>,
    pub antenna_diode_related_power_pins: Option<String>,

    pub bit_width: Option<usize>,

    pub capacitance: Option<f64>,
    pub fall_capacitance: Option<f64>,
    pub fanout_load: Option<f64>,
    pub drive_current: Option<f64>,
    pub fall_current_slope_after_threshold: Option<f64>,
    pub fall_current_slope_before_threshold: Option<f64>,
    pub fall_time_after_threshold: Option<f64>,
    pub fall_time_before_threshold: Option<f64>,

    pub max_capacitance: Option<f64>,
    pub min_capacitance: Option<f64>,
    pub max_fanout: Option<f64>,
    pub min_fanout: Option<f64>,
    pub max_transition: Option<f64>,
    pub min_transition: Option<f64>,
    pub min_period: Option<f64>,
    pub min_pulse_width_high: Option<f64>,
    pub min_pulse_width_low: Option<f64>,
    pub max_input_delta_overdrive_high: Option<f64>,
    pub max_input_delta_underdrive_high: Option<f64>,

    pub clamp_0_function: Option<String>,
    pub clamp_1_function: Option<String>,
    pub clamp_latch_function: Option<String>,
    pub clamp_z_function: Option<String>,
    pub function: Option<LibExpr>,
    pub illegal_clamp_condition: Option<String>,
    pub complementary_pin: Option<String>,
    pub nextstate_type: Option<LibNextStateType>,

    pub clock: Option<bool>,
    pub clock_gate_clock_pin: Option<bool>,
    pub clock_gate_enable_pin: Option<bool>,
    pub clock_gate_test_pin: Option<bool>,
    pub clock_gate_obs_pin: Option<bool>,
    pub clock_gate_out_pin: Option<bool>,
    pub clock_isolation_cell_clock_pin: Option<bool>,

    pub direction: LibPinDirection,
    pub driver_type: Option<LibDriverType>,
    pub output_signal_level: Option<String>,
    pub output_signal_level_high: Option<f64>,
    pub output_signal_level_low: Option<f64>,
    pub output_voltage: Option<String>,
    pub input_signal_level: Option<String>,
    pub input_voltage: Option<String>,

    pub has_builtin_pad: Option<bool>,
    pub hysteresis: Option<bool>,
    pub is_pad: Option<bool>,
    pub is_unconnected: Option<bool>,
    pub multicell_pad_pin: Option<bool>,
    pub inverted_output: Option<bool>,

    pub connection_class: Option<String>,
    pub fault_model: Option<String>,
    pub dont_fault: Option<LibDontFault>,
    pub internal_node: Option<String>,
    pub input_map: Option<String>,

    pub pin_func_type: Option<LibPinFuncType>,
    pub primary_output: Option<bool>,
    pub pulling_current: Option<f64>,
    pub pulling_resistance: Option<f64>,
    pub restore_action: Option<LibAction>,
    pub restore_edge_type: Option<LibRestoreEdgeType>,
    pub rise_capacitance: Option<f64>,
    pub rise_current_slope_after_threshold: Option<f64>,
    pub rise_current_slope_before_threshold: Option<f64>,
    pub rise_time_after_threshold: Option<f64>,
    pub rise_time_before_threshold: Option<f64>,
    pub save_action: Option<LibAction>,
    pub signal_type: Option<LibSignalType>,
    pub slew_control: Option<LibSlewControl>,
    pub state_function: Option<LibExpr>,
    pub test_output_only: Option<bool>,
    pub three_state: Option<LibExpr>,
    pub x_function: Option<LibExpr>,

    // Complex attr
    pub fall_capacitance_range: Option<[f64; 2]>,
    pub rise_capacitance_range: Option<[f64; 2]>,

    // Group
    pub timings: Vec<LibTiming>,
}

#[model]
#[derive(Debug, Clone)]
pub struct LibTiming {
    pub related_pin: String,
    pub timing_sense: Option<LibTimingSense>,
    pub timing_type: Option<LibTimingType>,

    pub cell_fall: Option<LibLuTable>,
    pub cell_rise: Option<LibLuTable>,
    pub fall_transition: Option<LibLuTable>,
    pub rise_transition: Option<LibLuTable>,

    pub fall_constraint: Option<LibLuTable>,
    pub rise_constraint: Option<LibLuTable>,
}

#[model]
#[derive(Debug, Clone)]
pub struct LibInternalPower {
    pub related_pin: String,
    pub fall_power: LibLuTable,
    pub rise_power: LibLuTable,
}

#[model]
#[derive(Debug, Clone)]
pub struct LibLuTable {
    pub template_name: String,
    pub index_1: Option<Vec<f64>>,
    pub index_2: Option<Vec<f64>>,
    pub index_3: Option<Vec<f64>>,
    pub values: Vec<Vec<f64>>,
}

lib_enum!(LibTimingSense, positive_unate, negative_unate, non_unate);

lib_enum!(LibTimingType, 
    combinational, combinational_rise,
    combinational_fall, three_state_disable,
    three_state_disable_rise, three_state_disable_fall,
    three_state_enable, three_state_enable_rise,
    three_state_enable_fall,rising_edge, falling_edge,
    preset, clear, hold_rising, hold_falling,
    setup_rising, setup_falling, recovery_rising,
    recovery_falling, skew_rising, skew_falling,
    removal_rising, removal_falling, min_pulse_width,
    minimum_period, max_clock_tree_path,
    min_clock_tree_path,non_seq_setup_rising,
    non_seq_setup_falling, non_seq_hold_rising,
    non_seq_hold_falling, nochange_high_high,
    nochange_high_low,
    nochange_low_high,
    nochange_low_low);

lib_enum!(LibPinDirection, input, output, inout, internal);

lib_enum!(LibAntennaDiodeType, power, ground, power_and_ground);

lib_enum!(LibDriverType,
    pull_up,
    pull_down,
    open_drain,
    open_source,
    bus_hold,
    resistive,
    resistive0,
    resistive1,
);

lib_enum!(LibNextStateType,
    data,
    preset,
    clear,
    load,
    scan_in,
    scan_enable,
);

lib_enum!(LibAction, l, h, r, f);

lib_enum!(LibRestoreEdgeType, edge_trigger, leading, trailing);

lib_enum!(LibSlewControl, low, medium, high, none);

lib_enum!(LibSignalType,
    test_scan_in,
    test_scan_in_inverted,
    test_scan_out,
    test_scan_out_inverted,
    test_scan_enable,
    test_scan_enable_inverted,
    test_scan_clock,
    test_scan_clock_a,
    test_scan_clock_b,
    test_clock,
);

lib_enum!(LibPinFuncType,
    clock_enable,
    active_high,
    active_low,
    active_rising,
    active_falling,
);

impl LibPin {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { 
            name: name.into(),
            ..Default::default()
        }
    }
}