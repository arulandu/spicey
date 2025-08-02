use super::{Parse, Netlist, Validate};
use super::{symbol::Symbol, terminal::Terminal, value::{Value, Unit}};
use nom::bytes::complete::{tag_no_case};
use nom::character::complete::{multispace1};
use nom::combinator::{peek};
use nom::{IResult, Parser as NomParser};

#[derive(Debug, PartialEq)]
pub struct Resistor {
    name: String,
    pos: Terminal,
    neg: Terminal,
    value: Value,
}
impl Parse for Resistor {
    fn parse(s: &str) -> IResult<&str, Resistor> {
        peek(tag_no_case("R")).parse(s)?;
        let (r, (name, _, pos, _, neg, _, mut val)) = (
            Symbol::parse,
            multispace1,
            Terminal::parse,
            multispace1,
            Terminal::parse,
            multispace1,
            Value::parse,
        ).parse(s)?;

        Ok((
            r,
            Resistor {
                name: name.into(),
                pos,
                neg,
                value: val.into(),
            },
        ))
    }
}
impl Netlist for Resistor {
    fn netlist(&self) -> String {
        format!("{} {} {} {}", self.name, self.pos.netlist(), self.neg.netlist(), self.value.netlist())
    }
}

impl Validate for Resistor {
    fn validate(&mut self) -> Result<(), String> {
        self.value.convert(Unit::Ohm)?;
        Ok(())
    }
}