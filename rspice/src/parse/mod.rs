mod base;
mod components;
mod source;
mod simulate;
mod measures;
mod error;
mod subckt;
use std::path::Path;

use nom::branch::alt;

pub use base::*;
pub use components::*;
pub use source::*;
pub use simulate::*;
pub use measures::*;
pub use error::*;
pub use subckt::*;

use crate::model::{Component, Instance, MeasureCommand, Model, SimCommand, Source, Spice, Subckt};
use nom::{error::convert_error, Err};
use nom::combinator::map;

pub fn load_spice<P: AsRef<Path>>(path: P) -> Result<Spice, SpiceReadError> {
    let input = std::fs::read_to_string(path.as_ref())?;
    read_spice(&input)
}

pub fn read_spice(full_input: &str) -> Result<Spice, SpiceReadError> {
    let mut spice = Spice::default();
    let mut input = full_input;

    while !input.trim_start().is_empty() {
        input = skip_blank_or_comment_lines(input);
        if input.is_empty() {
            break;
        }

        match statement(input) {
            Ok((rest, stmt)) => {
                match stmt {
                    ParsedStatement::Component(c) => spice.components.push(c),
                    ParsedStatement::Source(s) => spice.sources.push(s),
                    ParsedStatement::SimCommand(s) => spice.simulation.push(s),
                    ParsedStatement::Measure(m) => spice.measures.push(m),
                    ParsedStatement::Instance(i) => spice.instances.push(i),
                    ParsedStatement::Subckt(s) => spice.subckts.push(s),
                    ParsedStatement::Model(m) => spice.model.push(m),
                }
                input = rest;
            }
            Err(Err::Failure(e)) => {
                let first_error_input = e.errors.get(0).map(|(slice, _)| *slice).unwrap_or(input);
                let err_text = convert_error(full_input, e);
                let line_num = get_error_line(full_input, first_error_input);
                return Err(SpiceReadError::Parse(format!("Error at line {}:\n{}", line_num, err_text)));
            }
            Err(Err::Error(_)) => {
                let line_num = get_error_line(full_input, input);
                return Err(SpiceReadError::Parse(format!(
                    "At line {}: Unknown statement: {}",
                    line_num,
                    preview_line(input)
                )));
            }
            Err(Err::Incomplete(e)) => {
                return Err(SpiceReadError::Parse(format!("Incomplete: {:?}", e)));
            }
        }
    }

    Ok(spice)
}

fn get_error_line(full_input: &str, error_input: &str) -> usize {
    let err_pos = error_input.as_ptr() as usize - full_input.as_ptr() as usize;
    let line_num = full_input[..err_pos].chars().filter(|&c| c == '\n').count() + 1;
    line_num
}

pub enum ParsedStatement {
    Component(Component),
    Source(Source),
    SimCommand(SimCommand),
    Measure(MeasureCommand),
    Instance(Instance),
    Subckt(Subckt),
    Model(Model),
}

fn statement(input: &str) -> NomResult<ParsedStatement> {
    alt((
        map(component, ParsedStatement::Component),
        map(source, ParsedStatement::Source),
        map(sim_command, ParsedStatement::SimCommand),
        map(measure_command, ParsedStatement::Measure),
        map(instance, ParsedStatement::Instance),
        map(subckt, ParsedStatement::Subckt),
        map(model, ParsedStatement::Model),
    ))(input)
}

pub fn skip_blank_or_comment_lines(mut input: &str) -> &str {
    loop {
        input = input.trim_start();
        if input.is_empty() {
            return "";
        }

        if let Ok((rest, _)) = comment(input) {
            input = rest;
            continue;
        }

        if input.starts_with('\n') {
            input = &input[1..];
            continue;
        }

        return input;
    }
}

fn preview_line(input: &str) -> &str {
    input.lines().next().unwrap_or(input).trim()
}


#[cfg(test)]
mod tests {
    use crate::model::{Component, MeasureCommand, SimCommand, SourceValue};
    use runit::u;

    use super::*;

    #[test]
    fn test_read_spice_with_subckt_and_instance() {
        let input = r#"
    * Test circuit
    .SUBCKT inv in out vdd gnd
    M1 out in vdd vdd pmos L=1u W=2u
    M2 out in gnd gnd nmos L=1u W=1u
    .ENDS
    
    X1 a b vdd gnd inv
    Vdd vdd 0 DC 5
    .TRAN 1n 10n
    "#;
        
        let spice = read_spice(input).map_err(|e| {
            println!("{}", e);
        }).unwrap();
        
        assert_eq!(spice.subckts.len(), 1);
        assert_eq!(spice.instances.len(), 1);
        assert_eq!(spice.sources.len(), 1);
        assert_eq!(spice.simulation.len(), 1);
    }

    #[test]
    fn test_read_spice_basic() {
        let input = r#"
            * Simple resistor circuit
            R1 in out 10k
            C1 out 0 1u
            V1 in 0 DC 5
            .TRAN 1n 10n
            .MEAS TRAN rise_time TRIG V(out) VAL=0.2 RISE=1 TARG V(out) VAL=0.8 RISE=1
        "#;

        let spice = read_spice(input).map_err(|e| {
            println!("{}", e);
        }).unwrap();

        // Check number of parsed objects
        assert_eq!(spice.components.len(), 2);  // R1, C1
        assert_eq!(spice.sources.len(), 1);     // V1
        assert_eq!(spice.simulation.len(), 1);  // .TRAN
        assert_eq!(spice.measures.len(), 1);    // .MEAS

        // Validate parsed resistor
        if let Component::R(r) = &spice.components[0] {
            assert_eq!(r.name, "1");
            assert_eq!(r.node_pos, "in");
            assert_eq!(r.node_neg, "out");
            assert_eq!(r.resistance, u!(10. kΩ));
        } else {
            panic!("Expected resistor");
        }

        // Validate parsed source
        assert_eq!(&spice.sources[0].name, "1");
        assert_eq!(&spice.sources[0].node_pos, "in");
        assert_eq!(&spice.sources[0].node_neg, "0");
        if let SourceValue::DcVoltage(dc) = &spice.sources[0].value {
            assert_eq!(*dc, u!(5. V));
        } else {
            panic!("Expected DC voltage source");
        }

        // Validate .TRAN
        if let SimCommand::Tran(tran) = &spice.simulation[0] {
            assert_eq!(tran.t_step, u!(1. ns));
            assert_eq!(tran.t_stop, u!(10. ns));
        } else {
            panic!("Expected .TRAN command");
        }

        // Validate .MEAS
        if let MeasureCommand::Rise(rise) = &spice.measures[0] {
            assert_eq!(rise.name, "rise_time");
        } else {
            panic!("Expected .MEAS rise command");
        }
    }

    #[test]
    fn test_unknown_statement_error() {
        let input = "
            R1 in out 1k
            .DC V1 0 10 1
            ??? this line is invalid
            .TRAN 1n 10n
        ";
        let result = read_spice(input);
        assert!(matches!(result, Err(SpiceReadError::Parse(_))));
        println!("{:?}", result.unwrap_err());
    }
    
    #[test]
    fn test_subckt_parse_failure() {
        let input = "
            .SUBCKT
            R1 a b 1k
            .ENDS
        ";
        let result = read_spice(input);
        assert!(matches!(result, Err(SpiceReadError::Parse(_))));
    }

    #[test]
    fn test_read_spice_basic_components() {
        let input = r#"
            R1 1 0 1k
            C1 1 0 1u
            L1 1 0 10u
            D1 1 0 Dmodel
            Q1 3 2 0 NPN
            M1 3 2 1 0 nmos L=0.18u W=1u
        "#;

        let spice = read_spice(input).unwrap();
        assert_eq!(spice.components.len(), 6);
    }

    #[test]
    fn test_read_spice_sources_dc() {
        let input = r#"
            V1 1 0 DC 5
            I1 2 0 0.001
        "#;

        let spice = read_spice(input).unwrap();
        assert_eq!(spice.sources.len(), 2);
    }

    #[test]
    fn test_read_spice_sources_sin_pwl() {
        let input = r#"
            Vsin 1 0 SIN(0 5 1k)
            Vpwl 2 0 PWL(0 0 1u 5 2u 0)
        "#;

        let spice = read_spice(input).unwrap();
        assert_eq!(spice.sources.len(), 2);
    }

    #[test]
    fn test_read_spice_sim_commands() {
        let input = r#"
            .DC Vin 0 5 0.1
            .AC LIN 10 1 1k
            .TRAN 1n 10n 0n 1n
        "#;

        let spice = read_spice(input).unwrap();
        assert_eq!(spice.simulation.len(), 3);
    }

    #[test]
    fn test_read_spice_measures() {
        let input = r#"
            .model NMOS1 NMOS (LEVEL=1 VTO=0.7 KP=20u LAMBDA=0.02)
            .model PMOS1 PMOS (LEVEL=1 VTO=-0.7 KP=10u LAMBDA=0.02)
            .MEAS TRAN t_rise TRIG V(1) VAL=0.2 RISE=1 TARG V(1) VAL=0.8 RISE=1
            .MEAS TRAN avgval AVG V(1) FROM=1n TO=10n
            .MEAS TRAN when FIND I(V1) WHEN V(2)=1.0
        "#;

        let spice = read_spice(input).unwrap();
        assert_eq!(spice.measures.len(), 3);
    }

    #[test]
    fn test_read_spice_subckt_and_instance() {
        let input = r#"
.SUBCKT inverter in out vdd gnd
M1 out in vdd vdd pmos 
+ L=1u W=2u
M2 out in gnd gnd nmos L=1u W=1u
.ENDS

Xinv a b vdd gnd inverter
        "#;

        let spice = read_spice(input).unwrap();
        assert_eq!(spice.subckts.len(), 1);
        assert_eq!(spice.instances.len(), 1);
        assert_eq!(spice.subckts[0].ports, ["in", "out", "vdd", "gnd"]);
        assert_eq!(spice.subckts[0].components.len(), 2);
    }

    #[test]
    fn test_read_spice_line_continuation() {
        let input = r#"
V1 1 0 
+ DC 5
        "#;

        let spice = read_spice(input).unwrap();
        assert_eq!(spice.sources.len(), 1);
    }

    #[test]
    fn test_read_spice_comment_and_blank_lines() {
        let input = r#"
* This is a comment
R1 1 0 1k

* Another comment
C1 1 0 1u
        "#;

        let spice = read_spice(input).unwrap();
        assert_eq!(spice.components.len(), 2);
    }

    #[test]
    fn test_read_spice_failure_invalid_line() {
        let input = r#"
R1 1 0 1k
THIS_IS_INVALID
        "#;

        let res = read_spice(input);
        assert!(res.is_err());
        if let Err(SpiceReadError::Parse(msg)) = res {
            assert!(msg.contains("Unknown statement"));
        } else {
            panic!("Expected SpiceReadError::Parse");
        }
    }
}

