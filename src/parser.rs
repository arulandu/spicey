use nom::Finish;
use std::fs;

use super::ast::{Ast, Parse, Netlist, Validate};

pub fn parse_file(file_path: &str) -> Result<Ast, String> {
    let data = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
    parse_str(&data)
}

pub fn parse_str(data: &str) -> Result<Ast, String> {
    let (_, mut ast) = Finish::finish(Ast::parse(data)).map_err(|e| e.to_string())?;
    ast.validate()?;
    Ok(ast)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! generate_netlist_test {
        ($name:ident, $path:expr) => {
            #[test]
            fn $name() {
                let circuit = parse_file($path).unwrap();
                let netlist = circuit.netlist();
                let circuit2 = parse_str(&netlist).unwrap();
                assert_eq!(circuit, circuit2, "Netlist roundtrip failed for {}", $path);
            }
        };
    }

    generate_netlist_test!(empty, "./netlists/empty.sp");
    generate_netlist_test!(resistor, "./netlists/resistor.sp");
    generate_netlist_test!(vdiv, "./netlists/vdiv.sp");
}
