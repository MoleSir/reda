# RSpice

Simulate Electronic Circuit using Rust and the Ngspice Simulators. 

Aim to replace PySpice :)


## Features

Simulate

- [x] .sp file parse  
- [x] NgSpice dynamtic library simulator
- [x] NgSpice server simulator
- [x] Execute op, dc and tran analysis
- [x] Analysis draw
- [ ] Spice library
- [ ] .param and .option 


## Examples

```rust
let mut circuit = Circuit::new("Transient sin");

circuit.add_sine_voltage("input", "in", "0", SineVoltage::sin(u!(1 V), u!(100 Hz)));
circuit.add_resistor("1", "in", "1", u!(3 kΩ));
circuit.add_resistor("2", "1", "0", u!(1 kΩ));

let simulate = NgSpiceShared::default()?;
let mut simulator = circuit.simulator(simulate);
let command = TranCommandBuilder::default()
    .t_stop(u!(0.02 s))
    .t_step(u!(1 us))
    .build().unwrap();
let analysis = simulator.run_tran(&command)?;

analysis.draw_nodes(&Drawer::default(), &["in"], "./images/transient-sin-nodes.png")?;
analysis.draw_all_branchs(&Drawer::default(), "./images/transient-sin-branchs.png")?;
```


## References

- https://github.com/PySpice-org/PySpice
- https://web.stanford.edu/class/ee133/handouts/general/spice_ref.pdf