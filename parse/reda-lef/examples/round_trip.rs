use reda_lef::{LefCellLibrary, LefTechnology};

fn main() {
    // Round-trip test for tech LEF
    let tech = LefTechnology::load_file("./bench/NangateOpenCellLibrary.tech.lef")
        .expect("Failed to parse tech LEF");
    println!("Parsed tech LEF: {} layers, {} vias, {} sites, {} viarules",
        tech.layers.len(), tech.vias.len(), tech.sites.len(), tech.via_rules.len());

    tech.save_file("./temp/test_tech.lef").expect("Failed to write tech LEF");

    let tech2 = LefTechnology::load_file("./temp/test_tech.lef")
        .expect("Failed to re-parse tech LEF");
    println!("Re-parsed tech LEF: {} layers, {} vias, {} sites, {} viarules",
        tech2.layers.len(), tech2.vias.len(), tech2.sites.len(), tech2.via_rules.len());

    assert_eq!(tech.layers.len(), tech2.layers.len(), "Layer count mismatch");
    println!("Tech LEF round-trip OK!");

    // Round-trip test for cell LEF
    let cells = LefCellLibrary::load_file("./bench/NangateOpenCellLibrary.macro.lef")
        .expect("Failed to parse cell LEF");
    println!("Parsed cell LEF: {} macros, {} vias, {} sites",
        cells.macros.len(), cells.vias.len(), cells.sites.len());

    cells.save_file("./temp/test_cells.lef").expect("Failed to write cell LEF");

    let cells2 = LefCellLibrary::load_file("./temp/test_cells.lef")
        .expect("Failed to re-parse cell LEF");
    println!("Re-parsed cell LEF: {} macros, {} vias, {} sites",
        cells2.macros.len(), cells2.vias.len(), cells2.sites.len());

    assert_eq!(cells.macros.len(), cells2.macros.len(), "Macro count mismatch");
    println!("Cell LEF round-trip OK!");
}
