use runit::{Angle, Current, Voltage};


#[derive(Debug, Clone)]
pub struct AcCurrent {
    pub magnitude: Current, 
    pub phase_deg: Angle,
}

#[derive(Debug, Clone)]
pub struct AcVoltage {
    pub magnitude: Voltage, 
    pub phase_deg: Angle,
}

impl AcVoltage {
    pub fn to_spice(&self) -> String {
        format!(
            "AC {} {}",
            self.magnitude,
            self.phase_deg
        )
    }
}

impl AcCurrent {
    pub fn to_spice(&self) -> String {
        format!(
            "AC {} {}",
            self.magnitude,
            self.phase_deg
        )
    }
}