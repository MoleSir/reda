use rspice::{netlist::Circuit, simulate::ngspice::NgSpiceShared};
use runit::u;

/// https://pyspice.fabrice-salvaire.fr/releases/v1.4/examples/fundamental-laws/th%C3%A9venin-norton-theorem.html
fn main_result() -> Result<(), Box<dyn std::error::Error>> {
    let mut circuit = Circuit::new("Thévenin Representation");
    circuit.add_dc_voltage("input", "1", "0", u!(10 V));
    circuit.add_resistor("generator", "1", "load", u!(10 Ω));
    circuit.add_resistor("load", "load", "0", u!(1 kΩ));

    let simulate = NgSpiceShared::default()?;
    let mut simulator = circuit.simulator(simulate);
    let analysis = simulator.run_op()?;

    let node_a = analysis.get_node("load").unwrap();
    println!("Node {}: {}", "load", node_a);

    Ok(())
}

fn main() {
    if let Err(e) = main_result() {
        eprintln!("{}", e);
    }
}