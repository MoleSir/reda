use crate::error::{LibErrorContext, LibError};
use crate::model::{LibCell, LibFF, LibGroup, LibLeakagePower, LibPgPin, LibPin};
use super::FromLibGroup;

impl FromLibGroup for LibCell {
    type Error = LibError;
    fn from_group(group: LibGroup) -> Result<Self, LibError> {
        let mut group = group;
        let name = group.expect_name().context(|| "cell name")?;
        let err_context = |msg: &str| {
            format!("{} in {} cell", msg, &name)
        };

        let area = group.attr_float_opt("area", || err_context("semantic `area` attr"))?;
        let drive_strength = group.attr_float_opt("drive_strength", || err_context("semantic `drive_strength` attr"))?.map(|f| f as usize);
        let cell_leakage_power = group.attr_float_opt("cell_leakage_power", || err_context("semantic `cell_leakage_power` attr"))?;

        let mut pins = vec![];
        let mut pg_pins = vec![];
        let mut leakage_powers = vec![];
        let mut groups = vec![];
        let mut ff = None;

        for group in group.groups {
            match group.kind.as_str() {
                "pin" => pins.push(LibPin::from_group(group).context(|| err_context("semantic pin"))?),
                "pg_pin" => pg_pins.push(LibPgPin::from_group(group).context(|| err_context("semantic pg_pin"))?),
                "leakage_power" => leakage_powers.push(LibLeakagePower::from_group(group).context(|| err_context("semantic leakage_power"))?),
                "ff" => if ff.is_some() {
                    Err(LibError::NotOnlyFF).context(|| err_context("semantic ff"))?;
                } else {
                    ff = Some(LibFF::from_group(group).context(|| err_context("semantic ff"))?)
                },
                _ => groups.push(group),
            }
        }

        Ok(Self {
            name,
            area,
            drive_strength,
            cell_leakage_power,
            pins,
            pg_pins,
            leakage_powers,
            attrs: group.attrs,
            groups,
            ff,
        })
    }
}

impl FromLibGroup for LibPgPin {
    type Error = LibError;

    fn from_group(group: LibGroup) -> Result<Self, Self::Error> {
        let mut group = group;
        let name = group.expect_name().context(|| "cell name")?;
        let err_context = |msg: &str| format!("{} in `{}` pg_pin", msg, &name);

        Ok(Self {
            name: name.clone(),
            voltage_name: group.attr_lliteral("voltage_name", || err_context("semantic `voltage_name` attr"))?.to_string(),
            pg_type: group.attr_parse("pg_type", &|| err_context("semantic `pg_type` attr"))?,
            user_pg_type: group.attr_string_opt("user_pg_type", || err_context("semantic `user_pg_type` attr"))?.map(String::from),
            physical_connection: group.attr_parse_opt("physical_connection", &|| err_context("semantic `physical_connection` attr"))?,

            attrs: group.attrs,
            groups: group.groups,
        })
    }
}

impl FromLibGroup for LibLeakagePower {
    type Error = LibError;

    fn from_group(group: LibGroup) -> Result<Self, Self::Error> {
        let err_context = |msg: &str| format!("{} in leakage_power", msg);

        Ok(Self {
            when: group.attr_expr("when", &|| err_context("semantic `when` attr"))?,
            value: group.attr_float("value", &|| err_context("semantic `value` attr"))?,
            attrs: group.attrs,
            groups: group.groups,
        })
    }
}

impl FromLibGroup for LibFF {
    type Error = LibError;

    fn from_group(group: LibGroup) -> Result<Self, Self::Error> {
        let err_context = |msg: &str| format!("{} in ff", msg);
        let next_state = group.attr_expr_opt("next_state", &|| err_context("semantic `next_state` attr"))?;
        let clear = group.attr_expr_opt("clear", &|| err_context("semantic `clear` attr"))?;
        let clocked_on = group.attr_expr_opt("clocked_on", &|| err_context("semantic `clocked_on` attr"))?;
        let clocked_on_also = group.attr_expr_opt("clocked_on_also", &|| err_context("semantic `clocked_on_also` attr"))?;
        let preset = group.attr_expr_opt("preset", &|| err_context("semantic `preset` attr"))?;
        let clear_preset_var1 = group.attr_parse_opt("clear_preset_var1", &|| err_context("semantic `clear_preset_var1` attr"))?;
        let clear_preset_var2 = group.attr_parse_opt("clear_preset_var2", &|| err_context("semantic `clear_preset_var2` attr"))?;

        Ok(Self {
            names: group.params,
            next_state,
            clocked_on,
            clear,
            clocked_on_also,
            preset,
            clear_preset_var1,
            clear_preset_var2,

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

    #[test]
    fn test_cell_into_group() {
        let mut cell = LibCell::new("my_cell");
        cell.groups.push(LibGroup::new("gtype", vec!["gname".to_string()]));
        cell.pins.push(LibPin::new("a"));
        cell.pins.push(LibPin::new("b"));
        // let group = cell.into_group();
        // assert_eq!(group.type_, "cell");
        // assert_eq!(group.name, "my_cell");
        // assert_eq!(group.groups.len(), 3);
    }

}