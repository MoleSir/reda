use rspice::{netlist::Circuit, probe::Drawer, simulate::ngspice::NgSpiceShared, Model, SineVoltage, TranCommandBuilder};
use runit::{num, u};

/// https://pyspice.fabrice-salvaire.fr/releases/v1.4/examples/transistor/ac-coupled-amplifier.html
fn main_result() -> Result<(), Box<dyn std::error::Error>> {
    let mut circuit = Circuit::new("AC Coupled Amplifier");
    
    circuit.add_dc_voltage("power", "5", "0", u!(15 V));
    let sine = SineVoltage::sin(u!(0.5 V), u!(1 kHz));
    let period = sine.period();
    circuit.add_sine_voltage("in", "in", "0", sine);
    circuit.add_capacitor("1", "in", "2", u!(10 uF));
    circuit.add_resistor("1", "5", "2", u!(100 kΩ));
    circuit.add_resistor("2", "2", "0", u!(20 kΩ));
    circuit.add_resistor("C", "5", "4", u!(10 kΩ));
    circuit.add_bjt("1", "4", "2", "3", "bjt");
    let mut model = Model::npn("bjt");
    model.parameter("bf", 80.0);
    model.parameter("cjc", num!(5 p));
    model.parameter("rb", 100);
    circuit.add_model(model);
    circuit.add_resistor("E", "3", "0", u!(2 kΩ));
    circuit.add_capacitor("2", "4", "out", u!(10 uF));
    circuit.add_resistor("Load", "out", "0", u!(1 kΩ));

    let simulate = NgSpiceShared::default()?;
    let mut simulator = circuit.simulator(simulate);
    let command = TranCommandBuilder::default()
        .t_stop(2. * period)
        .t_step(u!(5 us))
        .build().unwrap();
    let analysis = simulator.run_tran(&command)?;

    analysis.draw_nodes(&Drawer::default(), &["in", "out"], "./images/ac-coupled-amplifier-nodes.png")?;
    analysis.draw_all_branchs(&Drawer::default(), "./images/ac-coupled-amplifier-branchs.png")?;

    Ok(())
}

fn main() {
    if let Err(e) = main_result() {
        eprintln!("{}", e);
    }
}