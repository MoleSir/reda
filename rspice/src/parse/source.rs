use nom::{bytes::complete::tag_no_case, error::{context, VerboseError, VerboseErrorKind}};
use nom::character::complete::char;
use nom::combinator::{opt, map};
use nom::branch::alt;
use runit::{num, u, Current, Voltage};
use crate::{model::{AcCurrent, PulseVoltage, PwlVoltage, SineVoltage, Source, SourceKind, SourceValue}, AcVoltage};
use super::{angle_number, current_number, frequency_number, hws, identifier, node, number, time_number, voltage_number, NomResult, ToFailure};

/// I/V<name> pos neg <value>
pub fn source(input: &str) -> NomResult<Source> {
    context("source", |input| {
        let (input, name) = context("name", hws(identifier))(input)?;
        let first_char = name.chars().next().unwrap_or_default().to_ascii_uppercase();

        let src_kind = if first_char == 'V' {
            SourceKind::Voltage
        } else if first_char == 'I' {
            SourceKind::Current
        } else {
            return Err(nom::Err::Error(VerboseError {
                errors: [(input, VerboseErrorKind::Context("Source must begin with V or I"))].into(),
            }));
        };

        let (input, node_pos) = context("node_pos", hws(node))(input).to_failure()?;
        let (input, node_neg) = context("node_neg", hws(node))(input).to_failure()?;
        let (input, value) = context("source_value", hws(move |i| source_value(i, src_kind)))(input).to_failure()?;

        Ok((
            input,
            Source {
                name: name[1..].to_string(),
                node_pos: node_pos.to_string(),
                node_neg: node_neg.to_string(),
                value,
            },
        ))
    })(input)
}

pub fn source_value(input: &str, kind: SourceKind) -> NomResult<SourceValue> {
    match kind {
        SourceKind::Voltage => context("source_value", alt((
            map(dc_voltage, SourceValue::DcVoltage),
            map(ac_voltage, SourceValue::AcVoltage),
            map(sine_voltage, SourceValue::Sin),
            map(pwl_voltage, SourceValue::Pwl),
            map(pulse_voltage, SourceValue::Pulse),
        )))(input),
        SourceKind::Current => context("source_value", alt((
            map(dc_current, SourceValue::DcCurrent),
            map(ac_current, SourceValue::AcCurrent),
            map(sine_voltage, SourceValue::Sin),
            map(pwl_voltage, SourceValue::Pwl),
            map(pulse_voltage, SourceValue::Pulse),
        )))(input)
    }
}

pub fn dc_voltage(input: &str) -> NomResult<Voltage> {
    context("dc", |input| {
        let (input, opt_result) = opt(alt((
            hws(tag_no_case("DC=")),
            hws(tag_no_case("DC")),
        )))(input)?;

        let (input, value) = if opt_result.is_some() {
            hws(voltage_number)(input).to_failure()?
        } else {
            hws(voltage_number)(input)?
        };

        Ok((input, value))
    })(input)
}

pub fn dc_current(input: &str) -> NomResult<Current> {
    context("dc", |input| {
        let (input, opt_result) = opt(alt((
            hws(tag_no_case("DC=")),
            hws(tag_no_case("DC")),
        )))(input)?;

        let (input, value) = if opt_result.is_some() {
            hws(current_number)(input).to_failure()?
        } else {
            hws(current_number)(input)?
        };

        Ok((input, value))
    })(input)
}

pub fn ac_voltage(input: &str) -> NomResult<AcVoltage> {
    context("ac voltage", |input| {
        let (input, opt_result) = opt(alt((
            hws(tag_no_case("AC=")),
            hws(tag_no_case("AC")),
        )))(input)?;

        let (input, (magnitude, phase_deg)) = if opt_result.is_some() {
            let (input, magnitude) = hws(voltage_number)(input).to_failure()?;
            let (input, phase_deg) = hws(angle_number)(input).to_failure()?;
            (input, (magnitude, phase_deg))
        } else {
            let (input, magnitude) = hws(voltage_number)(input)?;
            let (input, phase_deg) = hws(angle_number)(input)?;
            (input, (magnitude, phase_deg))
        };

        Ok((input, AcVoltage { magnitude, phase_deg }))
    })(input)
} 

pub fn ac_current(input: &str) -> NomResult<AcCurrent> {
    context("ac current", |input| {
        let (input, opt_result) = opt(alt((
            hws(tag_no_case("AC=")),
            hws(tag_no_case("AC")),
        )))(input)?;

        let (input, (magnitude, phase_deg)) = if opt_result.is_some() {
            let (input, magnitude) = hws(current_number)(input).to_failure()?;
            let (input, phase_deg) = hws(angle_number)(input).to_failure()?;
            (input, (magnitude, phase_deg))
        } else {
            let (input, magnitude) = hws(current_number)(input)?;
            let (input, phase_deg) = hws(angle_number)(input)?;
            (input, (magnitude, phase_deg))
        };

        Ok((input, AcCurrent { magnitude, phase_deg }))
    })(input)
} 

pub fn sine_voltage(input: &str) -> NomResult<SineVoltage> {
    context("SIN", |input| {
        let (input, _) = hws(tag_no_case("SIN"))(input)?;
        let (input, _) = hws(char('('))(input).to_failure()?;

        let (input, vo) = context("vo", hws(voltage_number))(input).to_failure()?;
        let (input, va) = context("va", hws(voltage_number))(input).to_failure()?;
        let (input, freq_hz) = context("freq", hws(frequency_number))(input).to_failure()?;
        let (input, delay) = context("td", opt(hws(time_number)))(input).to_failure()?;
        let (input, damping) = context("a", opt(hws(frequency_number)))(input).to_failure()?;
        let (input, phase_deg) = context("phase", opt(hws(number)))(input).to_failure()?;
        let (input, _) = hws(char(')'))(input).to_failure()?;

        Ok((
            input,                
            SineVoltage {
                vo,
                va,
                freq_hz,
                delay: delay.unwrap_or(u!(0.0 s)),
                damping: damping.unwrap_or(u!(0.0 hz)),
                phase_deg: phase_deg.unwrap_or(num!(0.0)),
            },
        ))
    })(input)
}

pub fn pwl_voltage(input: &str) -> NomResult<PwlVoltage> {
    context("PWL", |input| {
        let (input, _) = hws(tag_no_case("PWL"))(input)?;
        let (input, _) = hws(char('('))(input).to_failure()?;

        let mut points = Vec::new();
        let mut input = input;

        loop {
            let (i, t) = hws(time_number)(input).to_failure()?;
            let (i, v) = hws(voltage_number)(i).to_failure()?;
            points.push((t, v));
            input = i;

            let (i, end) = opt(hws(char(')')))(input).to_failure()?;
            if end.is_some() {
                input = i;
                break;
            }
        }

        Ok((input, PwlVoltage { points }))
    })(input)
}

pub fn pulse_voltage(input: &str) -> NomResult<PulseVoltage> {
    context("PULSE", |input| {
        let (input, _) = hws(tag_no_case("PULSE"))(input)?;
        let (input, _) = hws(char('('))(input).to_failure()?;

        let (input, v0)     = context("v0", hws(voltage_number))(input).to_failure()?;
        let (input, v1)     = context("v1", hws(voltage_number))(input).to_failure()?;
        let (input, delay)  = context("td", hws(time_number))(input).to_failure()?;
        let (input, rise)   = context("tr", hws(time_number))(input).to_failure()?;
        let (input, fall)   = context("tf", hws(time_number))(input).to_failure()?;
        let (input, width)  = context("tw", hws(time_number))(input).to_failure()?;
        let (input, period) = context("to", hws(time_number))(input).to_failure()?;
        let (input, _)      = hws(char(')'))(input).to_failure()?;

        Ok((
            input,
            PulseVoltage {
                v0,
                v1,
                delay,
                rise,
                fall,
                width,
                period,
            },
        ))
    })(input)
}

#[allow(unused)]
#[cfg(test)]
mod test {
    use nom::Err;

    use runit::{num, u};
    use super::*;

    #[test]
    fn test_dc_source_voltage() {
        let (_, s) = dc_voltage("DC 5V").unwrap();
        assert_eq!(s, u!(5.0 V));
    }

    #[test]
    fn test_dc_source_current_no_keyword() {
        let (_, s) = dc_current("1.2mA").unwrap();
        assert_eq!(s, u!(1.2 mA));
    }

    #[test]
    fn test_dc_source_equals_syntax() {
        let (_, s) = dc_current("DC=1.0").unwrap();
        assert_eq!(s, u!(1.0 A));
    }

    #[test]
    fn test_ac_voltage_source() {
        let (_, src) = ac_voltage("AC 1.0 90").unwrap();
        assert_eq!(src.magnitude, u!(1.0 V));
        assert_eq!(src.phase_deg, u!(90.0 rad));
    }
    
    #[test]
    fn test_ac_current_source_with_ac_eq() {
        let (_, src) = ac_current("AC=2.5 180").unwrap();
        assert_eq!(src.magnitude, u!(2.5 A));
        assert_eq!(src.phase_deg, u!(180.0 rad));
    }

    #[test]
    fn test_sine_voltage_basic() {
        let (_, s) = sine_voltage("SIN(0 1 1k)").unwrap();
        assert_eq!(s.freq_hz, u!(1.0 kHz));
        assert_eq!(s.phase_deg, num!(0.0)); // default
    }
    
    #[test]
    fn test_pwl_voltage_points() {
        let (_, s) = pwl_voltage("PWL(0 0 1n 1.8 2n 0)").unwrap();
        assert_eq!(s.points.len(), 3);
    }
    
    #[test]
    fn test_pulse_voltage() {
        let (_, s) = pulse_voltage("PULSE(0 1 1n 1n 1n 10n 20n)").unwrap();
        assert_eq!(s.v1, u!(1.0 V));
        assert_eq!(s.width, u!(10.0 ns));
    }

    #[test]
    fn test_source_sine() {
        let (_, src) = source("Vsig n1 0 SIN(0 1 1k 0.1 0.05 45)").unwrap();
        assert_eq!(src.name, "sig");
        match src.value {
            SourceValue::Sin(s) => {
                assert_eq!(s.freq_hz, u!(1.0 kHz));
                assert_eq!(s.phase_deg, num!(45.0));
            },
            _ => panic!("Expected sine voltage"),
        }
    }
    
    #[test]
    fn test_source_dc_voltage() {
        let (_, src) = source("Vdd vdd 0 1.8").unwrap();
        match src.value {
            SourceValue::DcVoltage(dc) => {
                assert_eq!(dc, u!(1.8 V));
            },
            _ => panic!("Expected DC source"),
        }
    }
    
    #[test]
    fn test_source_ac_current() {
        let (_, src) = source("Iin in 0 AC 2.0 180").unwrap();
        match src.value {
            SourceValue::AcCurrent(ac) => {
                assert_eq!(ac.magnitude, u!(2.0 A));
            },
            _ => panic!("Expected AC source"),
        }
    }
    
    #[test]
    fn test_source_bad_prefix() {
        let input = "X1 0 N001 5";
        let res = source(input);
        assert!(matches!(res, Err(Err::Error(_))));
    }
    
    #[test]
    fn test_source_missing_value() {
        let input = "V1 0 N001";
        let res = source(input);
        assert!(matches!(res, Err(Err::Failure(_))));
    }
    
    #[test]
    fn test_source_invalid_sin_format() {
        let input = "V1 0 N001 SIN(1 0.5)"; // 缺频率参数
        let res = source(input);
        assert!(matches!(res, Err(Err::Failure(_))));
    }
    
    #[test]
    fn test_source_invalid_pwl_point() {
        let input = "I1 N1 N2 PWL(0 1 2)"; // 少一个值（奇数个参数）
        let res = source(input);
        assert!(matches!(res, Err(Err::Failure(_))));
    }
    
}