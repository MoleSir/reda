use reda_lef::{LefCellLibrary, LefGeometry, LefTechnology};

fn main() {
    if let Err(e) = main_result() {
        eprint!("Error: >>{}<<", e);
    }
}

fn main_result() -> Result<(), Box<dyn std::error::Error>> {
    let _ = LefTechnology::load_file("./bench/NangateOpenCellLibrary.tech.lef")?;
    let _ = LefCellLibrary::load_file("./bench/NangateOpenCellLibrary.macro.rect.lef")?;
    let _ = LefCellLibrary::load_file("./bench/NangateOpenCellLibrary.macro.mod.lef")?;
    let cells = LefCellLibrary::load_file("./bench/NangateOpenCellLibrary.macro.lef")?;
    for (name, makcro) in cells.macros.iter() {
        print!("{}", name);
        if let Some(size) = makcro.size {
            println!("{:?}", size);
        }
        for pin in makcro.pins.iter() {
            println!("\t{}", pin.name);
            for gs in pin.port.iter() {
                println!("\t\t{}", gs.layer_name);
                for g in gs.geometries.iter() {
                    match g {
                        LefGeometry::Rect(rect) => { println!("\t\t\tRECT: {} {}", rect.lower_left, rect.upper_right) }
                        LefGeometry::Polygon(ps) => {
                            print!("\t\t\tPOLYGON: ");
                            for p in ps.iter() {
                                print!("{:?} ", p);
                            }
                            println!("");
                        }
                        _ => {},
                    }
                }
            }
        }
    }
    Ok(())
}