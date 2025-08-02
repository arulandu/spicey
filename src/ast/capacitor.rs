use super::{Parse, Netlist, Validate};
use super::{symbol::Symbol, terminal::Terminal, value::{Value, Unit, Prefix}};
use nom::bytes::complete::{tag_no_case};
use nom::character::complete::{multispace1};
use nom::combinator::{opt, peek};
use nom::{IResult, Parser as NomParser};

#[derive(Debug, PartialEq)]
pub struct Capacitor {
    name: String,
    pos: Terminal,
    neg: Terminal,
    value: Value,
    initial: Value
}

impl Parse for Capacitor {
    fn parse(s: &str) -> IResult<&str, Capacitor> {
        peek(tag_no_case("C")).parse(s)?;
        let (r, (name, _, pos, _, neg, _, val, initial)) = (
            Symbol::parse,
            multispace1,
            Terminal::parse,
            multispace1,
            Terminal::parse,
            multispace1,
            Value::parse,
            opt((
                multispace1,
                tag_no_case("IC="),
                Value::parse
            ))
        ).parse(s)?;

        Ok((
            r,
            Capacitor {
                name: name.into(),
                pos,
                neg,
                value: val.into(),
                initial: initial.map_or(Value {
                    value: 0.,
                    unit: Unit::Unknown,
                    prefix: Prefix::Base
                } , move |(_, _, v)| v)
            },
        ))
    }
}
impl Netlist for Capacitor {
    fn netlist(&self) -> String {
        format!("{} {} {} {} IC={}", self.name, self.pos.netlist(), self.neg.netlist(), self.value.netlist(), self.initial.netlist())
    }
}

impl Validate for Capacitor {
    fn validate(&mut self) -> Result<(), String> {
        self.value.convert(Unit::Farad)?;
        Ok(())
    }
}