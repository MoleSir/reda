use reda_lib_macro::model;
use crate::{lib_enum, model::LibPin};

use super::{LibExpr, LibPinDirection};

#[model]
#[derive(Debug, Clone, Default)]
pub struct LibCell {
    pub name: String,

    pub area: Option<f64>,
    pub drive_strength: Option<usize>,
    pub cell_leakage_power: Option<f64>,
    pub ff: Option<LibFF>,

    pub leakage_powers: Vec<LibLeakagePower>,
    pub pins: Vec<LibPin>,
    pub pg_pins: Vec<LibPgPin>,
}

#[model]
#[derive(Debug, Clone, Default)]
pub struct LibPgPin {
    pub name: String,
    pub voltage_name: String,
    pub pg_type: LibPgType,  
    pub user_pg_type: Option<String>,
    pub physical_connection: Option<LibPhysicalConnection>,
}

#[model]
#[derive(Debug, Clone, Default)]
pub struct LibFF {
    pub names: Vec<String>,
    pub next_state: Option<LibExpr>,
    pub clocked_on: Option<LibExpr>,
    pub clear: Option<LibExpr>,
    pub clocked_on_also: Option<LibExpr>,
    pub preset: Option<LibExpr>,
    pub clear_preset_var1: Option<LibClearPresetVar>,
    pub clear_preset_var2: Option<LibClearPresetVar>,
}

lib_enum!(LibDontFault, 
    Sa0,
    Sa1,
    Sa01,
);

lib_enum!(LibFpgaCellType, 
    Sa0,
    Sa1,
    Sa01,
);

lib_enum!(LibClearPresetVar,
    L, H, N, T, X
);

lib_enum!(LibPgType, 
    primary_power, 
    primary_ground, 
    backup_power, 
    backup_ground, 
    internal_power, 
    internal_ground, 
    pwell, 
    nwell, 
    deepnwell, 
    deeppwell
);

lib_enum!(LibPhysicalConnection,
    device_layer,
    routing_pin,
);

#[model]
#[derive(Debug, Clone)]
pub struct LibLeakagePower {
    pub when: LibExpr,
    pub value: f64,
}

impl LibCell {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { 
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn pin(&self, name: &str) -> Option<&LibPin> {
        self.pins.iter().find(|p| p.name == name)
    }

    pub fn input_pins(&self) -> impl Iterator<Item = &LibPin> {
        self.pins.iter()
            .filter(|pin| {
                pin.direction == LibPinDirection::Input
            })
    } 

    pub fn output_pins(&self) -> impl Iterator<Item = &LibPin> {
        self.pins.iter()
            .filter(|pin| {
                pin.direction == LibPinDirection::Output
            })
    }

    pub fn get_pin(&self, name: &str) -> Option<&LibPin> {
        self.pins.iter()
            .find(|pin| pin.name == name)
    }

    pub fn get_pg_pin(&self, name: &str) -> Option<&LibPgPin> {
        self.pg_pins.iter()
            .find(|pin| pin.name == name)
    }
}