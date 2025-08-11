use rspice::{netlist::Circuit, probe::Drawer, simulate::ngspice::NgSpiceShared, AcCommand};
use runit::u;

/// https://pyspice.fabrice-salvaire.fr/releases/v1.4/examples/filter/low-pass-rc-filter.html
fn main_result() -> Result<(), Box<dyn std::error::Error>> {
    let mut circuit = Circuit::new("Low-Pass RC Filter");
    
    circuit.add_ac_voltage("input", "in", "0", 1., 0.);
    circuit.add_resistor("1", "in", "out", u!(1 kΩ));
    circuit.add_capacitor("1", "out", "0", u!(1 uF));

    let simulate = NgSpiceShared::default()?;
    let mut simulator = circuit.simulator(simulate);
    let analysis = simulator.run_ac(&AcCommand::dec(10, u!(1 Hz), u!(1 MHz)))?;
    
    let mut drawer = Drawer::default();
    drawer.height = 360;
    analysis.draw_gain(&drawer, "in", "out", "./images/low-pass_rc_filter-gain.png")?;
    analysis.draw_phase(&drawer, "in", "out", "./images/low-pass_rc_filter-phase.png")?;
    
    Ok(())
}

fn main() {
    if let Err(e) = main_result() {
        eprintln!("{}", e);
    }
}