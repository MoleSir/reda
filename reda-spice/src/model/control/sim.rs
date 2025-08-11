use derive_builder::Builder;
use reda_unit::{Frequency, Time, Voltage};

use crate::ToSpice;

#[derive(Debug, Clone)]
pub enum SimCommand {
    Dc(DcCommand),
    Ac(AcCommand),
    Tran(TranCommand)
}

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option, into))]
pub struct DcCommand {
    pub src_name: String,
    pub start: Voltage,
    pub stop: Voltage,
    pub step: Voltage,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum AcSweepType {
    Lin, // 线性扫描
    Dec, // 每十倍频率 ND 点
    Oct, // 每倍频率 NO 点
}

#[derive(Debug, Clone)]
pub struct AcCommand {
    pub sweep_type: AcSweepType,
    pub points: usize,
    pub f_start: Frequency,
    pub f_stop: Frequency,
}

impl AcCommand {
    pub fn linear<F1: Into<Frequency>, F2: Into<Frequency>>(points: usize, f_start: F1, f_stop: F2) -> Self {
        Self::new(AcSweepType::Lin, points, f_start, f_stop)
        
    }

    pub fn dec<F1: Into<Frequency>, F2: Into<Frequency>>(points: usize, f_start: F1, f_stop: F2) -> Self {
        Self::new(AcSweepType::Dec, points, f_start, f_stop)

    }

    pub fn oct<F1: Into<Frequency>, F2: Into<Frequency>>(points: usize, f_start: F1, f_stop: F2) -> Self {
        Self::new(AcSweepType::Oct, points, f_start, f_stop)

    }

    pub fn new<F1: Into<Frequency>, F2: Into<Frequency>>(ty: AcSweepType, points: usize, f_start: F1, f_stop: F2) -> Self {
        Self {
            sweep_type: ty,
            points,
            f_start: f_start.into(),
            f_stop: f_stop.into(),
        }
    }
}

#[derive(Debug, Clone, Builder)]
#[builder(setter(into, strip_option))]
pub struct TranCommand {
    pub t_step: Time,
    pub t_stop: Time,
    #[builder(default)]
    pub t_start: Option<Time>,
    #[builder(default)]
    pub t_max: Option<Time>,
    #[builder(default = "false")]
    pub uic: bool, // Use Initial Condition
}

impl ToSpice for SimCommand {
    fn to_spice(&self) -> String {
        match self {
            SimCommand::Dc(dc) => dc.to_spice(),
            SimCommand::Ac(ac) => ac.to_spice(),
            SimCommand::Tran(tran) => tran.to_spice(),
        }
    }
}

impl ToSpice for DcCommand {
    fn to_spice(&self) -> String {
        format!(
            ".DC {} {} {} {}",
            self.src_name, self.start, self.stop, self.step
        )
    }
}

impl ToSpice for AcCommand {
    fn to_spice(&self) -> String {
        format!(
            ".AC {} {} {} {}",
            self.sweep_type.to_spice_str(),
            self.points,
            self.f_start.to_spice(),
            self.f_stop.to_spice(),
        )
    }
}

impl ToSpice for TranCommand {
    fn to_spice(&self) -> String {
        let mut line = format!(".TRAN {} {}", self.t_step, self.t_stop);

        if let Some(t_start) = self.t_start {
            line.push_str(&format!(" {}", t_start));
        }

        if let Some(t_max) = self.t_max {
            if self.t_start.is_none() {
                line.push_str(" 0");
            }
            line.push_str(&format!(" {}", t_max));
        }

        if self.uic {
            line.push_str(" UIC");
        }

        line
    }
}

impl AcSweepType {
    pub fn to_spice_str(&self) -> &'static str {
        match self {
            AcSweepType::Lin => "LIN",
            AcSweepType::Dec => "DEC",
            AcSweepType::Oct => "OCT",
        }
    }
}
