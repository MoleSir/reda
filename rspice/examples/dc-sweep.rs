use rspice::probe::Drawer;
use rspice::{netlist::Circuit, simulate::ngspice::NgSpiceShared, DcCommandBuilder};
use rspice::{num, u};

fn main_result() -> Result<(), Box<dyn std::error::Error>> {
    let mut cir = Circuit::new("DC Sweep");
    cir.add_dc_voltage("1", "in", "0", num!(0.));
    cir.add_resistor("1", "in", "out", num!(2 k));
    cir.add_resistor("2", "out", "0", num!(1 k));
    
    let shared = NgSpiceShared::default()?;
    let mut simulator = cir.simulator(shared);
    
    let command = DcCommandBuilder::default()
        .src_name("V1")
        .start(num!(0.))
        .stop(num!(5.))
        .step(num!(0.1))
        .build().unwrap();
    
    let analysis = simulator.run_dc_voltage(&command)?;
    println!("{}", analysis.get_voltage_at("out", u!(200 uV)).unwrap());

    analysis.draw_all_branchs(&Drawer::default(), "./images/dc-sweep-branchs.png")?;
    analysis.draw_all_nodes(&Drawer::default(), "./images/dc-sweep-nodes.png")?;

    Ok(())
}

fn main() {
    if let Err(e) = main_result() {
        eprintln!("{}", e);
    }
}