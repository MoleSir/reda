use std::collections::HashMap;
use nom::{bytes::complete::tag_no_case, error::{context, VerboseError, VerboseErrorKind}, Err};
use nom::{character::complete::char, multi::many0};
use crate::model::{Capacitor, Component, Diode, Inductor, Model, ModelKind, MosFET, MosFETBuilder, Resistor, BJT};
use super::{capacitance_number, hws, identifier, inductance_number, node, number, resistance_number, NomResult, ToFailure};
use reda_unit::Number;

use nom::branch::alt;
use nom::combinator::map;

pub fn component(input: &str) -> NomResult<Component> {
    alt((
        map(resistor, Component::R),
        map(capacitor, Component::C),
        map(inductor, Component::L),
        map(diode, Component::D),
        map(bjt, Component::Q),
        map(mos_fet, Component::M),
    ))(input)
}

/// Rname N+ N- Value
pub fn resistor(input: &str) -> NomResult<Resistor> {
    context("resistor", |input| {
        let (input, name) = context("name", hws(identifier))(input)?;
        if !name.starts_with('R') && !name.starts_with('r') {
            return Err(Err::Error(VerboseError {
                errors: [(input, VerboseErrorKind::Context("should begin with R"))].into(),
            }));
        }

        let (input, node_pos) = hws(node)(input).to_failure()?;
        let (input, node_neg) = hws(node)(input).to_failure()?;
        let (input, resistance) = hws(resistance_number)(input).to_failure()?;

        let r = Resistor {
            name: name[1..].to_string(),
            node_pos: node_pos.to_string(),
            node_neg: node_neg.to_string(),
            resistance,
        };

        Ok((input, r))  
    })(input)
}

/// Cname N+ N- Value <IC=Initial Condition>
pub fn capacitor(input: &str) -> NomResult<Capacitor> {
    context("capacitor", |input| {
        let (input, name) = context("name", hws(identifier))(input)?;
        if !name.starts_with('C') && !name.starts_with('c') {
            return Err(Err::Error(VerboseError {
                errors: [(input, VerboseErrorKind::Context("should begin with C"))].into(),
            }));
        }

        let (input, node_pos) = hws(node)(input).to_failure()?;
        let (input, node_neg) = hws(node)(input).to_failure()?;
        let (input, value) = hws(capacitance_number)(input).to_failure()?;

        Ok((
            input,
            Capacitor {
                name: name[1..].to_string(),
                node_pos: node_pos.to_string(),
                node_neg: node_neg.to_string(),
                capacitance: value,
            },
        ))
    })(input)
}

/// Lname N+ N- Value <IC=Initial Condition>
pub fn inductor(input: &str) -> NomResult<Inductor> {
    context("inductor", |input| {
        let (input, name) = context("name", hws(identifier))(input)?;
        if !name.starts_with('L') && !name.starts_with('l') {
            return Err(Err::Error(VerboseError {
                errors: [(input, VerboseErrorKind::Context("should begin with L"))].into(),
            }));
        }

        let (input, node_pos) = hws(node)(input).to_failure()?;
        let (input, node_neg) = hws(node)(input).to_failure()?;
        let (input, value) = hws(inductance_number)(input).to_failure()?;

        Ok((
            input,
            Inductor {
                name: name[1..].to_string(),
                node_pos: node_pos.to_string(),
                node_neg: node_neg.to_string(),
                inductance: value,
            },
        ))
    })(input)
}

/// Dname N+ N- MODName
pub fn diode(input: &str) -> NomResult<Diode> {
    context("diode", |input| {
        let (input, name) = context("name", hws(identifier))(input)?;
        if !name.starts_with('D') && !name.starts_with('d') {
            return Err(Err::Error(VerboseError {
                errors: [(input, VerboseErrorKind::Context("should begin with D"))].into(),
            }));
        }

        let (input, node_pos) = hws(node)(input).to_failure()?;
        let (input, node_neg) = hws(node)(input).to_failure()?;
        let (input, model_name) = hws(identifier)(input).to_failure()?;

        Ok((
            input,
            Diode {
                name: name[1..].to_string(),
                node_pos: node_pos.to_string(),
                node_neg: node_neg.to_string(),
                model_name: model_name.to_string(),
            },
        ))
    })(input)
}

/// Qname NC NB NE Model
pub fn bjt(input: &str) -> NomResult<BJT> {
    context("bjt", |input| {
        let (input, name) = context("name", hws(identifier))(input)?;
        if !name.starts_with('Q') && !name.starts_with('q') {
            return Err(Err::Error(VerboseError {
                errors: [(input, VerboseErrorKind::Context("should begin with Q"))].into(),
            }));
        }

        let (input, collector) = hws(node)(input).to_failure()?;
        let (input, base) = hws(node)(input).to_failure()?;
        let (input, emitter) = hws(node)(input).to_failure()?;
        let (input, model_name) = hws(identifier)(input).to_failure()?;

        Ok((
            input,
            BJT {
                name: name[1..].to_string(),
                collector: collector.to_string(),
                base: base.to_string(),
                emitter: emitter.to_string(),
                model_name: model_name.to_string(),
            },
        ))
    })(input)
}

/// Mname ND NG NS NB ModelName [params]
pub fn mos_fet(input: &str) -> NomResult<MosFET> {
    context("mosfet", |input| {
        let (input, name) = context("name", hws(identifier))(input)?;
        if !name.starts_with('M') && !name.starts_with('m') {
            return Err(Err::Error(VerboseError {
                errors: [(input, VerboseErrorKind::Context("should begin with M"))].into(),
            }));
        }

        let mut builder = MosFETBuilder::default();

        let (input, drain) = hws(node)(input).to_failure()?;
        let (input, gate) = hws(node)(input).to_failure()?;
        let (input, source) = hws(node)(input).to_failure()?;
        let (input, bulk) = hws(node)(input).to_failure()?;
        let (input, model_name) = hws(identifier)(input).to_failure()?;

        builder
            .drain(drain)
            .gate(gate)
            .source(source)
            .bulk(bulk)
            .model_name(model_name)
            .name(&name[1..]);

        let mut parameters = HashMap::new();
        let (input, raw_parameters) = many0(hws(parameter_pair))(input)?;
        for (k, v) in raw_parameters {
            match k.to_ascii_lowercase().as_str() {
                "l" => { builder.length(v); }
                "w" => { builder.width(v); }
                _ => { parameters.insert(k, v); }
            }
        }
        builder.parameters(parameters);

        match builder.build() {
            Ok(mos) => Ok((input, mos)),
            Err(_) => Err(Err::Failure(VerboseError {
                errors: [(input, VerboseErrorKind::Context("no w/l given"))].into(),
            }))
        }
    })(input)
}

/// .model <name> <type> (<param1=val1 param2=val2 ...>)
pub fn model(input: &str) -> NomResult<Model> {
    let (input, _) = context("keyword", hws(tag_no_case(".model")))(input)?;
    let (input, name) = hws(identifier)(input).to_failure()?;
    let (input, kind) = hws(model_kind)(input).to_failure()?;

    let (input, _) = hws(tag_no_case("("))(input).to_failure()?;
    let (input, parameters) = many0(parameter_pair)(input).to_failure()?;
    let (input, _) = hws(tag_no_case(")"))(input).to_failure()?;
    
    Ok((input, Model {
        name: name.to_string(),
        kind,
        parameters: parameters.into_iter().collect(),
    }))
}

fn model_kind(input: &str) -> NomResult<ModelKind> {
    map(
        hws(alt((
            tag_no_case("NPN"),
            tag_no_case("D"),
            tag_no_case("NMOS"),
            tag_no_case("PMOS"),
        ))),
        |s: &str| match &s.to_ascii_uppercase()[..] {
            "NPN" => ModelKind::NPN,
            "D" => ModelKind::Diode,
            "NMOS" => ModelKind::NMos,
            "PMOS" => ModelKind::PMos,
            _ => unreachable!(),
        },
    )(input)
}

/// Parse a key=value pair where key is identifier and value is Number
fn parameter_pair(input: &str) -> NomResult<(String, Number)> {
    let (input, key) = hws(identifier)(input)?;
    let (input, _)   = hws(char('='))(input)?;
    let (input, val) = hws(number)(input)?;
    Ok((input, (key.to_string(), val)))
}

#[allow(unused)]
#[cfg(test)]
mod test {
    use reda_unit::{num, u};
    use super::*;

    #[test]
    fn test_basic() {
        let (left, r) = resistor("Rhhh 1  \n+n2 1.5k  \n777").unwrap();
        assert_eq!(left, "\n777");
        assert_eq!(r.name, "hhh");
        assert_eq!(r.node_pos, "1");
        assert_eq!(r.node_neg, "n2");
        assert_eq!(r.resistance, u!(1.5 kΩ));

        let (left, c) = capacitor("C1 nplus nminus 10u extra").unwrap();
        assert_eq!(left, "extra");
        assert_eq!(c.name, "1");
        assert_eq!(c.node_pos, "nplus");
        assert_eq!(c.node_neg, "nminus");
        assert_eq!(c.capacitance, u!(10.0 uF));

        let (left, l) = inductor("Lfoo a b 5m more").unwrap();
        assert_eq!(left, "more");
        assert_eq!(l.name, "foo");
        assert_eq!(l.node_pos, "a");
        assert_eq!(l.node_neg, "b");
        assert_eq!(l.inductance, u!(5.0 mH));
    }

    #[test]
    fn test_basic_failed() {
        let result = resistor("Xhhh 1  n2 1.5k  777");
        assert!(matches!(result, Err(Err::Error(_))));

        let result = resistor("Rhhh 1 n2 _");
        assert!(matches!(result, Err(Err::Failure(_))));

        let result= resistor("Rhhh 1  \r\nn2 1.5k  \n777");
        assert!(matches!(result, Err(Err::Failure(_))));
    }

    #[test]
    fn test_mosfet_parse() {
        let input = "M1 D G S B NM L=1u W=5u VTH=0.7 KP=20u\n";
        let (_, mos) = mos_fet(input).unwrap();
    
        assert_eq!(mos.name, "1");
        assert_eq!(mos.drain, "D");
        assert_eq!(mos.gate, "G");
        assert_eq!(mos.source, "S");
        assert_eq!(mos.bulk, "B");
        assert_eq!(mos.model_name, "NM");
        assert_eq!(mos.length, u!(1.0 um));
        assert_eq!(mos.width,  u!(5.0 um));
        assert_eq!(mos.parameters.get("VTH"), Some(&num!(0.7)));
        assert_eq!(mos.parameters.get("KP"), Some(&num!(20.0 u)));

        let input = "M1 out in vdd vdd pmos L=1u W=2u\n";
        let (_, mos) = mos_fet(input).unwrap();
    
        assert_eq!(mos.name, "1");
        assert_eq!(mos.drain, "out");
        assert_eq!(mos.gate, "in");
        assert_eq!(mos.source, "vdd");
        assert_eq!(mos.bulk, "vdd");
        assert_eq!(mos.model_name, "pmos");
        assert_eq!(mos.length, u!(1.0 um));
        assert_eq!(mos.width,  u!(2.0 um));

        let input = "M1 \n+out in vdd vdd \n+pmos L=1u W=2u\n";
        let (_, mos) = mos_fet(input).unwrap();
    
        assert_eq!(mos.name, "1");
        assert_eq!(mos.drain, "out");
        assert_eq!(mos.gate, "in");
        assert_eq!(mos.source, "vdd");
        assert_eq!(mos.bulk, "vdd");
        assert_eq!(mos.model_name, "pmos");
        assert_eq!(mos.length, u!(1.0 um));
        assert_eq!(mos.width,  u!(2.0 um));
    }

    #[test]
    fn test_mosfet_failed() {
        let result = mos_fet("KK D G S B NM L=1u W=5u VTH=0.7 KP=20u\n");
        assert!(matches!(result, Err(Err::Error(_))));

        let result = mos_fet("M1 D G S B NM L=1u VTH=0.7 KP=20u\n");
        assert!(matches!(result, Err(Err::Failure(_))));

        let result = mos_fet("M1 D G  NM L=1u VTH=0.7 KP=20u\n");
        assert!(matches!(result, Err(Err::Failure(_))));
    }

    #[test]
    fn test_component_match() {
        macro_rules! assert_component {
            ($input:expr, $variant:path) => {{
                let (_, c) = component($input).expect("parse failed");
                match c {
                    $variant(_) => {}
                    _ => panic!("Expected {:?}, got {:?}", stringify!($variant), c),
                }
            }};
        }
        
        assert_component!("R1 n1 n2 10k", Component::R);
        assert_component!("C1 n2 0 5u", Component::C);
        assert_component!("L1 n2 n3 10n", Component::L);
        assert_component!("D1 n1 n0 Dmod", Component::D);
        assert_component!("Q1 c b e NPN", Component::Q);
        assert_component!("M1 d g s b Mmod L=1u W=5u", Component::M);
    }

    #[test]
    fn test_component_failed() {
        let result = component("KK D G S B NM L=1u W=5u VTH=0.7 KP=20u\n");
        assert!(matches!(result, Err(Err::Error(_))));

        let result = mos_fet("M2 D G S B NM L=1u VTH=0.7 KP=20u\n");
        assert!(matches!(result, Err(Err::Failure(_))));
    }

}