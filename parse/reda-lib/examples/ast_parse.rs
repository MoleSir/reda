use reda_lib::model::LibLibrary;

fn main_result() -> Result<(), Box<dyn std::error::Error>> {
    let library = LibLibrary::load_file("./data/NangateOpenCellLibrary_typical.lib")?;
    println!("{}", library.nom_process.unwrap());
    println!("{}", library.name);
    for cell in library.cells.iter().filter(|cell| cell.ff.is_some()) {
        println!("{}", cell.name);
        println!("{:?}", cell.ff.as_ref().unwrap());
    }

    for oc in library.operating_conditions.iter() {
        println!("{:?}", oc);
    }

    for table in library.lu_table_templates.iter() {
        println!("{}", table.name);
    }

    println!("{}", library.time_unit);
    println!("{}", library.capacitive_load_unit.unwrap().value());
    println!("{}", library.default_input_pin_cap.unwrap());
    
    Ok(())
}

fn main() {
    if let Err(e) = main_result() {
        eprint!("{}", e);
    }
}