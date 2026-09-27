use reda_gds::GdsLibrary;

fn main_result() -> Result<(), Box<dyn std::error::Error>> {
    let mut lib = GdsLibrary::load_file("./data/leafcell/bitcell.gds")?;
    let sense_amp = GdsLibrary::load_file("./data/leafcell/sense_amp.gds")?;
    let write_driver = GdsLibrary::load_file("./data/leafcell/write_driver.gds")?;
    let column_tri_gate = GdsLibrary::load_file("./data/leafcell/column_tri_gate.gds")?;
    let precharge = GdsLibrary::load_file("./data/leafcell/precharge.gds")?;

    println!("{}", lib.meters_per_dbunit);
    println!("{}", precharge.meters_per_dbunit);
    println!("{}", sense_amp.meters_per_dbunit);
    println!("{}", column_tri_gate.meters_per_dbunit);
    println!("{}", write_driver.meters_per_dbunit);

    lib.structures.insert("precharge".into(), precharge.structures.get("precharge").unwrap().clone());
    lib.structures.insert("sense_amp".into(), sense_amp.structures.get("sense_amp").unwrap().clone());
    lib.structures.insert("column_tri_gate".into(), column_tri_gate.structures.get("column_tri_gate").unwrap().clone());
    lib.structures.insert("write_driver".into(), write_driver.structures.get("write_driver").unwrap().clone());

    println!("{}", lib.structures.len());

    lib.save_gds_file("./data/leafcell/leafcell.gds")?;

    let lib = GdsLibrary::load_file("./data/leafcell/leafcell.gds")?;
    println!("{}", lib.structures.len());
    println!("{}", lib.meters_per_dbunit);

    Ok(())
}

fn main() {
    if let Err(e) = main_result() {
        eprintln!("{}", e);
    }
}