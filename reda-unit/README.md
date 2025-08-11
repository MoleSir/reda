# reda-unit

A Rust library for physical units with compile-time safety and automatic unit conversions.

- [x] Supports SI units and common derived units  
- [x] Strongly typed unit-safe arithmetic  
- [x] Convenient macros for constructing quantities, e.g. `u!(10. mV)`  
- [x] Built-in physical formulas and unit relations, e.g. Ohm's law, power, energy  



## Example

```rust
use reda_unit::u;

fn main() {
    let voltage = u!(5.0 V);
    let resistance = u!(10. Ω);
    let current = voltage / resistance; // Automatically inferred as Current
    println!("Current = {}", current); // prints: Current = 0.5A
}
```



## Supported Units

You can create quantities with the `u!` macro using units below (including common prefixes):

| Unit          | Symbol | Description            |
| ------------- | ------ | ---------------------- |
| Voltage       | V      | Electric potential     |
| Current       | A      | Electric current       |
| Resistance    | Ω      | Electrical resistance  |
| Capacitance   | F      | Capacitance            |
| Inductance    | H      | Inductance             |
| Charge        | Q      | Electric charge        |
| Power         | W      | Power                  |
| Energy        | J      | Energy                 |
| Time          | s      | Time                   |
| Frequency     | Hz     | Frequency              |
| Length        | m      | Length                 |
| Area          | m²     | Area                   |
| Force         | N      | Force                  |
| Pressure      | Pa     | Pressure               |
| Magnetic Flux | Wb     | Magnetic flux          |
| Flux Density  | T      | Magnetic flux density  |
| Conductance   | S      | Electrical conductance |
| Velocity      | m/s    | Velocity               |
| Acceleration  | m/s²   | Acceleration           |
| Temperature   | K      | Temperature            |
| Angle         | rad    | Angle (radians)        |



## Physical Formulas Implemented

The library supports unit-safe versions of common physical relations, for example:

- Ohm's Law:
     $$
     V = I \times R
     $$
     
- Power:
     $$
     P = V \times I
     $$
     
- Energy from power and time:
     $$
     E = P \times t
     $$
     
- Charge from capacitance and voltage:
     $$
     Q = C \times V
     $$
     
- Current from charge and time:
     $$
     I = \frac{Q}{t}
     $$
     
- Power from force and velocity:
     $$
     P = F \times v
     $$
     
- Energy from force and distance:
     $$
     E = F \times d
     $$
     
- Pressure from force and area:
     $$
     P = \frac{F}{A}
     $$
     
- Magnetic flux from flux density and area:
     $$
     \Phi = B \times A
     $$
     
- Voltage from magnetic flux and time:
     $$
     V = \frac{\Phi}{t}
     $$

