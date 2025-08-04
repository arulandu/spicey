# Spicey
Spicey is a Rust DSL for circuits that supports interop with SPICE netlists and ships with a NgSpice REPL.

## Usage
MacOS support only for the moment. Clone the repository and use `cargo run` to start a wrapped NgSpice REPL.

## Roadmap
- [ ] MVP:
    - [x] Parsing and validation of basic circuit components
    - [x] Netlist generation from DSL
    - [x] Calling NgSpice
    - [x] Loading DSL circuits into NgSpice
    - [x] Data extraction
    - [ ] Quitting and controlled exists
    - [ ] Exporting data. Plotting (basic)
- [ ] AST Features:
    - [ ] Subcircuit support 
    - [ ] Transient and controlled sources
    - [ ] Models

## Related Work
- https://github.com/ftorres16/ftspice
- https://github.com/Harnesser/tiny-spice-rs
- https://github.com/ua-kxie/paprika
- https://ashwith.github.io/ngspicepy/_modules/ngspicepy/ngspicepy.html
