use reda_gds::GdsLibrary;

fn main_result() -> Result<(), Box<dyn std::error::Error>> {
    let lib = GdsLibrary::load_file("./data/cells/dff.gds")?;
    eprintln!("{}", lib.name);

    let cell_1rw = GdsLibrary::load_file("./data/cells/cell_1rw.gds")?;
    eprintln!("{}", cell_1rw.create_date);

    lib.save_gds_file("./temp/dff.gds")?;

    Ok(())
}

fn main() {
    if let Err(e) = main_result() {
        eprintln!("{}", e);
    }
}