mod rawfile;
use std::{io::Write, path::PathBuf, process::{Command, Stdio}};
use rawfile::RawFile;
use crate::{probe::{AcAnalysis, DcVoltageAnalysis, OpAnalysis, ToAnalysis, TranAnalysis}, simulate::Simulate};

use super::error::{NgSpiceError, NgSpiceResult};

pub struct NgSpiceServer {
    ngspice_path: PathBuf,
}

impl NgSpiceServer {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self { ngspice_path: path.into() }
    }

    pub fn run(&self, netlist: &str) -> NgSpiceResult<RawFile> {
        let mut child = Command::new(&self.ngspice_path)
            .arg("-s")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(NgSpiceError::Io)?;

        child.stdin.as_mut().unwrap().write_all(netlist.as_bytes())?;
        
        let output = child.wait_with_output()?;
        let stdout = output.stdout;
        Self::parse_stdout(&stdout)?;
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        let number_of_points = Self::parse_point_count(&stderr)?;

        Ok(RawFile::parse(&stdout, number_of_points).map_err(|s| NgSpiceError::ParseRawFile(s.to_string()))?)
    }

    fn parse_point_count(stderr: &str) -> NgSpiceResult<usize> {
        for line in stderr.lines() {
            if let Some(caps) = line.strip_prefix("@@@ ") {
                let parts: Vec<_> = caps.split_whitespace().collect();
                if parts.len() >= 2 {
                    return Ok(parts[1].parse().map_err(|_| NgSpiceError::MissingPoints)?);
                }
            }
        }
        Err(NgSpiceError::MissingPoints)
    }

    fn parse_stdout(_stdout: &[u8]) -> NgSpiceResult<()> {
        Ok(())
    }
}

impl Simulate for NgSpiceServer {
    type Err = NgSpiceError;

    fn run_dc(&mut self, netlist: &str) -> Result<DcVoltageAnalysis, Self::Err> {
        let rawfile = self.run(netlist)?;
        rawfile.to_dc_voltage_analysis()
    }

    fn run_op(&mut self, netlist: &str) -> Result<OpAnalysis, Self::Err> {
        let rawfile = self.run(netlist)?;
        rawfile.to_op_analysis()
    }

    fn run_tran(&mut self, netlist: &str) -> Result<TranAnalysis, Self::Err> {
        let rawfile = self.run(netlist)?;
        rawfile.to_tran_analysis()    
    }

    fn run_ac(&mut self, netlist: &str) -> Result<AcAnalysis, Self::Err> {
        let rawfile = self.run(netlist)?;
        rawfile.to_ac_analysis()   
    }
}

#[allow(unused)]
#[cfg(test)]
mod tests {
    use std::{io::{stdout, Write}, process::{Command, Stdio}};

    use crate::probe::ToAnalysis;
    use reda_unit::{num, u};

    use super::NgSpiceServer;

    #[test]
    fn test_op() {
        let netlist = r#"
    * Operating Point Test
    V1 in 0 DC 5
    R1 in out 1k
    R2 out 0 2k
    .op
    .end
    "#;

        let server = NgSpiceServer::new("/usr/local/bin/ngspice");
        let raw = server.run(netlist).expect("run fialed");
        println!("{:?}", raw);
    }

    #[test]
    fn test_op_analysis() {
        let netlist = r#"
    * Operating Point Test
    V1 in 0 DC 5
    R1 in out 1k
    R2 out 0 2k
    .op
    .end
    "#;

        let server = NgSpiceServer::new("/usr/local/bin/ngspice");
        let rawfile = server.run(netlist).expect("run fialed");
        let analysis = rawfile.to_op_analysis().expect("to dc");

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

    #[test]
    fn test_dc() {
        let netlist = r#"
    * Simple DC Sweep
    V1 in 0 DC 0
    R1 in out 2k
    R2 out 0 1k
    .dc V1 0 5 0.1
    .end
    "#;

        let server = NgSpiceServer::new("/usr/local/bin/ngspice");
        let raw = server.run(netlist).expect("run fialed");
        println!("{:?}", raw);
    }

    #[test]
    fn test_dc_analysis() {
        let netlist = r#"
    * Simple DC Sweep
    V1 in 0 DC 0
    R1 in out 2k
    R2 out 0 1k
    .dc V1 0 5 0.1
    .end
    "#;

        let server = NgSpiceServer::new("/usr/local/bin/ngspice");
        let rawfile = server.run(netlist).expect("run fialed");
        let analysis = rawfile.to_dc_voltage_analysis().expect("Failed to convert to DC analysis");
    
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
    fn test_tran_analysis() {
        let netlist = r#"
* RC low-pass filter
V1 in 0 DC 1
R1 in out 1k
C1 out 0 1u
.IC V(out)=0
.tran 1u 1m
.end
    "#;
    
        let server = NgSpiceServer::new("/usr/local/bin/ngspice");
        let rawfile = server.run(netlist).expect("run fialed");

        let analysis = rawfile.to_tran_analysis().expect("ana");

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
}