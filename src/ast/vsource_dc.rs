use super::{Parse, Netlist, Validate};
use super::{symbol::Symbol, terminal::Terminal, value::{Value, Unit}};
use nom::bytes::complete::{tag_no_case};
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::{opt, peek};
use nom::sequence::terminated;
use nom::{IResult, Parser as NomParser};

#[derive(Debug, PartialEq)]
pub struct VoltageSourceDC {
    name: String,
    pos: Terminal,
    neg: Terminal,
    value: Value,
}
impl Parse for VoltageSourceDC {
    fn parse(s: &str) -> IResult<&str, VoltageSourceDC> {
        peek(tag_no_case("V")).parse(s)?;
        let (r, (name, _, pos, _, neg, _, _, val)) = (
            Symbol::parse,
            multispace1,
            Terminal::parse,
            multispace1,
            Terminal::parse,
            multispace1,
            opt(terminated(tag_no_case("DC="), multispace0)),
            Value::parse,
        ).parse(s)?;

        Ok((
            r,
            VoltageSourceDC {
                name: name.into(),
                pos,
                neg,
                value: val.into(),
            },
        ))
    }
}
impl Netlist for VoltageSourceDC {
    fn netlist(&self) -> String {
        format!("{} {} {} DC={}", self.name, self.pos.netlist(), self.neg.netlist(), self.value.netlist())
    }
}

impl Validate for VoltageSourceDC {
    fn validate(&mut self) -> Result<(), String> {
        self.value.convert(Unit::Volt)?;
        Ok(())
    }
}