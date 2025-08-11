use reda_gds::GdsLibrary;

fn main_result() -> Result<(), Box<dyn std::error::Error>> {
    let lib = GdsLibrary::read_gds("./data/cells/dff.gds")?;
    lib.write_text("./temp/dff.txt")?;

    let lib = GdsLibrary::read_gds("./data/sram/sram_1rw0r0w_8_256_freepdk45.gds")?;
    lib.write_text("./temp/sram.txt")?;
    Ok(())
}

fn main() {
    if let Err(e) = main_result() {
        eprintln!("{}", e);
    }
}