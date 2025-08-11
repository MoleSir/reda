mod op;
mod dc;
mod ac;
mod tran;
mod error;

pub use op::*;
pub use dc::*;
pub use tran::*;
pub use ac::*;
pub use error::*;

// #[derive(Debug, Clone)]
// pub enum Analysis {
//     Op(OpAnalysis),
//     Dc(DcAnalysis),
//     Ac(AcAnalysis),
//     Tran(TranAnalysis),
// }

pub trait ToAnalysis {
    type Err;
    fn to_tran_analysis(&self) -> Result<TranAnalysis, Self::Err>;
    fn to_dc_voltage_analysis(&self) -> Result<DcVoltageAnalysis, Self::Err>;
    fn to_op_analysis(&self) -> Result<OpAnalysis, Self::Err>;
    fn to_ac_analysis(&self) -> Result<AcAnalysis, Self::Err>;
}
