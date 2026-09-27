use reda_gds::GdsLibrary;

fn main_result() -> Result<(), Box<dyn std::error::Error>> {
    let lib = GdsLibrary::load_file("./data/cells/dff.gds")?;
    lib.save_text_file("./temp/dff.txt")?;

    let lib = GdsLibrary::load_file("./data/sram/sram_1rw0r0w_8_256_freepdk45.gds")?;
    lib.save_text_file("./temp/sram.txt")?;
    Ok(())
}

fn main() {
    if let Err(e) = main_result() {
        eprintln!("{}", e);
    }
}