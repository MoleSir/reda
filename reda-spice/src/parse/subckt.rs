use nom::{bytes::complete::tag_no_case, error::{context, VerboseError, VerboseErrorKind}, multi::many0, Err};

use crate::model::{Instance, Subckt};
use super::{comment, component, hws, identifier, node, NomResult, ToFailure};

/// Parse a complete .SUBCKT ... .ENDS block from &str
pub fn subckt<'a>(input: &'a str) -> NomResult<'a, Subckt> {
    context("subckt", |mut input: &'a str| {
        // Parse header line
        let (i, _) = context("keyword", hws(tag_no_case(".SUBCKT")))(input)?;
        let (i, (name, ports)) = context("declaration", hws(subckt_decl))(i).to_failure()?;
        input = i;

        let mut components = vec![];
        let mut instances = vec![];

        // line-by-line
        loop {
            input = input.trim_start();
            if let Ok((rest, _)) = comment(input) {
                input = rest;
                continue;
            }

            // Check for .ENDS
            if input.to_ascii_lowercase().starts_with(".ends") {
                // consume this line and break
                if let Some(pos) = input.find('\n') {
                    input = &input[pos + 1..];
                } else {
                    input = "";
                }
                break;
            }
            
            // Try component
            match hws(component)(input) {
                Ok((rest, c)) => {
                    components.push(c);
                    input = rest;
                    continue;
                }
                Err(Err::Error(_)) => {
                    // Try instance
                    match hws(instance)(input) {
                        Ok((rest, inst)) => {
                            instances.push(inst);
                            input = rest;
                            continue;
                        }
                        Err(Err::Error(_)) => {
                            return Err(Err::Failure(VerboseError {
                                errors: [(input, VerboseErrorKind::Context("unknown line in subckt"))].into(),
                            }));
                        }
                        Err(e @ Err::Failure(_)) | Err(e @ Err::Incomplete(_)) => return Err(e),
                    }
                }
                Err(e @ Err::Failure(_)) | Err(e @ Err::Incomplete(_)) => return Err(e),
            }
        }

        Ok((
            input,
            Subckt {
                name,
                ports,
                components,
                instances,
            },
        ))
    })(input)
}

fn subckt_decl(input: &str) -> NomResult<(String, Vec<String>)> {
    context("subckt_decl", |input| {
        let (input, name) = context("name", hws(identifier))(input)?;
        let (input, ports) = context("ports", many0(hws(node)))(input)?;
        Ok((
            input,
            (
                name.to_string(),
                ports.iter().map(|s| s.to_string()).collect(),
            ),
        ))
    })(input)
}

/// Xname node1 node2 ... subckt_name
pub fn instance(input: &str) -> NomResult<Instance> {
    context("instance", |input| {
        let (input, name) = context("name", hws(identifier))(input)?;

        if !name.starts_with('X') && !name.starts_with('x') {
            return Err(Err::Error(VerboseError {
                errors: [(input, VerboseErrorKind::Context("should begin with X"))].into(),
            }));
        }

        let (input, args) = context("args", many0(hws(node)))(input)?;

        if args.is_empty() {
            return Err(Err::Failure(VerboseError {
                errors: [(input, VerboseErrorKind::Context("missing subckt name"))].into(),
            }));
        }

        let subckt_name = args.last().unwrap().to_string();
        let pins = args[..args.len() - 1]
            .iter()
            .map(|s| s.to_string())
            .collect();

        Ok((
            input,
            Instance {
                name: name.to_string(),
                pins,
                subckt_name,
            },
        ))
    })(input)
}

#[allow(unused)]
#[cfg(test)]
mod test {
    use super::*;
    use nom::{Err, error::convert_error};

    #[test]
    fn test_parse_subckt_and_instance() {
        let input = 
        r#".SUBCKT inverter in out vdd gnd
        M1 out in vdd vdd pmos L=1u W=2u
        M2 out in gnd gnd nmos L=1u W=1u
        .ENDS
        Xinv a b vdd gnd inverter
        "#;

        let (rest, subckt) = subckt(input).unwrap();

        assert_eq!(subckt.name, "inverter");
        assert_eq!(subckt.ports, ["in", "out", "vdd", "gnd"]);
        assert_eq!(subckt.components.len(), 2);

        let (_, inst) = instance(rest.trim()).unwrap();
        assert_eq!(inst.name, "Xinv");
        assert_eq!(inst.pins, ["a", "b", "vdd", "gnd"]);
        assert_eq!(inst.subckt_name, "inverter");
    }

    #[test]
    fn test_instance_bad_prefix_error() {
        let input = "Y1 in out myblk";
        let res = instance(input);
        assert!(matches!(res, Err(Err::Error(_))));
    }

    #[test]
    fn test_subckt_unknown_line_failure() {
        let input = r#".SUBCKT foo in out
            ??? bad line
            .ENDS
        "#;

        let res = subckt(input);
        assert!(matches!(res, Err(Err::Failure(_))));
    }
}
