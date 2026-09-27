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
    let library = GdsLibrary::load_file(cli.input_path)?;
    library.save_text_file(cli.output_path)?;
    Ok(())
}

fn main() {
    if let Err(e) = main_result() {
        eprintln!("{}", e);
    }
}