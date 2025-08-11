use crate::AcCurrent;
use crate::AcVoltage;
use crate::BJTBuilder;
use crate::CapacitorBuilder;
use crate::Component;
use crate::DiodeBuilder;
use crate::InductorBuilder;
use crate::Model;
use crate::MosFETBuilder;
use reda_unit::Angle;
use reda_unit::Capacitance;
use reda_unit::Current;
use reda_unit::Inductance;
use reda_unit::Length;
use reda_unit::Resistance;
use reda_unit::Time;
use reda_unit::Voltage;
use crate::PulseVoltage;
use crate::PwlVoltage;
use crate::ResistorBuilder;
use crate::SineVoltage;
use crate::Source;
use crate::SourceValue;
use super::Circuit;

impl Circuit {
    pub fn add_resistor<S1, S2, S3, N>(&mut self, name: S1, node_pos: S2, node_neg: S3, resistance: N) 
    where 
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        N: Into<Resistance>,
    {
        let r = ResistorBuilder::default()
            .name(name)
            .node_neg(node_neg)
            .node_pos(node_pos)
            .resistance(resistance).build().unwrap();
        self.components.push(Component::R(r));
    }

    pub fn add_capacitor<S1, S2, S3, N>(&mut self, name: S1, node_pos: S2, node_neg: S3, capacitance: N) 
    where 
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        N: Into<Capacitance>,
    {
        let c = CapacitorBuilder::default()
            .name(name)
            .node_neg(node_neg)
            .node_pos(node_pos)
            .capacitance(capacitance).build().unwrap();
        self.components.push(Component::C(c));
    }

    pub fn add_inductor<S1, S2, S3, N>(&mut self, name: S1, node_pos: S2, node_neg: S3, inductance: N) 
    where 
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        N: Into<Inductance>,
    {
        let l = InductorBuilder::default()
            .name(name)
            .node_neg(node_neg)
            .node_pos(node_pos)
            .inductance(inductance).build().unwrap();
        self.components.push(Component::L(l));
    }

    pub fn add_diode<S1, S2, S3>(&mut self, name: S1, node_p: S2, node_n: S3, model_name: S1)
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
    {
        let d = DiodeBuilder::default()
            .name(name)
            .node_pos(node_p)
            .node_neg(node_n)
            .model_name(model_name)
            .build()
            .unwrap();
        self.components.push(Component::D(d));
    }

    pub fn add_bjt<S1, S2, S3, S4, S5>(&mut self, name: S1, collector: S2, base: S3, emitter: S4, model_name: S5)
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        S4: Into<String>,
        S5: Into<String>,
    {
        let q = BJTBuilder::default()
            .name(name)
            .collector(collector)
            .base(base)
            .emitter(emitter)
            .model_name(model_name)
            .build()
            .unwrap();
        self.components.push(Component::Q(q));
    }

    pub fn add_mosfet<S1, S2, S3, S4, S5, S6, N1, N2>(
        &mut self,
        name: S1,
        drain: S2,
        gate: S3,
        source: S4,
        bulk: S5,
        model_name: S6,
        length: N1,
        width: N2,
    )
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        S4: Into<String>,
        S5: Into<String>,
        S6: Into<String>,
        N1: Into<Length>,
        N2: Into<Length>,
    {
        let m = MosFETBuilder::default()
            .name(name)
            .drain(drain)
            .gate(gate)
            .source(source)
            .bulk(bulk)
            .model_name(model_name)
            .length(length)
            .width(width)
            .build()
            .unwrap();
        self.components.push(Component::M(m));
    }

    pub fn add_dc_voltage<S1, S2, S3, N>(&mut self, name: S1, node_p: S2, node_n: S3, value: N)
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        N: Into<Voltage>,
    {
        self.sources.push(Source {
            name: name.into(),
            node_pos: node_p.into(),
            node_neg: node_n.into(),
            value: SourceValue::DcVoltage(value.into()),
        });
    }

    pub fn add_dc_current<S1, S2, S3, N>(&mut self, name: S1, node_p: S2, node_n: S3, value: N)
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        N: Into<Current>,
    {
        self.sources.push(Source {
            name: name.into(),
            node_pos: node_p.into(),
            node_neg: node_n.into(),
            value: SourceValue::DcCurrent(value.into()),
        });
    }

    pub fn add_ac_voltage<S1, S2, S3, N1, N2>(&mut self, name: S1, node_p: S2, node_n: S3, magnitude: N1, phase_deg: N2)
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        N1: Into<Voltage>,
        N2: Into<Angle>,
    {
        self.sources.push(Source {
            name: name.into(),
            node_pos: node_p.into(),
            node_neg: node_n.into(),
            value: SourceValue::AcVoltage(AcVoltage {
                magnitude: magnitude.into(),
                phase_deg: phase_deg.into(),
            }),
        });
    }

    pub fn add_ac_current<S1, S2, S3, N1, N2>(&mut self, name: S1, node_p: S2, node_n: S3, magnitude: N1, phase_deg: N2)
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        N1: Into<Current>,
        N2: Into<Angle>,
    {
        self.sources.push(Source {
            name: name.into(),
            node_pos: node_p.into(),
            node_neg: node_n.into(),
            value: SourceValue::AcCurrent(AcCurrent {
                magnitude: magnitude.into(),
                phase_deg: phase_deg.into(),
            }),
        });
    }

    pub fn add_sine_voltage<S1, S2, S3>(&mut self, name: S1, node_p: S2, node_n: S3, voltage: SineVoltage) 
        where
            S1: Into<String>,
            S2: Into<String>,
            S3: Into<String>,
        {
            self.sources.push(Source {
                name: name.into(),
                node_pos: node_p.into(),
                node_neg: node_n.into(),
                value: SourceValue::Sin(voltage)
            });
        }

    pub fn add_pwl_voltage<S1, S2, S3>(&mut self, name: S1, node_p: S2, node_n: S3, points: Vec<(Time, Voltage)>)
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
    {
        self.sources.push(Source {
            name: name.into(),
            node_pos: node_p.into(),
            node_neg: node_n.into(),
            value: SourceValue::Pwl(PwlVoltage { points }),
        });
    }

    pub fn add_pulse_voltage<S1, S2, S3>(&mut self, name: S1, node_p: S2, node_n: S3, voltage: PulseVoltage) 
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
    {
        self.sources.push(Source {
            name: name.into(),
            node_pos: node_p.into(),
            node_neg: node_n.into(),
            value: SourceValue::Pulse(voltage)
        });
    }

    pub fn add_model(&mut self, model: Model) {
        self.models.push(model);
    }
}

