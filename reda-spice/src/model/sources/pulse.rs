use derive_builder::Builder;
use reda_unit::{u, Time, Voltage};

#[derive(Debug, Clone, Builder)]
pub struct PulseVoltage {
    pub v0: Voltage,      // 初始电压 Vo
    pub v1: Voltage,      // 峰值电压 V1
    pub delay: Time,      // Td 初始延迟
    pub rise: Time,       // Tr 上升时间
    pub fall: Time,       // Tf 下降时间
    pub width: Time,      // Tw 脉宽
    pub period: Time,     // To 周期
}

impl PulseVoltage {
    pub fn clock(vdd: Voltage, period: Time, slew: Time) -> Self {
        Self {
            v0: u!(0 V),
            v1: vdd,
            delay: u!(0 s),
            rise: slew,
            fall: slew,
            width: (period - 2. * slew) / 2.,
            period
        }
    }

    pub fn voltage_at(&self, time: Time) -> Voltage {
        if time <= self.delay {
            return self.v0;
        } 

        let t_in_cycle = (time - self.delay) % self.period;

        let delta_v = self.v1 - self.v0;
        match t_in_cycle {
            t if t.value() < 0.0 => self.v0,
            t if t < self.rise => {
                // In rise
                self.v0 + delta_v * (t / self.rise)
            }
            t if t < (self.rise + self.width) => {
                // high
                self.v1
            }
            t if t < (self.rise + self.width + self.fall) => {
                // In fall
                self.v1 - delta_v * ((t - self.rise - self.width) / self.fall)
            }
            _ => self.v0
        }
    }

    pub fn to_spice(&self) -> String {
        format!(
            "PULSE({} {} {} {} {} {} {})",
            self.v0,
            self.v1,
            self.delay,
            self.rise,
            self.fall,
            self.width,
            self.period
        )
    }
}