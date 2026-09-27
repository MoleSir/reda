# reda-v

A verilog file library in Rust.



## Features

- Parse complete Verilog source files into a structured `Verilog` AST
- Support for multiple modules within one file
- Module parsing, including:
    - Ports (`input`, `output`, `inout`) with optional ranges
    - Parameters with constant expressions
    - Nets (`wire`, `reg`) with optional ranges
    - Continuous assignments (`assign`)
    - Always blocks with sensitivity lists (`posedge`, `negedge`, or `*`)
    - Instantiations of other modules
- Statement parsing inside procedural blocks:
    - Blocking assignments (`=`)
    - Non-blocking assignments (`<=`)
    - `if/else` conditionals
    - Sequential `begin ... end` blocks
- Hierarchical AST representation with structs like:
    - `Verilog`, `Module`
    - `Port`, `Parameter`, `Net`, `Assign`, `Always`, `Instance`
    - `Stmt`, `Expr`, `Range`, etc.




## Examples

````rust
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
````



## LICENSE

MIT



## References

- https://en.wikipedia.org/wiki/Verilog