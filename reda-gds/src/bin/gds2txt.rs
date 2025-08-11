use std::path::PathBuf;
use reda_gds::GdsLibrary;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input GDS file path
    input_path: PathBuf,

    /// Output text file path
    output_path: PathBuf,
}

fn main_result() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let library = GdsLibrary::read_gds(cli.input_path)?;
    library.write_text(cli.output_path)?;
    Ok(())
}

fn main() {
    if let Err(e) = main_result() {
        eprintln!("{}", e);
    }
}