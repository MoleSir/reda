use rspice::{netlist::Circuit, simulate::ngspice::NgSpiceShared};
use runit::u;

/// https://pyspice.fabrice-salvaire.fr/releases/v1.4/examples/fundamental-laws/millman-theorem.html
fn main_result() -> Result<(), Box<dyn std::error::Error>> {
    let mut circuit = Circuit::new("Millman's theorem");
    let number_of_branches = 3;
    for i in 1..number_of_branches+1 {
        circuit.add_dc_voltage(
            format!("input{}", i), 
            format!("{}", i), 
            "0", 
            u!(i V),
        );
        circuit.add_resistor(
            format!("{}", i), 
            format!("{}", i), 
            "A",
            u!(i kΩ));
    }

    let simulate = NgSpiceShared::default()?;
    let mut simulator = circuit.simulator(simulate);
    let analysis = simulator.run_op()?;

    let node_a = analysis.get_node("A").unwrap();
    println!("Node {}: {}", "A", node_a);

    Ok(())
}

fn main() {
    if let Err(e) = main_result() {
        eprintln!("{}", e);
    }
}