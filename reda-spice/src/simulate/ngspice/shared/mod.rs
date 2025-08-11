mod api;
mod plot;
mod callback;

use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::LazyLock;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::ffi::{CStr, CString, c_char, c_double, c_int, c_void};
use api::{NgSpiceAPI, VecData, VecInfoAll, VecValuesAll};
use libloading::Library;
use num_complex::Complex64;
use plot::Plot;
use regex::Regex;
use callback::{DefaultNgSpiceSharedCallback, NgSpiceSharedCallback};
use crate::probe::{AcAnalysis, DcVoltageAnalysis, OpAnalysis, ToAnalysis, TranAnalysis};
use crate::simulate::Simulate;
use crate::Value;

use super::error::*;

#[cfg(unix)]
use libc::setlocale;
#[cfg(unix)]
use libc::LC_NUMERIC;

static NGSPICE_ID: LazyLock<AtomicI32> = LazyLock::new(|| AtomicI32::new(0));
fn next_count() -> i32 {
    NGSPICE_ID.fetch_add(1, Ordering::SeqCst)
}

pub struct NgSpiceShared {
    ngspice_id: i32,
    pub api: NgSpiceAPI,
    callback: Box<dyn NgSpiceSharedCallback>,
    library_path: PathBuf,

    stdout: Vec<String>,
    stderr: Vec<String>,
    error_in_stdout: bool,
    error_in_stderr: bool,
    spinit_not_found: bool,
    is_running: bool,

    ngspice_version: Option<u32>,
    has_xspice: bool,
    has_cider: bool,
    extensions: Vec<String>,
}


impl NgSpiceShared {
    pub fn default() -> NgSpiceResult<Self> {
        Self::new_with_callback(None, DefaultNgSpiceSharedCallback)
    }

    pub fn new(library_path: PathBuf) -> NgSpiceResult<Self> {
        Self::new_with_callback(Some(library_path), DefaultNgSpiceSharedCallback)
    }

    pub fn new_with_callback(
        library_path: Option<PathBuf>, 
        callback: impl NgSpiceSharedCallback + 'static
    ) -> NgSpiceResult<Self> {
        let ngspice_id = next_count();
    
        let library_path = match library_path {
            Some(p) => p,
            None => Self::setup_platform()?,
        };
        
        let lib = Self::load_library(&library_path)?;
        let api = NgSpiceAPI::new(lib);

        Ok(NgSpiceShared {
            library_path,
            callback: Box::new(callback),
            api,
            stdout: vec![],
            stderr: vec![],
            error_in_stdout: false,
            error_in_stderr: false,
            spinit_not_found: false,
            is_running: false,
            ngspice_id,
            ngspice_version: None,
            has_xspice: false,
            has_cider: false,
            extensions: Vec::new(),
        })
    }

    fn setup_platform() -> NgSpiceResult<PathBuf> {
        if let Ok(path) = env::var("NGSPICE_LIBRARY_PATH") {
            return Ok(PathBuf::from(path))
        }

        #[cfg(target_os = "linux")] {
            return Ok(PathBuf::from("libngspice.so"));
        }

        #[allow(unreachable_code)]
        Err(NgSpiceError::Platform)
    }

    fn load_library<P: AsRef<Path>>(library_path: P) -> NgSpiceResult<Library> {
        let library_path = library_path.as_ref();
        #[cfg(target_os = "linux")] {
            unsafe {
                let c_locale = CString::new("C").unwrap();
                setlocale(LC_NUMERIC, c_locale.as_ptr());
            }
        }

        log::debug!("Loading ngspice library: {:?}", library_path);
        let lib = unsafe { Library::new(library_path)? };

        Ok(lib)
    } 
}

impl NgSpiceShared {
    pub fn init(&mut self) -> NgSpiceResult<()> {
        self.api.init(
            Some(Self::send_char_callback), 
            Some(Self::send_stat_callback), 
            Some(Self::exit_callback),
            Some(Self::send_data_callback), 
            Some(Self::send_init_data_callback), 
            Some(Self::background_thread_running_callback), 
            self as *const Self as *mut c_void
        );

        self.api.init_sync(
            Some(Self::get_vsrc_data_callback), 
            Some(Self::get_isrc_data_callback), 
            None,
            &self.ngspice_id as *const i32 as *mut i32, 
            self as *const Self as *mut c_void
        );

        self.get_infomation()
    }

    pub fn exec_command(&mut self, command: &str) -> NgSpiceResult<String> {
        log::debug!("Execute command: {}", command);
        self.clear_output();

        let result = self.api.command(command).unwrap();
        if result != 0 {
            return Err(NgSpiceError::command(
                command.into(),
                format!("ngSpice_Command return '{}'", result)
            ));
        }

        if self.error_in_stdout || self.error_in_stderr {
            return Err(NgSpiceError::command(
                command.into(),
                "Error in stdout/stderr".into()
            ));
        } 

        Ok(self.stdout())
    }

    pub fn set(&mut self, key: &str) -> NgSpiceResult<()> {
        self.exec_command(&format!("set {}", key))?;
        Ok(())
    }

    pub fn reset(&mut self) -> NgSpiceResult<()> {
        self.exec_command("reset")?;
        Ok(())
    }

    pub fn status(&mut self) -> NgSpiceResult<String> {
        self.exec_command("status")
    }

    pub fn step(&mut self, step: Option<usize>) -> NgSpiceResult<()> {
        match step {
            Some(step) => self.exec_command(&format!("step {}", step))?,
            None => self.exec_command("step")?
        };
        Ok(())
    }

    pub fn listing(&mut self) -> NgSpiceResult<String> {
        self.exec_command("listing")
    }

    pub fn load_circuit(&mut self, circuit: &str) -> NgSpiceResult<()> {
        let circuit_lines: Vec<_> = circuit
            .lines()
            .collect();

        self.clear_output();
        let result = self.api.circ(&circuit_lines).unwrap();
        if result != 0 {
            return Err(NgSpiceError::circuit(circuit.into(), format!("ngSpice_Circ returned {}", result)));
        }

        if self.error_in_stdout || self.error_in_stderr {
            return Err(NgSpiceError::circuit(
                circuit.into(),
                "Error in stdout/stderr".into()
            ));
        } 

        Ok(())
    }

    /// Run the simulation
    pub fn run(&mut self, background: bool) -> NgSpiceResult<()> {
        let command = if background { "bg_run" } else { "run" };
        self.exec_command(command)?;

        if background {
            self.is_running = true;
        } else {
            log::debug!("Simulation is done");
        }
        
        Ok(())
    }

    pub fn halt(&mut self) -> NgSpiceResult<()> {
        self.exec_command("bg_halt")?;
        Ok(())
    }

    pub fn resume(&mut self, background: bool) -> NgSpiceResult<()> {
        let command = if background { "bg_resume" } else { "resume" };
        self.exec_command(command)?;
        Ok(())
    }

    pub fn get_vec(&self, name: &str) -> NgSpiceResult<Vec<f64>> {
        match self.api.get_vec_real_data(name).unwrap() {
            Some(data) => Ok(data),
            None => Err(NgSpiceError::ResultNotFound(name.into())),
        }
    }

    pub fn get_plot(&self, plot_name: &str) -> NgSpiceResult<Plot> {
        let vec_names = self.api.all_vecs(plot_name)?;
        let mut vectors = HashMap::new();

        for name in vec_names {
            let full_name = format!("{}.{}", plot_name, name);
            match self.api.get_vec_data(&full_name) {
                Ok(Some(data)) => {
                    match data {
                        VecData::Real(values) => {
                            let values = values.into_iter().map(|v| Value::real(v)).collect();
                            vectors.insert(name.clone(), values);
                        }
                        VecData::Complex(values) => {
                            let values = values.into_iter().map(|(re, im)| Value::complex(re, im)).collect();
                            vectors.insert(name.clone(), values);
                        }
                    }
                }
                _ => {
                    eprintln!("Warning: failed to load vector {}", full_name);
                }
            }
        }

        Ok(Plot {
            name: plot_name.to_string(),
            vectors,
        })
    }

    pub fn destroy(&mut self, plot_name: &str) -> NgSpiceResult<()> {
        self.exec_command(&format!("destroy {}", plot_name))?;
        Ok(())
    }

    pub fn destroy_all(&mut self) -> NgSpiceResult<()> {
        self.exec_command("destroy all")?;
        Ok(())
    }

    pub fn get_infomation(&mut self) -> NgSpiceResult<()> {
        self.ngspice_version = None;
        self.has_xspice = false;
        self.has_cider = false;
        self.extensions.clear();

        let output = self.exec_command("version -f")?;
        let version_regex = Regex::new(r"\*\* ngspice-(\d+)").unwrap();

        for line in output.lines() {
            if let Some(caps) = version_regex.captures(line) {
                if let Some(matched) = caps.get(1) {
                    self.ngspice_version = matched.as_str().parse::<u32>().ok();
                }
            }

            if line.contains("** XSPICE") {
                self.has_xspice = true;
                self.extensions.push("XSPICE".to_string());
            }

            if line.contains("CIDER") {
                self.has_cider = true;
                self.extensions.push("CIDER".to_string());
            }
        }

        log::debug!(
            "Ngspice version {:?} with extensions: {}",
            self.ngspice_version,
            self.extensions.join(", ")
        );

        Ok(())
    }

    fn clear_output(&mut self) {
        self.stdout.clear();
        self.stderr.clear();
        self.error_in_stdout = false;
        self.error_in_stderr = false;
    }
}

/// Property
impl NgSpiceShared {
    pub fn stdout(&self) -> String {
        self.stdout.join(" ")
    }

    pub fn stderr(&self) -> String {
        self.stderr.join(" ")
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn library_path(&self) -> &Path {
        self.library_path.as_path()
    }

    pub fn ngspice_version(&self) -> u32 {
        self.ngspice_version.unwrap()
    }

    pub fn has_xspice(&self) -> bool {
        self.has_xspice
    }

    pub fn has_cider(&self) -> bool {
        self.has_cider
    }

    pub fn set_callback(&mut self, callback: impl NgSpiceSharedCallback + 'static) {
        self.callback = Box::new(callback)
    }
}

impl NgSpiceShared {
    unsafe extern "C" fn send_char_callback(message_c: *mut c_char, id: c_int, user_data: *mut c_void) -> c_int {
        let shared: &mut Self = unsafe { &mut *(user_data as *mut Self) };
        let message = unsafe { match CStr::from_ptr(message_c).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return 1,
        }};

        log::debug!("[ngSpice raw] {message}");

        let (prefix, content) = if let Some(pos) = message.find(' ') {
            message.split_at(pos)
        } else {
            ("", &message[..])
        };

        let content = content.trim_start();

        if prefix == "stderr" {
            shared.stderr.push(content.to_string());
            if content.starts_with("Warning:") {
                eprintln!("[ngSpice warning] {content}");
            } else {
                shared.error_in_stderr = true;
                if content == "Note: can't find init file." {
                    shared.spinit_not_found = true;
                    eprintln!("[ngSpice warning] spinit was not found");
                } else {
                    eprintln!("[ngSpice error] {content}");
                }
            }
        } else {
            shared.stdout.push(content.to_string());
            if content.to_lowercase().contains("error") {
                shared.error_in_stdout = true;
                eprintln!("[ngSpice error? stdout] {content}");
            } else {
                println!("[ngSpice] {content}");
            }
        }

        shared.callback.send_char(&message, id)
    }

    unsafe extern "C" fn send_stat_callback(message_c: *mut c_char, id: c_int, user_data: *mut c_void) -> c_int {
        let shared: &mut Self = unsafe { &mut *(user_data as *mut Self) };
        let message = unsafe { match CStr::from_ptr(message_c).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return 1,
        }};

        shared.callback.send_stat(&message, id)
    }

    unsafe extern "C" fn exit_callback(exit_status: c_int, immediate_unloding: bool, quit_exit: bool, ngspice_id: c_int, _user_data: *mut c_void) -> c_int {
        log::debug!(
            "ngspice_id-{} exit status={} immediate_unloding={} quit_exit={}",
            ngspice_id,
            exit_status,
            immediate_unloding,
            quit_exit
        );
        exit_status
    }

    unsafe extern "C" fn send_data_callback(data: *mut VecValuesAll, number_of_vectors: c_int, ngspice_id: c_int, user_data: *mut c_void) -> c_int {
        let handler = unsafe { &mut *(user_data as *mut Self) };
        let data_ref = unsafe { &*data };
        let vecsa_array = unsafe { 
            std::slice::from_raw_parts(data_ref.vecsa, number_of_vectors as usize)
        };

        let mut actual_vector_values = HashMap::new();
        for &vec_ptr in vecsa_array {
            if vec_ptr.is_null() {
                continue;
            }
    
            let vec = unsafe { &*vec_ptr };
    
            let name = if vec.name.is_null() {
                "<null>".to_string()
            } else {
                unsafe { CStr::from_ptr(vec.name).to_string_lossy().into_owned() }
            };
    
            let value = Complex64::new(vec.creal, vec.cimag);
            actual_vector_values.insert(name, value);
        }
        
        handler.callback.send_data(actual_vector_values, number_of_vectors, ngspice_id)
    }

    unsafe extern "C" fn send_init_data_callback(data: *mut VecInfoAll, ngspice_id: c_int, user_data: *mut c_void) -> c_int {
        let handler = unsafe { &mut *(user_data as *mut Self) };
        if data.is_null() {
            return 0;
        }
        let data_ref = unsafe { &*data };
        handler.callback.send_init_data(data_ref, ngspice_id)
    }

    unsafe extern "C" fn background_thread_running_callback(is_running: bool, ngspice_id: c_int, user_data: *mut c_void) -> c_int {
        let handler = unsafe { &mut *(user_data as *mut Self) };
        log::debug!("ngspice_id-{} background_thread_running {}", ngspice_id, is_running);
        handler.is_running = is_running;
        0
    }

    unsafe extern "C" fn get_vsrc_data_callback(voltage: *mut c_double, time: c_double, node: *mut c_char, ngspice_id: c_int, user_data: *mut c_void) -> c_int {
        let handler = unsafe { &mut *(user_data as *mut Self) };
        let node = unsafe { match CStr::from_ptr(node).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return 1,
        }};
        let voltage = unsafe { &mut *voltage };

        handler.callback.get_vsrc_data(voltage, time, node, ngspice_id)
    }

    unsafe extern "C" fn get_isrc_data_callback(current: *mut c_double, time: c_double, node: *mut c_char, ngspice_id: c_int, user_data: *mut c_void) -> c_int {
        let handler = unsafe { &mut *(user_data as *mut Self) };
        let node = unsafe { match CStr::from_ptr(node).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return 1,
        }};
        let current = unsafe { &mut *current }; 

        handler.callback.get_isrc_data(current, time, node, ngspice_id)
    }
}

impl NgSpiceShared {
    pub fn simulate(&mut self, circuit: &str) -> NgSpiceResult<Plot> {
        self.init()?;
        // self.destroy_all()?;
        self.load_circuit(circuit)?;
        self.run(false)?;
        let plot_name = self.api.cur_plot().unwrap();
        let plot = self.get_plot(&plot_name)?;
        Ok(plot)
    }
}

impl Simulate for NgSpiceShared {
    type Err = NgSpiceError;

    fn run_dc(&mut self, netlist: &str) -> Result<DcVoltageAnalysis, Self::Err> {
        let plot = self.simulate(netlist)?;
        plot.to_dc_voltage_analysis()
    }

    fn run_op(&mut self, netlist: &str) -> Result<OpAnalysis, Self::Err> {
        let plot = self.simulate(netlist)?;
        plot.to_op_analysis()
    }

    fn run_tran(&mut self, netlist: &str) -> Result<TranAnalysis, Self::Err> {
        let plot = self.simulate(netlist)?;
        plot.to_tran_analysis()    
    }
    
    fn run_ac(&mut self, netlist: &str) -> Result<AcAnalysis, Self::Err> {
        let plot = self.simulate(netlist)?;
        plot.to_ac_analysis()
    }
}

#[allow(unused)]
#[cfg(test)]
mod tests {
    use reda_unit::{num, u};
    use crate::simulate::ngspice::shared;

    use super::*;
    use std::ffi::CString;
    use std::os::raw::{c_char, c_int};
    use callback::DefaultNgSpiceSharedCallback;
    use libloading::Symbol;

    #[test]
    fn test_load_lib() {
        let mut ng = NgSpiceShared::default().expect("Failed to create NgSpiceShared");
        ng.init().expect("Failed to init ngspice");
        ng.exec_command("version");

    }

    #[test]
    fn test_use_shared() {
        let mut ng = NgSpiceShared::default().expect("Failed to create NgSpiceShared");
        ng.init().expect("Failed to init ngspice");
    
        let netlist = r#"
* RC low-pass filter
V1 in 0 DC 1
R1 in out 1k
C1 out 0 1u
.IC V(out)=0
.tran 1u 1m
.end
    "#;
    
        ng.load_circuit(netlist).expect("Failed to load circuit");
        ng.run(false).expect("Failed to run simulation");
    
        let time = ng.api.get_vec_real_data("time")
            .expect("Vec access failed")
            .expect("Missing time vector");
    
        let vout = ng.api.get_vec_real_data("v(out)")
            .expect("Vec access failed")
            .expect("Missing v(out) vector");

        println!("v(out): {:?}", &vout);
        println!("{}", time.len());
        println!("{}", vout.len());
    }

    #[test]
    fn test_tran_analysis() {
        let mut ng = NgSpiceShared::default().expect("Failed to create NgSpiceShared");
        ng.init().expect("Failed to init ngspice");
    
        let netlist = r#"
* RC low-pass filter
V1 in 0 DC 1
R1 in out 1k
C1 out 0 1u
.IC V(out)=0
.tran 1u 1m
.end
    "#;
    
        ng.load_circuit(netlist).expect("Failed to load circuit");
        ng.run(false).expect("Failed to run simulation");

        let plot_name = ng.api.cur_plot().expect("No current plot");
        let plot = ng.get_plot(&plot_name).expect("plot");
        let analysis = plot.to_tran_analysis().expect("ana");

        println!("{}", analysis.time.len());

        println!("node:");
        for (name, values) in analysis.nodes.iter() {
            println!("{}", name);
        }
        
        println!("branches:");
        for (name, values) in analysis.branches.iter() {
            println!("{}", name);
        }
        
        println!("internal_parameters:");
        for (name, values) in analysis.internal_parameters.iter() {
            println!("{}", name);
        }

        println!("{}", analysis.get_voltage_at("out", u!(200 us)).unwrap());
    }

    #[test]
    fn test_dc_analysis() {
        let mut ng = NgSpiceShared::default()
            .expect("Failed to create NgSpiceShared");
        ng.init().expect("Failed to init ngspice");
    
        let netlist = r#"
    * Simple DC Sweep
    V1 in 0 DC 0
    R1 in out 2k
    R2 out 0 1k
    .dc V1 0 5 0.1
    .end
        "#;
    
        ng.load_circuit(netlist).expect("Failed to load circuit");
        ng.run(false).expect("Failed to run simulation");
    
        let plot_name = ng.api.cur_plot().expect("No current plot");
        let plot = ng.get_plot(&plot_name).expect("Failed to get plot");
    
        let analysis = plot.to_dc_voltage_analysis().expect("Failed to convert to DC analysis");
    
        println!("Sweep points: {}", analysis.sweep.len());
    
        println!("Nodes:");
        for (name, _) in analysis.nodes.iter() {
            println!("  {}", name);
        }
    
        println!("Branches:");
        for (name, _) in analysis.branches.iter() {
            println!("  {}", name);
        }
    
        println!("Internal parameters:");
        for (name, _) in analysis.internal_parameters.iter() {
            println!("  {}", name);
        }
    
        // 测试一个节点电压（如 "out"）在扫到 V1 = 2.0V 时的值
        let vout = analysis.get_voltage_at("out", u!(2.0 V));
        match vout {
            Some(val) => println!("Voltage at out when V1=2.0V: {} V", val.to_f64()),
            None => println!("No matching voltage value found for V1=2.0V"),
        }
    }
    
    #[test]
    fn test_operating_point_analysis() {
        let mut ng = 
            NgSpiceShared::default()
            .expect("Failed to create NgSpiceShared");
        ng.init().expect("Failed to init ngspice");
    
        let netlist = r#"
    * Operating Point Test
    V1 in 0 DC 5
    R1 in out 1k
    R2 out 0 2k
    .op
    .end
        "#;
    
        ng.load_circuit(netlist).expect("Failed to load circuit");
        ng.run(false).expect("Failed to run simulation");
    
        let plot_name = ng.api.cur_plot().expect("No current plot");
        let plot = ng.get_plot(&plot_name).expect("Failed to get plot");
    
        let analysis = plot
            .to_op_analysis()
            .expect("Failed to convert to operating point analysis");
    
        println!("Nodes:");
        for (name, values) in &analysis.nodes {
            println!("  {}: {:?}", name, values);
        }
    
        println!("Branches:");
        for (name, values) in &analysis.branches {
            println!("  {}: {:?}", name, values);
        }
    
        println!("Internal parameters:");
        for (name, values) in &analysis.internal_parameters {
            println!("  {}: {:?}", name, values);
        }
    
        // 检查输出节点的电压值
        let vout = analysis.nodes.get("out").unwrap();
        println!("{}", vout);
    }
    
}