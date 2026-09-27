use std::str::FromStr;

use reda_v::{model::Verilog, parse::load_file};

fn main() {
    let verilog = load_file("./data/adder.v").unwrap();
    for module in verilog.modules {
        println!("{:#?}", module);
    }

    let source = r#"
module adder (a, b, sum);
    input [3:0] a;
    input [3:0] b;
    output [4:0] sum;
    assign sum = a + b;
endmodule
    "#;

    let verilog = Verilog::from_str(source).unwrap();
    let adder = verilog.modules.get(0).unwrap();
    for p in adder.ports.iter() {
        println!("{}", p.name);
    }

}