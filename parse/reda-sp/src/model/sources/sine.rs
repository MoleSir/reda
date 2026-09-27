use core::f64;
use derive_builder::Builder;
use reda_unit::{num, Frequency, Number, Time, Voltage, u};

#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct SineVoltage {
    pub vo: Voltage,
    pub va: Voltage,
    pub freq_hz: Frequency,
    #[builder(default = "u!(0 s)")]
    pub delay: Time,
    #[builder(default = "u!(0 hz)")]
    pub damping: Frequency,
    #[builder(default = "num!(0)")]
    pub phase_deg: Number,
}

impl SineVoltage {
    // F(t) = A sin(2pi f t)
    pub fn sin(amplitude: Voltage, frequency: Frequency) -> Self {
        SineVoltageBuilder::default()
            .vo(u!(0 V))
            .va(amplitude)
            .freq_hz(frequency)
            .build().unwrap()
    }

    // F(t) = A cos(2pi f t)
    pub fn cos(amplitude: Voltage, frequency: Frequency) -> Self {
        let period = frequency.to_period();
        SineVoltageBuilder::default()
            .vo(u!(0 V))
            .va(amplitude)
            .freq_hz(frequency)
            .delay(-0.25 * period)
            .build().unwrap()
    }

    pub fn period(&self) -> Time {
        self.freq_hz.to_period()
    }

    pub fn voltage_at(&self, time: Time) -> Voltage {
        if time < self.delay {
            return self.vo; 
        }

        let td = time - self.delay;
        let envelope = (-self.damping * td).exp();
        let omega = (2.0 * f64::consts::PI) * self.freq_hz;
        let sine = (omega * td + self.phase_deg / 360.).sin();

        self.vo + self.va * envelope * sine
    }

    pub fn to_spice(&self) -> String {
        format!(
            "SIN({} {} {} {} {} {})",
            self.vo,
            self.va,
            self.freq_hz,
            self.delay,
            self.damping,
            self.phase_deg
        )
    }
}
