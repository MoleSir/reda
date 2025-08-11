use reda_spice::Spice;

fn main_result() -> Result<(), Box<dyn std::error::Error>> {
    let spice = Spice::load_from("./data/addr_mos.sp")?;
    println!("{:?}", spice);
    let spice = Spice::load_from("./data/filter.sp")?;
    println!("{:?}", spice);
    Ok(())
}

fn main() {
    if let Err(e) =  main_result() {
        eprintln!("{}", e);
    }
}