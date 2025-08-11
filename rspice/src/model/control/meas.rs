use runit::{Number, Time};

#[derive(Debug, Clone)]
pub enum MeasureCommand {
    Rise(MeasureRise),
    BasicStat(MeasureBasicStat),
    FindWhen(MeasureFindWhen),
}


/// .MEAS TRAN rise TRIG V(1) VAL=.2 RISE=1
///                 TARG V(1) VAL=.8 RISE=1
#[derive(Debug, Clone)]
pub struct MeasureRise {
    pub name: String,
    pub analysis: AnalysisType,
    pub trig: TrigTargCondition,
    pub targ: TrigTargCondition,
}

/// .MEAS TRAN avgval AVG V(1) FROM=10ns TO=55ns
#[derive(Debug, Clone)]
pub struct MeasureBasicStat {
    pub name: String,
    pub analysis: AnalysisType,
    pub stat: MeasureFunction,
    pub variable: OutputVariable,
    pub from: Time,
    pub to: Time,
}

/// .MEAS TRAN DesiredCurr FIND I(Vmeas) WHEN V(1)=1V
#[derive(Debug, Clone)]
pub struct MeasureFindWhen {
    pub name: String,
    pub analysis: AnalysisType,
    pub variable: OutputVariable,
    pub when: FindWhenCondition,
}

#[derive(Debug, Clone)]
pub struct TrigTargCondition {
    pub variable: OutputVariable,
    pub value: Number,
    pub edge: EdgeType, // RISE or FALL
    pub number: usize,  // 第几个上升沿/下降沿
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeType {
    Rise,
    Fall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeasureFunction {
    Avg,
    Rms,
    Min,
    Max,
    Pp,      // Peak to Peak
    Deriv,
    Integrate,
}

#[derive(Debug, Clone)]
pub struct FindWhenCondition {
    pub variable: OutputVariable,
    pub value: Number,
}

#[derive(Debug, Clone)]
pub struct ExpressionCondition {
    pub variable: OutputVariable,
    pub expression: String, // 如 "0.9*vdd"
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalysisType {
    Dc,
    Ac,
    Tran,
}

#[derive(Debug, Clone)]
pub enum OutputVariable {
    Voltage {
        node1: String,              // 例如 "n1"
        node2: Option<String>,      // None 表示对地，Some("n2") 表示 V(n1,n2)
        suffix: Option<OutputSuffix>,
    },
    Current {
        element_name: String,       // 例如 "V1" 表示电压源 V1 中的电流
        suffix: Option<OutputSuffix>,
    },
}

#[derive(Debug, Clone)]
pub enum OutputSuffix {
    Magnitude,
    Decibel,
    Phase,
    Real,
    Imag,
}
