use super::{Parse, Netlist, Validate};
use super::{symbol::Symbol, terminal::Terminal, value::{Value, Unit}};
use nom::bytes::complete::{tag_no_case, tag};
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::{opt, peek};
use nom::sequence::terminated;
use nom::{IResult, Parser as NomParser};

#[derive(Debug, PartialEq)]
pub struct VoltageSourceAC {
    name: String,
    pos: Terminal,
    neg: Terminal,
    value: Value,
    phase: Value
}
impl Parse for VoltageSourceAC {
    fn parse(s: &str) -> IResult<&str, VoltageSourceAC> {
        peek(tag_no_case("V")).parse(s)?;
        let (r, (name, _, pos, _, neg, _, _, _, val, _, phase)) = (
            Symbol::parse,
            multispace1,
            Terminal::parse,
            multispace1,
            Terminal::parse,
            multispace1,
            tag_no_case("AC"),
            multispace1,
            Value::parse,
            (tag(","), multispace1),
            Value::parse,
        ).parse(s)?;

        Ok((
            r,
            VoltageSourceAC {
                name: name.into(),
                pos,
                neg,
                value: val.into(),
                phase: phase.into(),
            },
        ))
    }
}

impl Netlist for VoltageSourceAC {
    fn netlist(&self) -> String {
        format!("{} {} {} AC {}, {}", self.name, self.pos.netlist(), self.neg.netlist(), self.value.netlist(), self.phase.netlist())
    }
}

impl Validate for VoltageSourceAC {
    fn validate(&mut self) -> Result<(), String> {
        self.value.convert(Unit::Volt)?;
        Ok(())
    }
}