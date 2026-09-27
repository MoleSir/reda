use reda_tf::Techfile;

fn main() {
    if let Err(e) = main_result() {
       println!("Error: {}", e);
    }
}

fn main_result() -> Result<(), Box<dyn std::error::Error>> {
    let techfile = Techfile::load_file("./bench/freepdk45.tf")?;
    println!("{}", techfile.technology.name);

    for layer in techfile.layers {
        println!("{} : {} : {}", layer.name(), layer.mask_name(), layer.layer_number());
    }

    // let techfile = Techfile::load_file("./bench/tsmcn28.tf")?;
    // println!("{}", techfile.technology.name);
    Ok(())
}