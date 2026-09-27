use crate::error::{LibErrorContext, LibError};
use crate::model::{LibGroup, LibLuTable, LibPin, LibPinDirection, LibTiming};

use super::FromLibGroup;

impl FromLibGroup for LibPin {
    type Error = LibError;

    fn from_group(group: LibGroup) -> Result<Self, LibError> {
        let mut group = group;
        let name = group.expect_name().context(|| "pin name")?;
        let err_context = |msg: &str| format!("{} in `{}` pin", msg, &name);

        // helpers that attach context when calling LibValue accessors
        let get_bool_opt = 
            |key: &str| group.attr_bool_opt(key, || err_context(&format!("semantic `{}` attr", key)));
        let get_float_opt = 
            |key: &str| group.attr_float_opt(key, || err_context(&format!("semantic `{}` attr", key)));
        let get_string_opt = 
            |key: &str| group.attr_string_opt(key, || err_context(&format!("semantic `{}` attr", key))).map(|v| v.map(String::from));
        let get_ident_opt = 
            |key: &str| group.attr_ident_opt(key, || err_context(&format!("semantic `{}` attr", key))).map(|v| v.map(String::from));
        let get_expr_opt = 
            |key: &str| group.attr_expr_opt(key, &|| err_context(&format!("semantic `{}` attr", key)));


        let alive_during_power_up = get_bool_opt("alive_during_power_up")?;
        let always_on = get_bool_opt("always_on")?;

        let antenna_diode_type = group.attr_parse_opt("antenna_diode_type", &|| err_context("semantic `antenna_diode_type` attr"))?;
        let nextstate_type = group.attr_parse_opt("nextstate_type", &|| err_context("semantic `nextstate_type` attr"))?;

        let antenna_diode_related_ground_pins = get_string_opt("antenna_diode_related_ground_pins")?;
        let antenna_diode_related_power_pins = get_string_opt("antenna_diode_related_power_pins")?;
        let bit_width = get_float_opt("bit_width")?.map(|v| v as usize);

        let capacitance = get_float_opt("capacitance")?;
        let fall_capacitance = get_float_opt("fall_capacitance")?;
        let fanout_load = get_float_opt("fanout_load")?;
        let drive_current = get_float_opt("drive_current")?;
        let fall_current_slope_after_threshold = get_float_opt("fall_current_slope_after_threshold")?;
        let fall_current_slope_before_threshold = get_float_opt("fall_current_slope_before_threshold")?;
        let fall_time_after_threshold = get_float_opt("fall_time_after_threshold")?;
        let fall_time_before_threshold = get_float_opt("fall_time_before_threshold")?;

        let max_capacitance = get_float_opt("max_capacitance")?;
        let min_capacitance = get_float_opt("min_capacitance")?;
        let max_fanout = get_float_opt("max_fanout")?;
        let min_fanout = get_float_opt("min_fanout")?;
        let max_transition = get_float_opt("max_transition")?;
        let min_transition = get_float_opt("min_transition")?;
        let min_period = get_float_opt("min_period")?;
        let min_pulse_width_high = get_float_opt("min_pulse_width_high")?;
        let min_pulse_width_low = get_float_opt("min_pulse_width_low")?;
        let max_input_delta_overdrive_high = get_float_opt("max_input_delta_overdrive_high")?;
        let max_input_delta_underdrive_high = get_float_opt("max_input_delta_underdrive_high")?;

        let clamp_0_function = get_string_opt("clamp_0_function")?;
        let clamp_1_function = get_string_opt("clamp_1_function")?;
        let clamp_latch_function = get_string_opt("clamp_latch_function")?;
        let clamp_z_function = get_string_opt("clamp_z_function")?;
        let illegal_clamp_condition = get_string_opt("illegal_clamp_condition")?;
        let complementary_pin = get_string_opt("complementary_pin")?;

        let clock = get_bool_opt("clock")?;
        let clock_gate_clock_pin = get_bool_opt("clock_gate_clock_pin")?;
        let clock_gate_enable_pin = get_bool_opt("clock_gate_enable_pin")?;
        let clock_gate_test_pin = get_bool_opt("clock_gate_test_pin")?;
        let clock_gate_obs_pin = get_bool_opt("clock_gate_obs_pin")?;
        let clock_gate_out_pin = get_bool_opt("clock_gate_out_pin")?;
        let clock_isolation_cell_clock_pin = get_bool_opt("clock_isolation_cell_clock_pin")?;

        let direction = group.attr_parse_opt("direction", &|| err_context("semantic `direction` attr"))?.unwrap_or(LibPinDirection::Input);

        let driver_type = group.attr_parse_opt("driver_type", &|| err_context("semantic `driver_type` attr"))?;

        let output_signal_level = get_string_opt("output_signal_level")?;
        let output_signal_level_high = get_float_opt("output_signal_level_high")?;
        let output_signal_level_low = get_float_opt("output_signal_level_low")?;
        let output_voltage = get_string_opt("output_voltage")?;
        let input_signal_level = get_string_opt("input_signal_level")?;
        let input_voltage = get_string_opt("input_voltage")?;

        let has_builtin_pad = get_bool_opt("has_builtin_pad")?;
        let hysteresis = get_bool_opt("hysteresis")?;
        let is_pad = get_bool_opt("is_pad")?;
        let is_unconnected = get_bool_opt("is_unconnected")?;
        let multicell_pad_pin = get_bool_opt("multicell_pad_pin")?;
        let inverted_output = get_bool_opt("inverted_output")?;

        let connection_class = get_string_opt("connection_class")?;
        let fault_model = get_string_opt("fault_model")?;
        let dont_fault = group.attr_parse_opt("dont_fault", &|| err_context("semantic `dont_fault` attr"))?;
        let internal_node = get_ident_opt("internal_node")?;
        let input_map = get_string_opt("input_map")?;

        let function = get_expr_opt("function")?;

        // enums via attr_parse_opt (uses FromStr impls above)
        let pin_func_type = group.attr_parse_opt("pin_func_type", &|| err_context("semantic `pin_func_type` attr"))?;
        let primary_output = get_bool_opt("primary_output")?;
        let pulling_current = get_float_opt("pulling_current")?;
        let pulling_resistance = get_float_opt("pulling_resistance")?;
        let restore_action = group.attr_parse_opt("restore_action", &|| err_context("semantic `restore_action` attr"))?;
        let restore_edge_type = group.attr_parse_opt("restore_edge_type", &|| err_context("semantic `restore_edge_type` attr"))?;

        // rise* (floats)
        let rise_capacitance = get_float_opt("rise_capacitance")?;
        let rise_current_slope_after_threshold = get_float_opt("rise_current_slope_after_threshold")?;
        let rise_current_slope_before_threshold = get_float_opt("rise_current_slope_before_threshold")?;
        let rise_time_after_threshold = get_float_opt("rise_time_after_threshold")?;
        let rise_time_before_threshold = get_float_opt("rise_time_before_threshold")?;

        let save_action = group.attr_parse_opt("save_action", &|| err_context("semantic `save_action` attr"))?;
        let signal_type = group.attr_parse_opt("signal_type", &|| err_context("semantic `signal_type` attr"))?;
        let slew_control = group.attr_parse_opt("slew_control", &|| err_context("semantic `slew_control` attr"))?;

        // boolean / expr fields
        let state_function = get_expr_opt("state_function")?;
        let test_output_only = get_bool_opt("test_output_only")?;
        let three_state = get_expr_opt("three_state")?;
        let x_function = get_expr_opt("x_function")?;

        let fall_capacitance_range = group.attr_floats_arr_opt("fall_capacitance_range", &|| err_context("semantic `fall_capacitance_range` attr"))?;
        let rise_capacitance_range = group.attr_floats_arr_opt("rise_capacitance_range", &|| err_context("semantic `rise_capacitance_range` attr"))?;

        let mut timings = vec![];
        let mut groups = vec![];

        for group in group.groups {
            match group.kind.as_str() {
                "timing" => timings.push(LibTiming::from_group(group).context(|| err_context("semantic timing"))?),
                _ => groups.push(group),
            }
        }

        Ok(Self {
            name,

            alive_during_power_up,
            always_on,

            antenna_diode_type,
            antenna_diode_related_ground_pins,
            antenna_diode_related_power_pins,

            bit_width,

            capacitance,
            fall_capacitance,
            fanout_load,
            drive_current,
            fall_current_slope_after_threshold,
            fall_current_slope_before_threshold,
            fall_time_after_threshold,
            fall_time_before_threshold,

            max_capacitance,
            min_capacitance,
            max_fanout,
            min_fanout,
            max_transition,
            min_transition,
            min_period,
            min_pulse_width_high,
            min_pulse_width_low,
            max_input_delta_overdrive_high,
            max_input_delta_underdrive_high,

            clamp_0_function,
            clamp_1_function,
            clamp_latch_function,
            clamp_z_function,
            illegal_clamp_condition,
            complementary_pin,
            nextstate_type,

            clock,
            clock_gate_clock_pin,
            clock_gate_enable_pin,
            clock_gate_test_pin,
            clock_gate_obs_pin,
            clock_gate_out_pin,
            clock_isolation_cell_clock_pin,

            direction,
            driver_type,
            output_signal_level,
            output_signal_level_high,
            output_signal_level_low,
            output_voltage,
            input_signal_level,
            input_voltage,

            has_builtin_pad,
            hysteresis,
            is_pad,
            is_unconnected,
            multicell_pad_pin,
            inverted_output,

            connection_class,
            fault_model,
            dont_fault,
            internal_node,
            input_map,

            function,

            pin_func_type,
            primary_output,
            pulling_current,
            pulling_resistance,
            restore_action,
            restore_edge_type,
            rise_capacitance,
            rise_current_slope_after_threshold,
            rise_current_slope_before_threshold,
            rise_time_after_threshold,
            rise_time_before_threshold,
            save_action,
            signal_type,
            slew_control,
            state_function,
            test_output_only,
            three_state,
            x_function,

            fall_capacitance_range,
            rise_capacitance_range,

            timings,
            attrs: group.attrs,
            groups: groups,
        })
    }
}

impl FromLibGroup for LibTiming {
    type Error = LibError;

    fn from_group(group: LibGroup) -> Result<Self, LibError> {
        let err_context = |msg: &str| format!("{} in timing", msg);

        let related_pin = group.attr_string("related_pin", || err_context("semantic `related_pin` attr"))?.to_string();
        let timing_sense = group.attr_parse_opt("timing_sense", &|| err_context("semantic `timing_sense` attr"))?;
        let timing_type = group.attr_parse_opt("timing_type", &|| err_context("semantic `timing_type` attr"))?;

        let mut cell_fall = None;
        let mut cell_rise = None;
        let mut fall_transition = None;
        let mut rise_transition = None;
        let mut fall_constraint = None;
        let mut rise_constraint = None;

        let mut groups = vec![];

        for group in group.groups {
            match group.kind.as_str() {
                "cell_fall" => cell_fall = Some(LibLuTable::from_group(group).context(|| err_context("semantic cell_fall"))?),
                "cell_rise" => cell_rise = Some(LibLuTable::from_group(group).context(|| err_context("semantic cell_rise"))?),
                "fall_transition" => fall_transition = Some(LibLuTable::from_group(group).context(|| err_context("semantic fall_transition"))?),
                "rise_transition" => rise_transition = Some(LibLuTable::from_group(group).context(|| err_context("semantic rise_transition"))?),
                "fall_constraint" => fall_constraint = Some(LibLuTable::from_group(group).context(|| err_context("semantic fall_constraint"))?),
                "rise_constraint" => rise_constraint = Some(LibLuTable::from_group(group).context(|| err_context("semantic rise_constraint"))?),
                _ => groups.push(group),
            }
        }

        Ok(Self {
            related_pin,
            timing_sense,
            timing_type,

            cell_fall,
            cell_rise,
            fall_transition,
            rise_transition,
            fall_constraint,
            rise_constraint,

            attrs: group.attrs,
            groups: groups,
        })
    }
}

impl FromLibGroup for LibLuTable {
    type Error = LibError;

    fn from_group(group: LibGroup) -> Result<Self, LibError> {
        let mut group = group;
        let template_name = group.expect_name().context(|| "pin name")?;
        let err_context = |msg: &str| format!("{} in template `{}` lut", msg, &template_name);

        let index_1 = group.attr_floats_opt("index_1", &|| err_context("semantic `index_1` attr"))?;
        let index_2 = group.attr_floats_opt("index_2", &|| err_context("semantic `index_2` attr"))?;
        let index_3 = group.attr_floats_opt("index_3", &|| err_context("semantic `index_3` attr"))?;

        let mut values = vec![];
        let attrs = group.attr_vector("values", &|| err_context("semantic `values` attr"))?;
        for attr in attrs.iter() {
            let floats = attr.floats().context(|| err_context("semantic `values` attr"))?;
            values.push(floats);
        }

        Ok(Self {
            template_name, 

            index_1,
            index_2,
            index_3,

            values,

            attrs: group.attrs,
            groups: group.groups,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pin_into_group() {
        let mut pin = LibPin::new("my_pin");
        pin.groups.push(LibGroup::new("gtype", vec!["gname".to_string()]));
        // let group = pin.into_group();
        // assert_eq!(group.type_, "pin");
        // assert_eq!(group.name, "my_pin");
        // assert_eq!(group.groups.len(), 1);
    }

    #[test]
    fn test_pin_from_group() {
        let mut group = LibGroup::new("pin", vec!["a".to_string()]);
        group.groups.push(LibGroup::new("gtype", vec!["gname".to_string()]));
        let pin = LibPin::from_group(group).unwrap();
        assert_eq!(pin.name, "a");
        assert_eq!(pin.groups.len(), 1);
    }
}