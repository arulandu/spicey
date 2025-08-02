use super::{Parse, Netlist, Validate};
use super::{symbol::Symbol, terminal::Terminal, value::{Value, Unit}};
use nom::bytes::complete::{tag_no_case};
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::{opt, peek};
use nom::sequence::terminated;
use nom::{IResult, Parser as NomParser};

#[derive(Debug, PartialEq)]
pub struct CurrentSourceDC {
    name: String,
    pos: Terminal,
    neg: Terminal,
    value: Value,
}
impl Parse for CurrentSourceDC {
    fn parse(s: &str) -> IResult<&str, CurrentSourceDC> {
        peek(tag_no_case("I")).parse(s)?;
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
            CurrentSourceDC {
                name: name.into(),
                pos,
                neg,
                value: val.into(),
            },
        ))
    }
}
impl Netlist for CurrentSourceDC {
    fn netlist(&self) -> String {
        format!("{} {} {} DC={}", self.name, self.pos.netlist(), self.neg.netlist(), self.value.netlist())
    }
}

impl Validate for CurrentSourceDC {
    fn validate(&mut self) -> Result<(), String> {
        self.value.convert(Unit::Ampere)?;
        Ok(())
    }
}