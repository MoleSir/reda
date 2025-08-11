
use libloading::{Library, Symbol};
use std::ffi::{CStr, CString, c_char, c_double, c_int, c_void};

/// 表示一个变量（电压、电流等）在某个仿真时间点上的值
#[repr(C)]
pub struct VecValues {
    pub name: *mut c_char,   // 变量名（如 "V(out)"）
    pub creal: f64,          // 实部（通常就是仿真值）
    pub cimag: f64,          // 虚部（AC 仿真时使用）
    pub is_scale: bool,      // 是否是时间轴或频率轴
    pub is_complex: bool,    // 是否是复数类型（AC 时为 true）
}

/// 表示一次仿真时间点（或频率点）中，所有变量的值
#[repr(C)]
pub struct VecValuesAll {
    pub veccount: c_int,             // 变量个数
    pub vecindex: c_int,             // 当前时间步索引
    pub vecsa: *mut *mut VecValues,  // 指向 VecValues 的数组
}

#[repr(C)]
pub struct NgComplex {
    pub cx_real: f64,
    pub cx_imag: f64,
}

#[repr(C)]
pub struct VectorInfo {
    pub v_name: *mut c_char,
    pub v_type: c_int,
    pub v_flags: i16,
    pub v_realdata: *mut c_double,
    pub v_compdata: *mut NgComplex,
    pub v_length: c_int,
}


/// 表示某个变量的元信息（用于初始化时）
#[repr(C)]
pub struct VecInfo {
    pub number: c_int,         // 变量编号
    pub vecname: *mut c_char,  // 变量名
    pub is_real: bool,         // 是否是实数变量
    pub pdvec: *mut c_void,    // 指向变量值数组
    pub pdvecscale: *mut c_void, // 指向时间轴或频率轴数组
}

/// 仿真开始时的变量信息整体结构
#[repr(C)]
pub struct VecInfoAll {
    pub name: *mut c_char,     // 当前 plot 名称（如 "tran1"）
    pub title: *mut c_char,    // 电路标题
    pub date: *mut c_char,     // 仿真日期
    pub type_: *mut c_char,    // 仿真类型（如 "tran"）
    pub veccount: c_int,       // 变量数量
    pub vecs: *mut *mut VecInfo, // 所有变量的描述数组
}

/// NgSpice 向用户输出普通消息（stdout）
pub type SendChar = unsafe extern "C" fn(*mut c_char, c_int, *mut c_void) -> c_int;

/// 输出状态信息（通常是仿真状态）
pub type SendStat = SendChar;

/// NgSpice 退出时触发的回调（可以在此释放资源）
pub type ControlledExit = unsafe extern "C" fn(c_int, bool, bool, c_int, *mut c_void) -> c_int;

/// 仿真运行过程中，每步传输数据时调用
pub type SendData = unsafe extern "C" fn(*mut VecValuesAll, c_int, c_int, *mut c_void) -> c_int;

/// 仿真开始时传送初始变量信息
pub type SendInitData = unsafe extern "C" fn(*mut VecInfoAll, c_int, *mut c_void) -> c_int;

/// 后台线程是否在运行的通知
pub type BGThreadRunning = unsafe extern "C" fn(bool, c_int, *mut c_void) -> c_int;

/// 同步接口：获取电压源值（如 PWL 定义的电源）
pub type GetVSRCData = unsafe extern "C" fn(*mut c_double, c_double, *mut c_char, c_int, *mut c_void) -> c_int;

/// 同步接口：获取电流源值（如 PWL 定义的电流源）
pub type GetISRCData = GetVSRCData;

/// 同步接口：获取时间同步数据（一般用于实时仿真同步）
pub type GetSyncData = unsafe extern "C" fn(c_double, *mut c_double, c_double, c_int, c_int, c_int, *mut c_void) -> c_int;

/// 包装 ngSpice 动态库
pub struct NgSpiceAPI {
    lib: Library,  // libloading 加载的动态库
}

pub enum VecData {
    Real(Vec<f64>),
    Complex(Vec<(f64, f64)>),
}

impl NgSpiceAPI {
    /// 创建 NgSpiceAPI 对象（从已加载的动态库）
    pub fn new(lib: Library) -> Self {
        Self { lib }
    }

    /// 初始化 ngSpice 引擎，注册各类回调函数
    /// 必须在运行仿真前调用
    pub fn init(
        &self,
        send_char: Option<SendChar>,
        send_stat: Option<SendStat>,
        controlled_exit: Option<ControlledExit>,
        send_data: Option<SendData>,
        send_init_data: Option<SendInitData>,
        bg_thread_running: Option<BGThreadRunning>,
        user_data: *mut c_void,
    ) -> i32 {
        unsafe {
            let f: Symbol<unsafe extern "C" fn(
                Option<SendChar>,
                Option<SendStat>,
                Option<ControlledExit>,
                Option<SendData>,
                Option<SendInitData>,
                Option<BGThreadRunning>,
                *mut c_void,
            ) -> i32> = self.lib.get(b"ngSpice_Init\0").unwrap();
            f(send_char, send_stat, controlled_exit, send_data, send_init_data, bg_thread_running, user_data)
        }
    }

    /// 初始化同步模式接口（少用，一般用于实时交互仿真）
    pub fn init_sync(
        &self,
        get_vsrc_data: Option<GetVSRCData>,
        get_isrc_data: Option<GetISRCData>,
        get_sync_data: Option<GetSyncData>,
        ret_code: *mut i32,
        user_data: *mut c_void,
    ) -> i32 {
        unsafe {
            let f: Symbol<unsafe extern "C" fn(
                Option<GetVSRCData>,
                Option<GetISRCData>,
                Option<GetSyncData>,
                *mut i32,
                *mut c_void,
            ) -> i32> = self.lib.get(b"ngSpice_Init_Sync\0").unwrap();
            f(get_vsrc_data, get_isrc_data, get_sync_data, ret_code, user_data)
        }
    }

    /// 向 NgSpice 发送命令（等价于控制台中输入）
    /// 例如：".tran 1n 1u", "run", "display"
    pub fn command(&self, cmd: &str) -> Result<i32, std::ffi::NulError> {
        let cstr = CString::new(cmd)?;
        unsafe {
            let f: Symbol<unsafe extern "C" fn(*const c_char) -> i32> = self.lib.get(b"ngSpice_Command\0").unwrap();
            Ok(f(cstr.as_ptr()))
        }
    }

    /// 提交 netlist（电路结构）
    /// 输入为字符串数组，必须 null 结尾
    pub fn circ(&self, lines: &[&str]) -> Result<i32, std::ffi::NulError> {
        let mut cstrings: Vec<CString> = lines.iter().map(|s| CString::new(*s)).collect::<Result<_, _>>()?;
        let mut ptrs: Vec<*mut c_char> = cstrings.iter_mut().map(|s| s.as_ptr() as *mut c_char).collect();
        ptrs.push(std::ptr::null_mut()); // null-terminated

        unsafe {
            let f: Symbol<unsafe extern "C" fn(*mut *mut c_char) -> i32> = self.lib.get(b"ngSpice_Circ\0").unwrap();
            Ok(f(ptrs.as_mut_ptr()))
        }
    }

    /// 返回当前激活的 plot（如 "tran1"）
    pub fn cur_plot(&self) -> Option<String> {
        unsafe {
            let f: Symbol<unsafe extern "C" fn() -> *mut c_char> = self.lib.get(b"ngSpice_CurPlot\0").unwrap();
            let ptr = f();
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// 获取所有已生成的 plot 名称（每次仿真生成一个）
    pub fn all_plots(&self) -> Vec<String> {
        unsafe {
            let f: Symbol<unsafe extern "C" fn() -> *mut *mut c_char> = self.lib.get(b"ngSpice_AllPlots\0").unwrap();
            let mut result = Vec::new();
            let mut ptr = f();
            while !ptr.is_null() && !(*ptr).is_null() {
                result.push(CStr::from_ptr(*ptr).to_string_lossy().into_owned());
                ptr = ptr.add(1);
            }
            result
        }
    }

    /// 获取某个 plot 中的所有变量名（如 "V(out)", "I(V1)" 等）
    pub fn all_vecs(&self, plotname: &str) -> Result<Vec<String>, std::ffi::NulError> {
        let cstr = CString::new(plotname)?;
        unsafe {
            let f: Symbol<unsafe extern "C" fn(*mut c_char) -> *mut *mut c_char> = self.lib.get(b"ngSpice_AllVecs\0").unwrap();
            let mut result = Vec::new();
            let mut ptr = f(cstr.into_raw());
            while !ptr.is_null() && !(*ptr).is_null() {
                result.push(CStr::from_ptr(*ptr).to_string_lossy().into_owned());
                ptr = ptr.add(1);
            }
            Ok(result)
        }
    }

    /// 当前仿真是否正在运行中（后台线程）
    pub fn running(&self) -> bool {
        unsafe {
            let f: Symbol<unsafe extern "C" fn() -> bool> = self.lib.get(b"ngSpice_running\0").unwrap();
            f()
        }
    }

    /// 设置仿真中断点时间（如暂停于 2us）
    pub fn set_breakpoint(&self, time: f64) -> bool {
        unsafe {
            let f: Symbol<unsafe extern "C" fn(c_double) -> bool> = self.lib.get(b"ngSpice_SetBkpt\0").unwrap();
            f(time)
        }
    }

    /// 获取指定变量的波形数据（仅支持实数变量）
    pub fn get_vec_real_data(&self, name: &str) -> Result<Option<Vec<f64>>, std::ffi::NulError> {
        let cstr = CString::new(name)?;
        unsafe {
            // 加载 ngGet_Vec_Info
            let f: Symbol<unsafe extern "C" fn(*mut c_char) -> *mut VectorInfo> =
                self.lib.get(b"ngGet_Vec_Info\0").unwrap();

            // 调用 C 函数获取 vector_info
            let ptr = f(cstr.into_raw());

            if ptr.is_null() {
                return Ok(None);
            }

            // 解引用 vector_info 结构
            let info = &*ptr;

            if info.v_realdata.is_null() || info.v_length <= 0 {
                return Ok(None);
            }

            let len = info.v_length as usize;
            let slice = std::slice::from_raw_parts(info.v_realdata, len);
            Ok(Some(slice.to_vec()))
        }
    }

    pub fn get_vec_complex_data(&self, name: &str) -> Result<Option<Vec<(f64, f64)>>, std::ffi::NulError> {
        let cstr = CString::new(name)?;
        unsafe {
            let f: Symbol<unsafe extern "C" fn(*mut c_char) -> *mut VectorInfo> =
                self.lib.get(b"ngGet_Vec_Info\0").unwrap();
    
            let ptr = f(cstr.into_raw());
            if ptr.is_null() {
                return Ok(None);
            }
    
            let info = &*ptr;
    
            if info.v_compdata.is_null() || info.v_length <= 0 {
                return Ok(None);
            }
    
            let len = info.v_length as usize;
            let slice = std::slice::from_raw_parts(info.v_compdata, len);
            Ok(Some(slice.iter().map(|c| (c.cx_real, c.cx_imag)).collect()))
        }
    }
    
    pub fn get_vec_data(&self, name: &str) -> Result<Option<VecData>, std::ffi::NulError> { 
        let cstr = CString::new(name)?;
        unsafe {
            let f: Symbol<unsafe extern "C" fn(*mut c_char) -> *mut VectorInfo> =
                self.lib.get(b"ngGet_Vec_Info\0").unwrap();
            
            // call c api
            let ptr = f(cstr.into_raw());
            if ptr.is_null() {
                return Ok(None);
            }
            
            // to rust struct
            let info = &*ptr;
            
            if info.v_length <= 0 {
                return Ok(None);
            }

            match (info.v_realdata.is_null(), info.v_compdata.is_null()) {
                (false, true) => {
                    let len = info.v_length as usize;
                    let slice = std::slice::from_raw_parts(info.v_realdata, len);
                    Ok(Some(VecData::Real(slice.to_vec())))
                }
                (true, false) => {
                    let len = info.v_length as usize;
                    let slice = std::slice::from_raw_parts(info.v_compdata, len);
                    Ok(Some(VecData::Complex(slice.iter().map(|c| (c.cx_real, c.cx_imag)).collect())))
                },
                (_, _) => return Ok(None),
            }
        }
    }
}
