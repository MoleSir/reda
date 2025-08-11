use std::{collections::HashMap, io::Cursor};
use byteorder::{LittleEndian, ReadBytesExt};
use derive_builder::Builder;
use runit::{Current, Frequency, Number, Temperature, Time, Voltage};
use std::str;

use crate::{
    probe::{AcAnalysis, DcAnalysis, DcVoltageAnalysis, OpAnalysis, ToAnalysis, TranAnalysis}, 
    simulate::ngspice::{NgSpiceError, NgSpiceResult},
    Value
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarType {
    Voltage,
    Current,
    Time,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub vartype: VarType,
    pub data: Vec<Value>,
}

impl Variable {
    pub fn is_voltage(&self) -> bool {
        self.vartype == VarType::Voltage
    }

    pub fn is_current(&self) -> bool {
        self.vartype == VarType::Current
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Flags {
    Real,
    Complex,
}

#[derive(Debug, Builder)]
#[builder(setter(strip_option, into))]
pub struct RawFile {
    #[builder(default)]
    pub circuit: Option<String>,
    #[builder(default)]
    pub title: Option<String>,
    pub date: String,
    pub plotname: String,
    pub flags: Flags,
    pub num_vars: usize,
    pub num_points: usize,
    pub variables: Vec<Variable>,
}

#[derive(thiserror::Error, Debug)]
pub enum RawFileError {
    #[error("Missing Binary line")]
    MissingBinaryLine,

    #[error("Invalid header: {0}")]
    InvalidHeader(String),

    #[error("Invalid binday: {0}")]
    InvalidBinary(String),
    
    #[error("Invalid header field '{0}' for '{1}'")]
    InvalidHeaderField(&'static str, String),

    #[error("Unexpect Terminate")]
    UnexpectTerminate,

    #[error("Build raw file error: {0}")]
    Build(#[from] RawFileBuilderError)
}

pub type RawFileResult<T> = Result<T, RawFileError>;

impl RawFile {
    pub fn parse(buf: &[u8], num_points: usize) -> RawFileResult<Self> {
        let header_end = find_subslice(buf, b"Binary:\n")
            .ok_or(RawFileError::MissingBinaryLine)?
            + "Binary:\n".len();

        let header = &buf[..header_end];
        let raw_data = &buf[header_end..];

        let header_str = std::str::from_utf8(header).map_err(|e| RawFileError::InvalidHeader(e.to_string()))?;
        println!("{}", header_str);
        let mut lines = header_str.lines();

        let mut builder = RawFileBuilder::default();
        builder.num_points(num_points);
        let mut variables = Vec::new();

        while let Some(line) = lines.next() {
            if line.starts_with("Title:") {
                builder.title(line[6..].trim());
            } else if line.starts_with("Date:") {
                builder.date(line[5..].trim());
            } else if line.starts_with("Plotname:") {
                builder.plotname(line[9..].trim());
            } else if line.starts_with("Flags:") {
                let f = line[6..].trim();
                let flags = match f {
                    "real" => Flags::Real,
                    "complex" => Flags::Complex,
                    f => return Err(RawFileError::InvalidHeaderField("Flags", format!("Unknown flag {}", f))),
                };
                builder.flags(flags);
            } else if line.starts_with("No. Variables:") {
                let num_vars = line[14..].trim()
                    .parse::<usize>()
                    .map_err(|e| RawFileError::InvalidHeaderField("No. Variables", e.to_string()))?;
                builder.num_vars(num_vars);
            } else if line.starts_with("No. Points:") {
                // let num_points = line[11..].trim()
                //     .parse::<usize>()
                //     .map_err(|e| RawFileError::InvalidHeaderField("No. Points", e.to_string()))?;
                // builder.num_points(num_points);
            } else if line.starts_with("Variables:") {
                // No. of Data Columns
                lines
                    .next()
                    .ok_or(RawFileError::UnexpectTerminate)?;

                let first_vline = loop {
                    let line = lines.next().ok_or(RawFileError::UnexpectTerminate)?;
                    if line.starts_with('\t') {
                        break line;
                    }
                };

                let num_vars = builder.num_vars
                    .ok_or(RawFileError::InvalidHeader("Variables come before No. Variables".into()))?;
  
                for i in 0..num_vars {
                    let vline =  if i == 0 {
                        first_vline
                    } else {
                        lines.next().ok_or(RawFileError::UnexpectTerminate)?
                    };      
                    let parts: Vec<&str> = vline.split_whitespace().collect();
                    if parts.len() != 3 {
                        return Err(RawFileError::InvalidHeaderField("Variables", format!("Bad var line: {}", vline)));
                    }
                    let name = parts[1].to_string();
                    let typ = match parts[2] {
                        "voltage" => VarType::Voltage,
                        "current" => VarType::Current,
                        "time" => VarType::Time,
                        t => return Err(RawFileError::InvalidHeaderField("Variables", format!("Unknown var type {}", t))),
                    };
                    variables.push(Variable {
                        name,
                        vartype: typ,
                        data: Vec::with_capacity(num_points)
                    });
                }
            }
        }

        // Now read raw data
        let num_vars = builder.num_vars
            .ok_or(RawFileError::InvalidHeader("No exit 'No. Variables'".into()))?;
        let flags = builder.flags
            .ok_or(RawFileError::InvalidHeader("No exit 'Flags'".into()))?;

        let mut reader = Cursor::new(raw_data);
        for _ in 0..num_points {
            for v in 0..num_vars {
                match flags {
                    Flags::Real => {
                        let val = reader.read_f64::<LittleEndian>()
                            .map_err(|e| RawFileError::InvalidBinary(e.to_string()))?;
                        variables[v].data.push(Value::real(val));
                    }
                    Flags::Complex => {
                        let re = reader.read_f64::<LittleEndian>()
                            .map_err(|e| RawFileError::InvalidBinary(e.to_string()))?;
                        let im = reader.read_f64::<LittleEndian>()
                            .map_err(|e| RawFileError::InvalidBinary(e.to_string()))?;
                        variables[v].data.push(Value::complex(re, im));
                    }
                }
            }
        }
        builder.variables(variables);

        Ok(builder.build()?)
    }
}

impl ToAnalysis for RawFile {
    type Err = NgSpiceError;

    fn to_tran_analysis(&self) -> NgSpiceResult<TranAnalysis> {
        let time = self.extract_time_vector().ok_or(NgSpiceError::NoTimeInTranAnalysis)?;

        let mut nodes = HashMap::new();
        let mut branches = HashMap::new();
    
        for variable in &self.variables {
            if variable.name == "time" {
                continue;
            }

            let name = Self::clean_name(&variable.name);

            if variable.is_voltage() {
                let voltages = Value::extract_units(&variable.data)
                    .ok_or(NgSpiceError::UnexpectComplexValue)?;
                nodes.insert(name.to_string(), voltages);
            } else if variable.is_current() {
                let currents = Value::extract_units(&variable.data)
                    .ok_or(NgSpiceError::UnexpectComplexValue)?;
                branches.insert(name.to_string(), currents);
            }
        }

        Ok(TranAnalysis {
            time,
            nodes,
            branches,
            internal_parameters: HashMap::new(),
        })
    }

    fn to_dc_voltage_analysis(&self) -> NgSpiceResult<DcVoltageAnalysis> {
        let sweep = self.extract_v_sweep()?;

        let mut nodes = HashMap::new();
        let mut branches = HashMap::new();

        for variable in &self.variables {
            if variable.name == "v(v-sweep)" {
                continue; // sweep
            }

            let name = Self::clean_name(&variable.name);

            if variable.is_voltage() {
                let voltages = Value::extract_units(&variable.data)
                    .ok_or(NgSpiceError::UnexpectComplexValue)?;
                nodes.insert(name.to_string(), voltages);
            } else if variable.is_current() {
                let currents = Value::extract_units(&variable.data)
                    .ok_or(NgSpiceError::UnexpectComplexValue)?;
                branches.insert(name.to_string(), currents);
            }
        }

        Ok(DcAnalysis {
            sweep,
            nodes,
            branches,
            internal_parameters: HashMap::new(),
        })
    }

    fn to_op_analysis(&self) -> NgSpiceResult<OpAnalysis> {
        let mut nodes = HashMap::new();
        let mut branches = HashMap::new();

        for variable in &self.variables {
            let value = Self::first_value_to_number(&variable.data)?;
            let name = Self::clean_name(&variable.name);

            if variable.is_voltage() {
                nodes.insert(name.to_string(), value.into());
            } else if variable.is_current() {
                branches.insert(name.to_string(), value.into());
            }
        }

        Ok(OpAnalysis {
            nodes,
            branches,
            internal_parameters: HashMap::new(),
        })
    }

    fn to_ac_analysis(&self) -> Result<AcAnalysis, Self::Err> {
        let frequency = self.extract_frequency_vector().ok_or(NgSpiceError::NoFrequencyInAcAnalysis)?;
    
        let mut nodes = HashMap::new();
        let mut branches = HashMap::new();
        let internal_parameters = HashMap::new();
    
        for variable in &self.variables {
            if variable.name == "frequency" {
                continue;
            }


            let name = Self::clean_name(&variable.name);

            if variable.is_voltage() {
                let voltages = Value::extract_unit_complexs(&variable.data)
                    .ok_or(NgSpiceError::UnexpectComplexValue)?;
                nodes.insert(name.to_string(), voltages);
            } else if variable.is_current() {
                let currents = Value::extract_unit_complexs(&variable.data)
                    .ok_or(NgSpiceError::UnexpectComplexValue)?;
                branches.insert(name.to_string(), currents);
            }
        }

        Ok(AcAnalysis {
            frequency,
            nodes,
            branches,
            internal_parameters
        })
    }
}


impl RawFile {
    fn find_variable(&self, name: &str) -> Option<&Variable> {
        for var in self.variables.iter() {
            if var.name == name {
                return Some(var);
            }
        }
        None
    }

    fn extract_time_vector(&self) -> Option<Vec<Time>> {
        let time_var = self.find_variable("time")?;
        let mut time = Vec::new();
        for v in &time_var.data {
            match v {
                Value::Real(f) => time.push(Time::new(*f)),
                _ => return None, 
            }
        }
        Some(time)
    }

    fn extract_frequency_vector(&self) -> Option<Vec<Frequency>> {
        let frequency_var = self.find_variable("frequency")?;
        let mut frequency = Vec::new();
        for v in &frequency_var.data {
            match v {
                Value::Complex(c) => frequency.push(Frequency::new(c.re)),
                _ => return None, 
            }
        }
        Some(frequency)
    }

    fn extract_v_sweep(&self) -> NgSpiceResult<Vec<Voltage>> {
        let var = self.find_variable("v(v-sweep)")
            .ok_or(NgSpiceError::NoVSweepInDcVolatgeAnalysis)?;

        Value::extract_units(&var.data)
            .ok_or(NgSpiceError::UnexpectComplexValue)
    }

    #[allow(unused)]
    fn extract_i_sweep(&self) -> NgSpiceResult<Vec<Current>> {
        let var = self.find_variable("i(i-sweep)")
            .ok_or(NgSpiceError::NoVSweepInDcVolatgeAnalysis)?;

        Value::extract_units(&var.data)
            .ok_or(NgSpiceError::UnexpectComplexValue)
    }

    #[allow(unused)]
    fn extract_temp_sweep(&self) -> NgSpiceResult<Vec<Temperature>> {
        let var = self.find_variable("temp-sweep")
            .ok_or(NgSpiceError::NoVSweepInDcVolatgeAnalysis)?;

        Value::extract_units(&var.data)
            .ok_or(NgSpiceError::UnexpectComplexValue)
    }

    fn first_value_to_number(values: &[Value]) -> NgSpiceResult<Number> {
        match values.get(0).unwrap() {
            Value::Real(r) => Ok(*r),
            Value::Complex(_) => Err(NgSpiceError::UnexpectComplexValue),
        }
    }

    fn clean_name(name: &str) -> &str {
        if name.contains('(') {
            let len = name.len();
            &name[2..(len-1)]
        } else {
            name
        }
    }
}


fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}
