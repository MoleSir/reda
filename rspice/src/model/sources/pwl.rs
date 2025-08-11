use runit::{Time, Voltage};

#[derive(Debug, Clone)]
pub struct PwlVoltage {
    pub points: Vec<(Time, Voltage)>,
}

impl PwlVoltage {
    pub fn voltage_at(&self, time: Time) -> Voltage {
        let n = self.points.len();
        if n == 0 {
            return 0.0.into();
        }

        if n == 1 {
            return self.points[0].1;
        }

        for i in 0..n - 1 {
            let (t0, v0) = self.points[i];
            let (t1, v1) = self.points[i + 1];

            if t0 <= time && time <= t1 {
                let ratio = (time - t0) / (t1 - t0);
                return v0 + ratio * (v1 - v0)
            }
        }

        return self.points.last().unwrap().1
    }

    pub fn to_spice(&self) -> String {
        let mut line = format!("PWL(");
        for (i, (t, v)) in self.points.iter().enumerate() {
            if i > 0 {
                line.push(' ');
            }
            line.push_str(&format!("{} {}", t, v));
        }
        line.push(')');
        line
    }
}
