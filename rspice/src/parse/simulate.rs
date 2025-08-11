use nom::error::{context, VerboseError, VerboseErrorKind};
use nom::Err;
use nom::{branch::alt, bytes::complete::tag_no_case, combinator::opt};
use nom::combinator::map;
use crate::model::{AcCommand, AcSweepType, DcCommand, SimCommand, TranCommand};
use super::{frequency_number, hws, identifier, time_number, unsigned_int, voltage_number, NomResult, ToFailure};

pub fn sim_command(input: &str) -> NomResult<SimCommand> {
    alt((
        context("dc_command", map(dc_command, SimCommand::Dc)),
        context("ac_command", map(ac_command, SimCommand::Ac)),
        context("tran_command", map(tran_command, SimCommand::Tran)),
    ))(input)
}


/// .DC SRCname START STOP STEP
pub fn dc_command(input: &str) -> NomResult<DcCommand> {
    context("dc_command", |input| {
        let (input, _) = context("keyword", hws(tag_no_case(".DC")))(input)?;
        let (input, src_name) = context("source_name", hws(identifier))(input).to_failure()?;
        let (input, start) = context("start_value", hws(voltage_number))(input).to_failure()?;
        let (input, stop) = context("stop_value", hws(voltage_number))(input).to_failure()?;
        let (input, step) = context("step_value", hws(voltage_number))(input).to_failure()?;

        Ok((input, DcCommand {
            src_name: src_name.to_string(),
            start,
            stop,
            step,
        }))
    })(input)
}


/// .AC LIN NP FSTART FSTOP
pub fn ac_command(input: &str) -> NomResult<AcCommand> {
    context("ac_command", |input| {
        let (input, _) = context("keyword", hws(tag_no_case(".AC")))(input)?;
        let (input, sweep_type_str) = context("sweep_type", hws(alt((
            tag_no_case("LIN"),
            tag_no_case("DEC"),
            tag_no_case("OCT"),
        ))))(input).to_failure()?;
        let sweep_type = match &sweep_type_str.to_ascii_uppercase()[..] {
            "LIN" => AcSweepType::Lin,
            "DEC" => AcSweepType::Dec,
            "OCT" => AcSweepType::Oct,
            _ => unreachable!(),
        };

        let (input, points) = context("points", hws(unsigned_int))(input).to_failure()?;
        let (input, f_start) = context("f_start", hws(frequency_number))(input).to_failure()?;
        let (input, f_stop) = context("f_stop", hws(frequency_number))(input).to_failure()?;

        Ok((input, AcCommand {
            sweep_type,
            points: points as usize,
            f_start,
            f_stop,
        }))
    })(input)
}


/// .TRAN TSTEP TSTOP <TSTART <TMAX>> <UIC>
pub fn tran_command(input: &str) -> NomResult<TranCommand> {
    context("tran_command", |input| {
        let (input, _) = context("keyword", hws(tag_no_case(".TRAN")))(input)?;
        let (input, t_step) = context("t_step", hws(time_number))(input).to_failure()?;
        let (input, t_stop) = context("t_stop", hws(time_number))(input).to_failure()?;
        let (input, t_start) = context("t_start", opt(hws(time_number)))(input).to_failure()?;
        let (input, t_max) = context("t_max", opt(hws(time_number)))(input).to_failure()?;
        let (input, uic) = context("UIC_flag", opt(hws(identifier)))(input).to_failure()?;
        if let Some(uic) = uic {
            if !uic.eq_ignore_ascii_case("UIC") {
                return Err(Err::Failure(VerboseError {
                    errors: [(input, VerboseErrorKind::Context("expected UIC or end of line"))].into(),
                }));
            }
        }

        Ok((input, TranCommand {
            t_step,
            t_stop,
            t_start,
            t_max,
            uic: uic.is_some(),
        }))
    })(input)
}

#[allow(unused)]
#[cfg(test)]
mod test {
    use nom::{error::convert_error, Err};

    use runit::{num, u};
    use super::*;

    #[test]
    fn test_dc_command() {
        let (_, dc) = dc_command(".DC V1 0 5 0.1").unwrap();
        assert_eq!(dc.src_name, "V1");
        assert_eq!(dc.step, u!(0.1 V));
    }

    #[test]
    fn test_ac_command() {
        let (_, ac) = ac_command(".AC DEC 10 1 1000").unwrap();
        assert_eq!(ac.sweep_type, AcSweepType::Dec);
        assert_eq!(ac.f_stop, u!(1000.0 Hz));
    }

    #[test]
    fn test_tran_command_with_uic() {
        let (_, tran) = tran_command(".TRAN 1n 100n 0 10n UIC").unwrap();
        assert_eq!(tran.t_step, u!(1. ns));
        assert_eq!(tran.t_max, Some(u!(10. ns)));
        assert!(tran.uic);
    }

    #[test]
    fn test_dc_command_invalid_number() {
        let input = ".DC V1 0 5 xyz";
        let result = sim_command(input);
        if let Err(Err::Failure(e)) = result.clone() {
            println!("{}", convert_error(input, e));
        }
        assert!(matches!(result, Err(Err::Failure(_))));
    }

    #[test]
    fn test_ac_command_bad_sweep() {
        let input = ".AC XXX 10 1k 10k";
        let result = sim_command(input);
        assert!(matches!(result, Err(Err::Failure(_))));
    }

    #[test]
    fn test_tran_command_missing_stop() {
        let input = ".TRAN 0.1n";
        let result = sim_command(input);
        assert!(matches!(result, Err(Err::Failure(_))));
    }

    #[test]
    fn test_tran_command_invalid_uic() {
        let input = ".TRAN 1n 10n 0n 1n unknownflag";
        let result = sim_command(input);
        if let Err(Err::Failure(e)) = &result {
            println!("{}", convert_error(input, e.clone()));
        }
        assert!(matches!(result, Err(Err::Failure(_))));
    }
}