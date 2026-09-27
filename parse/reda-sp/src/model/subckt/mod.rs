use derive_builder::Builder;
use reda_unit::{Capacitance, Inductance, Length, Resistance};

use super::{BJTBuilder, CapacitorBuilder, Component, DiodeBuilder, InductorBuilder, MosFETBuilder, ResistorBuilder, ToSpice};

#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct Subckt {
    pub name: String,                  // 子电路名
    pub ports: Vec<String>,            // 接口节点列表（按顺序）

    #[builder(default)]
    pub components: Vec<Component>,     // 子电路内部元件
    #[builder(default)]
    pub instances: Vec<Instance>,      // 实例化
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub name: String,                 // 实例名称，如 XU1
    pub pins: Vec<String>,            // 接口节点连接到外部的节点
    pub subckt_name: String,          // 被实例化的子电路名称
}

impl Subckt {
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
}

impl ToSpice for Subckt {
    fn to_spice(&self) -> String {
        let mut lines = vec![];
        lines.push(format!(
            ".SUBCKT {} {}\n",
            self.name,
            self.ports.join(" ")
        ));

        for c in self.components.iter() {
            lines.push(c.to_spice());
        }

        for i in self.instances.iter() {
            lines.push(i.to_spice());
        }

        lines.push(format!(".ENDS {}", self.name));

        lines.join("\n")
    }
}

impl ToSpice for Instance {
    fn to_spice(&self) -> String {
        format!(
            "X{} {} {}",
            self.name,
            self.pins.join(" "),
            self.subckt_name
        )
    }
}