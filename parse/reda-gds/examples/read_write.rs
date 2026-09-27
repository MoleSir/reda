use reda_gds::GdsLibrary;

fn main_result() -> Result<(), Box<dyn std::error::Error>> {
    let lib1 = GdsLibrary::load_file("./data/cells/dff.gds")?;
    // lib1.save_gds_file("./data/cells/dff_copy.gds")?;
    let lib2 = GdsLibrary::load_file("./data/cells/dff_copy.gds")?;

    println!("{}", lib1.meters_per_dbunit);
    println!("{}", lib2.meters_per_dbunit);
    Ok(())
}

fn main() {
    if let Err(e) = main_result() {
        eprintln!("{}", e);
    }
}