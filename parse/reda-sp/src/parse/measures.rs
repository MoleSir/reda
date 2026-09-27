use nom::{
    branch::alt, bytes::complete::{tag, tag_no_case, take_until}, combinator::map, error::context, sequence::{delimited, preceded}
};

use crate::model::{AnalysisType, FindWhenCondition, EdgeType, MeasureBasicStat, MeasureCommand, MeasureFindWhen, MeasureFunction, MeasureRise, OutputSuffix, OutputVariable, TrigTargCondition};
use super::{hws, identifier, number, time_number, unsigned_int, NomResult, ToFailure};

/// .MEAS TRAN rise ... 
pub fn measure_command(input: &str) -> NomResult<MeasureCommand> {
    context("measure_command", |input| {
        let (input, _) = context("keyword", hws(tag_no_case(".MEAS")))(input)?;
        let (input, analysis) = context("analysis_type", hws(analysis_type))(input).to_failure()?;
        let (input, name) = context("measure_name", hws(identifier))(input).to_failure()?;

        let x = alt((
            context("measure_rise", map(|i| measure_rise(i, name, analysis), MeasureCommand::Rise)),
            context("measure_basic_stat", map(|i| measure_basic_stat(i, name, analysis), MeasureCommand::BasicStat)),
            context("measure_find_when", map(|i| measure_find_when(i, name, analysis), MeasureCommand::FindWhen)),
        ))(input).to_failure();
        x
    })(input)
}

/// .MEAS TRAN rise TRIG V(1) VAL=.2 RISE=1
///                 TARG V(1) VAL=.8 RISE=1
fn measure_rise<'a>(input: &'a str, name: &'a str, analysis: AnalysisType) -> NomResult<'a, MeasureRise> {
    let (input, _) = context("TRIG keyword", hws(tag_no_case("TRIG")))(input)?;
    let (input, trig) = context("trigger_condition", hws(trig_targ_condition))(input).to_failure()?;
    let (input, _) = context("TARG keyword", hws(tag_no_case("TARG")))(input).to_failure()?;
    let (input, targ) = context("target_condition", hws(trig_targ_condition))(input).to_failure()?;

    Ok((input, MeasureRise { name: name.to_string(), analysis, trig, targ }))
}

/// .MEAS TRAN avgval AVG V(1) FROM=10ns TO=55ns
fn measure_basic_stat<'a>(input: &'a str, name: &'a str, analysis: AnalysisType) -> NomResult<'a, MeasureBasicStat> {
    let (input, stat) = context("stat_function", hws(measure_function))(input)?;
    let (input, variable) = context("variable", hws(output_variable))(input).to_failure()?;
    let (input, from) = context("FROM value", preceded(hws(tag_no_case("FROM=")), hws(time_number)))(input).to_failure()?;
    let (input, to) = context("TO value", preceded(hws(tag_no_case("TO=")), hws(time_number)))(input).to_failure()?;

    Ok((input, MeasureBasicStat { name: name.to_string(), analysis, stat, variable, from, to }))
}

/// .MEAS TRAN DesiredCurr FIND I(Vmeas) WHEN V(1)=1V
fn measure_find_when<'a>(input: &'a str, name: &'a str, analysis: AnalysisType) -> NomResult<'a, MeasureFindWhen> {
    let (input, _) = context("FIND keyword", hws(tag_no_case("FIND")))(input)?;
    let (input, variable) = context("variable", hws(output_variable))(input).to_failure()?;
    let (input, _) = context("WHEN keyword", hws(tag_no_case("WHEN")))(input).to_failure()?;
    let (input, condition) = context("condition", hws(finwhen_condition))(input).to_failure()?;

    Ok((input, MeasureFindWhen { name: name.to_string(), analysis, variable, when: condition }))
}


/// V(1) VAL=.2 RISE=1
fn trig_targ_condition(input: &str) -> NomResult<TrigTargCondition> {
    let (input, variable) = hws(output_variable)(input)?;
    let (input, _) = hws(tag_no_case("VAL="))(input)?;
    let (input, value) = hws(number)(input)?;
    let (input, edge) = hws(alt((
        map(tag_no_case("RISE"), |_| EdgeType::Rise),
        map(tag_no_case("FALL"), |_| EdgeType::Fall),
    )))(input)?;
    let (input, _) = hws(tag("="))(input)?;
    let (input, num) = hws(unsigned_int)(input)?;

    Ok((
        input,
        TrigTargCondition {
            variable,
            value,
            edge,
            number: num as usize,
        },
    ))
}

fn measure_function(input: &str) -> NomResult<MeasureFunction> {
    map(
        hws(alt((
            tag_no_case("AVG"),
            tag_no_case("RMS"),
            tag_no_case("MIN"),
            tag_no_case("MAX"),
            tag_no_case("PP"),
            tag_no_case("DERIV"),
            tag_no_case("INTEGRATE"),
        ))),
        |s: &str| match &s.to_ascii_uppercase()[..] {
            "AVG" => MeasureFunction::Avg,
            "RMS" => MeasureFunction::Rms,
            "MIN" => MeasureFunction::Min,
            "MAX" => MeasureFunction::Max,
            "PP" => MeasureFunction::Pp,
            "DERIV" => MeasureFunction::Deriv,
            "INTEGRATE" => MeasureFunction::Integrate,
            _ => unreachable!(),
        },
    )(input)
}

fn finwhen_condition(input: &str) -> NomResult<FindWhenCondition> {
    let (input, variable) = hws(output_variable)(input)?;
    let (input, _) = hws(tag("="))(input)?;
    let (input, value) = hws(number)(input)?;

    Ok((input, FindWhenCondition { variable, value }))
}

fn output_variable(input: &str) -> NomResult<OutputVariable> {
    let (input, kind) = hws(alt((tag_no_case("V"), tag_no_case("I"))))(input)?;

    let (input, var) = hws(delimited(
        hws(tag("(")),
        take_until(")"),
        tag(")"),
    ))(input)?;

    let suffix = if var.ends_with("M") {
        Some(OutputSuffix::Magnitude)
    } else if var.ends_with("DB") {
        Some(OutputSuffix::Decibel)
    } else if var.ends_with("P") {
        Some(OutputSuffix::Phase)
    } else if var.ends_with("R") {
        Some(OutputSuffix::Real)
    } else if var.ends_with("I") {
        Some(OutputSuffix::Imag)
    } else {
        None
    };

    if kind.eq_ignore_ascii_case("V") {
        let parts = var.split(',').map(|s| s.trim()).collect::<Vec<_>>();
        let node1 = parts.get(0).unwrap_or(&"").to_string();
        let node2 = parts.get(1).map(|s| s.to_string());
        Ok((input, OutputVariable::Voltage { node1, node2, suffix }))
    } else {
        Ok((input, OutputVariable::Current {
            element_name: var.to_string(),
            suffix,
        }))
    }
}

fn analysis_type(input: &str) -> NomResult<AnalysisType> {
    map(
        hws(alt((tag_no_case("TRAN"), tag_no_case("AC"), tag_no_case("DC")))),
        |s: &str| match &s.to_ascii_uppercase()[..] {
            "TRAN" => AnalysisType::Tran,
            "AC" => AnalysisType::Ac,
            "DC" => AnalysisType::Dc,
            _ => unreachable!(),
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use nom::{error::convert_error, Err};

    use super::*;
    use reda_unit::{num, u};

    #[test]
    fn test_measure_rise() {
        let input = ".MEAS TRAN rise1 TRIG V(n1) VAL=0.2 RISE=1 TARG V(n1) VAL=0.8 RISE=1";
        let (_, meas) = measure_command(input).unwrap();

        if let MeasureCommand::Rise(m) = meas {
            assert_eq!(m.name, "rise1");
            assert_eq!(m.analysis, AnalysisType::Tran);
            assert_eq!(m.trig.value, num!(0.2));
            assert_eq!(m.trig.edge, EdgeType::Rise);
            assert_eq!(m.trig.number, 1);
            assert_eq!(m.targ.value, num!(0.8));
            assert_eq!(m.targ.edge, EdgeType::Rise);
            assert_eq!(m.targ.number, 1);
        } else {
            panic!("Expected MeasureCommand::Rise");
        }
    }

    #[test]
    fn test_measure_basic_stat() {
        let input = ".MEAS TRAN avgval AVG V(n1) FROM=10u TO=55u";
        let (_, meas) = measure_command(input).unwrap();

        if let MeasureCommand::BasicStat(m) = meas {
            assert_eq!(m.name, "avgval");
            assert_eq!(m.analysis, AnalysisType::Tran);
            assert_eq!(m.stat, MeasureFunction::Avg);
            assert!(matches!(m.variable, OutputVariable::Voltage { .. }));
            assert_eq!(m.from, u!(10. us));
            assert_eq!(m.to, u!(55. us));
        } else {
            panic!("Expected MeasureCommand::BasicStat");
        }
    }

    #[test]
    fn test_measure_find_when() {
        let input = ".MEAS TRAN DesiredCurr FIND I(Vmeas) WHEN V(n1)=1V";
        let (_, meas) = measure_command(input).unwrap();

        if let MeasureCommand::FindWhen(m) = meas {
            assert_eq!(m.name, "DesiredCurr");
            assert_eq!(m.analysis, AnalysisType::Tran);
            assert!(matches!(m.variable, OutputVariable::Current { .. }));
            assert_eq!(m.when.value, num!(1));
        } else {
            panic!("Expected MeasureCommand::FindWhen");
        }
    }

    #[test]
    fn test_measure_bad_prefix() {
        let input = ".XXX TRAN AVG V(1) FROM=0 TO=1";
        let result = measure_command(input);
        assert!(matches!(result, Err(Err::Error(_))));
    }

    #[test]
    fn test_measure_unknown_type() {
        let input = ".MEAS TRAN BOGUS V(1) FROM=0 TO=1";
        let result = measure_command(input);
        assert!(matches!(result, Err(Err::Failure(_))));
    }

    #[test]
    fn test_measure_stat_missing_to() {
        let input = ".MEAS TRAN result AVG V(1) FROM=1";
        let result = measure_command(input);
        assert!(matches!(result, Err(Err::Failure(_))));
    }

    #[test]
    fn test_measure_rise_missing_targ() {
        let input = ".MEAS TRAN rise TRIG V(1) VAL=.2 RISE=1";
        let result = measure_command(input);
        assert!(matches!(result, Err(Err::Failure(_))));
    }

    #[test]
    fn test_measure_find_when_invalid_condition() {
        let input = ".MEAS TRAN result FIND I(R1) WHEN V(1) == 1";
        let result = measure_command(input);
        assert!(matches!(result, Err(Err::Failure(_))));
    }

    #[test]
    fn test_measure_debug_context() {
        let input = ".MEAS TRAN avgval AVG V(1) FROM=10ns TO=abc";
        let result = measure_command(input);
        if let Err(Err::Failure(e)) = result {
            println!("{}", convert_error(input, e));
        }
    }
}
