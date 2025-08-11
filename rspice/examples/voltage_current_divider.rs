use rspice::{netlist::Circuit, simulate::ngspice::NgSpiceShared};
use runit::u;

/// https://pyspice.fabrice-salvaire.fr/releases/v1.4/examples/fundamental-laws/voltage-current-divider.html
fn main_result() -> Result<(), Box<dyn std::error::Error>> {
    let mut circuit = Circuit::new("Voltage Divider");
    circuit.add_dc_voltage("input", "1", "0", u!(10 V));
    circuit.add_resistor("1", "1", "2", u!(2 kΩ));
    circuit.add_resistor("2", "2", "0", u!(1 kΩ));

    let simulate = NgSpiceShared::default()?;
    let mut simulator = circuit.simulator(simulate);
    let analysis = simulator.run_op()?;

    for (name, voltage) in analysis.nodes.iter() {
        println!("Node: {}: {:.3}", name, voltage);
    }

    Ok(())
}

fn main() {
    if let Err(e) = main_result() {
        eprintln!("{}", e);
    }
}