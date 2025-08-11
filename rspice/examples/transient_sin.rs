use rspice::{netlist::Circuit, probe::Drawer, simulate::ngspice::NgSpiceShared, SineVoltage, TranCommandBuilder};
use runit::u;

fn main_result() -> Result<(), Box<dyn std::error::Error>> {
    let mut circuit = Circuit::new("Transient sin");
    
    circuit.add_sine_voltage("input", "in", "0", SineVoltage::sin(u!(1 V), u!(100 Hz)));
    circuit.add_resistor("1", "in", "1", u!(3 kΩ));
    circuit.add_resistor("2", "1", "0", u!(1 kΩ));

    let simulate = NgSpiceShared::default()?;
    let mut simulator = circuit.simulator(simulate);
    let command = TranCommandBuilder::default()
        .t_stop(u!(0.02 s))
        .t_step(u!(1 us))
        .build().unwrap();
    let analysis = simulator.run_tran(&command)?;

    analysis.draw_nodes(&Drawer::default(), &["in"], "./images/transient-sin-nodes.png")?;
    analysis.draw_all_branchs(&Drawer::default(), "./images/transient-sin-branchs.png")?;

    Ok(())
}

fn main() {
    if let Err(e) = main_result() {
        eprintln!("{}", e);
    }
}