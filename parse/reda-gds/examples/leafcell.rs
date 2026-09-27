use reda_gds::GdsLibrary;

#[allow(unused)]
fn main_result() -> Result<(), Box<dyn std::error::Error>> {
    let precharge = GdsLibrary::load_file("./data/leafcell/precharge.gds")?;
    let sense_amp = GdsLibrary::load_file("./data/leafcell/sense_amp.gds")?;
    let column_tri_gate = GdsLibrary::load_file("./data/leafcell/column_tri_gate.gds")?;
    let write_driver = GdsLibrary::load_file("./data/leafcell/write_driver.gds")?;

    Ok(())
}

fn main() {
    if let Err(e) = main_result() {
        eprintln!("{}", e);
    }
}