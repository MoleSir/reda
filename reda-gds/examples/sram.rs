use reda_gds::GdsLibrary;

fn main_result() -> Result<(), Box<dyn std::error::Error>> {
    let lib = GdsLibrary::read_gds("./data/sram/sram_1rw0r0w_8_256_freepdk45.gds")?;
    eprintln!("{}", lib.name);
    eprintln!("{}", lib.create_date);
    Ok(())
}

fn main() {
    if let Err(e) = main_result() {
        eprintln!("{}", e);
    }
}