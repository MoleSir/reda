use std::collections::HashMap;
use num_complex::Complex64;
use super::api::VecInfoAll;

#[allow(unused)]
pub trait NgSpiceSharedCallback {
    /// NgSpice 向用户输出普通消息（stdout）
    fn send_char(&self, message: &str, ngspice_id: i32) -> i32;

    /// 输出状态信息（通常是仿真状态）
    fn send_stat(&self, message: &str, ngspice_id: i32) -> i32;

    /// 仿真运行过程中，每步传输数据时调用
    fn send_data(&self, actual_vector_values: HashMap<String, Complex64>, number_of_vectors: i32, ngspice_id: i32) -> i32;

    /// 仿真开始时传送初始变量信息
    fn send_init_data(&self, data: &VecInfoAll, ngspice_id: i32) -> i32;

    /// 同步接口：获取电压源值（如 PWL 定义的电源）
    fn get_vsrc_data(&self, voltage: &mut f64, time: f64, node: String, ngspice_id: i32) -> i32;
    /// 同步接口：获取电流源值（如 PWL 定义的电流源）
    fn get_isrc_data(&self, current: &mut f64, time: f64, node: String, ngspice_id: i32) -> i32;
}

pub struct DefaultNgSpiceSharedCallback;

#[allow(unused)]
impl NgSpiceSharedCallback for DefaultNgSpiceSharedCallback {
    fn send_char(&self, message: &str, ngspice_id: i32) -> i32 {
        // println!("Send char from '{}': {}", ngspice_id, message);
        0
    }

    fn send_stat(&self, message: &str, ngspice_id: i32) -> i32 {
        // println!("Send stat from '{}': {}", ngspice_id, message);
        0
    }

    fn send_data(&self, actual_vector_values: HashMap<String, Complex64>, number_of_vectors: i32, ngspice_id: i32) -> i32 {
        // println!("Sen data from '{}'", ngspice_id);
        0
    }

    fn send_init_data(&self, data: &VecInfoAll, ngspice_id: i32) -> i32 {
        // println!("Sen init data from '{}'", ngspice_id);
        0
    }

    fn get_vsrc_data(&self, voltage: &mut f64, time: f64, node: String, ngspice_id: i32) -> i32 {
        // println!("Sen init data from '{}'", ngspice_id);
        0
    }
    
    fn get_isrc_data(&self, current: &mut f64, time: f64, node: String, ngspice_id: i32) -> i32 {
        // println!("Sen init data from '{}'", ngspice_id);
        0
    }
}