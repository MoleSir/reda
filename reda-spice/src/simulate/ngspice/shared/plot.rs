use std::collections::HashMap;
use crate::probe::{AcAnalysis, DcAnalysis, DcVoltageAnalysis, OpAnalysis, ToAnalysis, TranAnalysis};
use super::{NgSpiceError, NgSpiceResult};
use crate::Value;
use reda_unit::{Current, Frequency, Number, Temperature, Time, Voltage};

#[derive(Debug, Clone)]
pub struct Plot {
    pub name: String,
    pub vectors: HashMap<String, Vec<Value>>,
}

impl ToAnalysis for Plot {
    type Err = NgSpiceError;
    
    fn to_tran_analysis(&self) -> NgSpiceResult<TranAnalysis> {
        let time = self.extract_time_vector().ok_or(NgSpiceError::NoTimeInTranAnalysis)?;

        let mut nodes = HashMap::new();
        let mut branches = HashMap::new();
        let mut internal_parameters = HashMap::new();
    
        for (name, values) in &self.vectors {
            if name == "time" {
                continue;
            }

            if Self::is_voltage_node(name) {
                let voltages = Value::extract_units(&values)
                    .ok_or(NgSpiceError::UnexpectComplexValue)?;
                nodes.insert(name.clone(), voltages);
            } else if Self::is_branch_current(name) {
                let currents = Value::extract_units(&values)
                    .ok_or(NgSpiceError::UnexpectComplexValue)?;
                branches.insert(name.clone(), currents);
            } else if Self::is_internal_parameter(name) {
                let numbers = Value::extract_numbers(&values)
                    .ok_or(NgSpiceError::UnexpectComplexValue)?;
                internal_parameters.insert(name.clone(), numbers);
            }
        }

        Ok(TranAnalysis {
            time,
            nodes,
            branches,
            internal_parameters,
        })
    }

    fn to_dc_voltage_analysis(&self) -> NgSpiceResult<DcVoltageAnalysis> {
        let sweep = self.extract_v_sweep()?;

        let mut nodes = HashMap::new();
        let mut branches = HashMap::new();
        let mut internal_parameters = HashMap::new();

        for (name, values) in &self.vectors {
            if name == "v-sweep" {
                continue; // sweep
            }

            if Self::is_voltage_node(name) {
                let voltages = Value::extract_units(values)
                    .ok_or(NgSpiceError::UnexpectComplexValue)?;
                nodes.insert(name.clone(), voltages);
            } else if Self::is_branch_current(name) {
                let current = Value::extract_units(values)
                    .ok_or(NgSpiceError::UnexpectComplexValue)?;
                branches.insert(name.clone(), current);

            } else if Self::is_internal_parameter(name) {
                let numbers = Value::extract_numbers(values)
                    .ok_or(NgSpiceError::UnexpectComplexValue)?;
                internal_parameters.insert(name.clone(), numbers);
            }
        }

        Ok(DcAnalysis {
            sweep,
            nodes,
            branches,
            internal_parameters,
        })
    }

    fn to_op_analysis(&self) -> NgSpiceResult<OpAnalysis> {
        let mut nodes = HashMap::new();
        let mut branches = HashMap::new();
        let mut internal_parameters = HashMap::new();

        for (name, values) in &self.vectors {
            let value = Self::first_value_to_number(values)?;

            if Self::is_voltage_node(name) {
                nodes.insert(name.clone(), value.into());
            } else if Self::is_branch_current(name) {
                branches.insert(name.clone(), value.into());

            } else if Self::is_internal_parameter(name) {
                internal_parameters.insert(name.clone(), value);
            }
        }

        Ok(OpAnalysis {
            nodes,
            branches,
            internal_parameters,
        })
    }

    fn to_ac_analysis(&self) -> Result<AcAnalysis, Self::Err> {
        let frequency = self.extract_frequency_vector().ok_or(NgSpiceError::NoFrequencyInAcAnalysis)?;

        let mut nodes = HashMap::new();
        let mut branches = HashMap::new();
        let mut internal_parameters = HashMap::new();
    
        for (name, values) in &self.vectors {
            if name == "frequency" {
                continue;
            }

            if Self::is_voltage_node(name) {
                let voltages = Value::extract_unit_complexs(&values)
                    .ok_or(NgSpiceError::UnexpectComplexValue)?;                
                nodes.insert(name.clone(), voltages);
            } else if Self::is_branch_current(name) {
                let currents = Value::extract_unit_complexs(&values)
                    .ok_or(NgSpiceError::UnexpectComplexValue)?;
                branches.insert(name.clone(), currents);
            } else if Self::is_internal_parameter(name) {
                let numbers = Value::extract_complexs(&values)
                    .ok_or(NgSpiceError::UnexpectComplexValue)?;
                internal_parameters.insert(name.clone(), numbers);
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

impl Plot {
    fn extract_time_vector(&self) -> Option<Vec<Time>> {
        let time_vec = self.vectors.get("time")?;
        let mut time = Vec::new();
        for v in time_vec {
            match v {
                Value::Real(f) => time.push(Time::new(*f)),
                _ => return None, 
            }
        }
        Some(time)
    }

    fn extract_frequency_vector(&self) -> Option<Vec<Frequency>> {
        let frequency_vec = self.vectors.get("frequency")?;
        let mut frequency = Vec::new();
        for v in frequency_vec {
            match v {
                Value::Complex(c) => frequency.push(Frequency::new(c.re)),
                _ => return None, 
            }
        }
        Some(frequency)
    }

    fn extract_v_sweep(&self) -> NgSpiceResult<Vec<Voltage>> {
        let vectors = self.vectors.get("v-sweep")
            .ok_or(NgSpiceError::NoVSweepInDcVolatgeAnalysis)?;

        Value::extract_units(vectors)
            .ok_or(NgSpiceError::UnexpectComplexValue)
    }

    #[allow(unused)]
    fn extract_i_sweep(&self) -> NgSpiceResult<Vec<Current>> {
        let vectors = self.vectors.get("i-sweep")
            .ok_or(NgSpiceError::NoVSweepInDcVolatgeAnalysis)?;

        Value::extract_units(vectors)
            .ok_or(NgSpiceError::UnexpectComplexValue)
    }

    #[allow(unused)]
    fn extract_temp_sweep(&self) -> NgSpiceResult<Vec<Temperature>> {
        let vectors = self.vectors.get("temp-sweep")
            .ok_or(NgSpiceError::NoVSweepInDcVolatgeAnalysis)?;

        Value::extract_units(vectors)
            .ok_or(NgSpiceError::UnexpectComplexValue)
    }

    fn first_value_to_number(values: &[Value]) -> NgSpiceResult<Number> {
        match values.get(0).unwrap() {
            Value::Real(r) => Ok(*r),
            Value::Complex(_) => Err(NgSpiceError::UnexpectComplexValue),
        }
    }
    
    fn is_voltage_node(name: &str) -> bool {
        !name.contains("#branch") && !name.starts_with('@')
    }
    
    fn is_branch_current(name: &str) -> bool {
        name.contains("#branch")
    }
    
    fn is_internal_parameter(name: &str) -> bool {
        name.starts_with('@')
    }
}
