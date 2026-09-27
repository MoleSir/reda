# rEDA

An EDA development kit in Rust, organized as a single Cargo workspace.

## Crates

### Base

- **[reda-unit](base/reda-unit)** — physical units with compile-time safety and automatic
  conversions; SI units, complex numbers and built-in formulas. Proc-macro companion:
  `reda-unit/codegen` (`reda-unit-macros`).
- **[reda-geometry](base/reda-geometry)** — geometry library: `vec2`/`mat2`/`mat3` math, plus
  point, edge, rect, polygon, polyline, path and text shapes with transforms.

### Parse

- **[reda-gds](parse/reda-gds)** — GDSII library: read and write `.gds`, write text format,
  and a simple gds → txt tool.
- **[reda-lef](parse/reda-lef)** — LEF (Library Exchange Format) parser: technology, site,
  macro, via and layer.
- **[reda-lefdef](parse/reda-lefdef)** — LEF/DEF library wrapping the Si2 LEF/DEF reference
  implementation (early stage).
- **[reda-def](parse/reda-def)** — DEF file library. Model types only so far; the reader is
  not implemented yet.
- **[reda-lib](parse/reda-lib)** — Liberty timing library: source → AST → semantic model, and
  writing back to `.lib`. Proc-macro companion: `parse/reda-lib/codegen` (`reda-lib-macro`).
- **[reda-sp](parse/reda-sp)** — SPICE netlist library: devices (MOS/BJT/diode), subcircuits,
  sources, simulation control and measurements.
- **[reda-v](parse/reda-v)** — Verilog parser producing a structured AST (modules, ports,
  expressions, ...).
- **[reda-tf](parse/reda-tf)** — technology `.tf` file parser: technology, layer, contact,
  design rule, tile and color.
- **[reda-oasis](parse/reda-oasis)** — OASIS layout format library (records and data).

### Simulate

- **[reda-spsim](simulate/reda-spsim)** — circuit simulation on top of ngspice: netlist
  building, AC/DC/transient analysis.

### Algorithm

- **[reda-partition](algorithm/reda-partition)** — graph partitioning: Kernighan-Lin is done,
  Fiduccia-Mattheyses is in progress.
- **[reda-placement](algorithm/reda-placement)** — placement. Not implemented yet.
- **[reda-route](algorithm/reda-route)** — routing. Not implemented yet.

### Tool

- **[reda-tokenize](tool/reda-tokenize)** — general tokenizer.

## License

MIT, see [LICENSE](LICENSE).
