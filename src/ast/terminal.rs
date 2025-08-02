use std::convert::Into;
use nom::character::complete::alpha1;
use nom::combinator::peek;
use nom::{IResult, Parser as NomParser};
use nom::bytes::complete::{take_while1};
use super::{Parse, Netlist, Validate};

#[derive(Debug, PartialEq)]
pub struct Terminal {
    name: String
}
impl Into<String> for Terminal {
    fn into(self) -> String {
        self.name
    }
}
impl Parse for Terminal {
    fn parse(s: &str) -> IResult<&str, Terminal> {
        let (r, s) = take_while1(move |c: char| c.is_alphanumeric() || c == '_' || c == '-').parse(s)?;
        Ok((r, Terminal {name: s.to_string()}))
    }
}
impl Netlist for Terminal {
    fn netlist(&self) -> String {
        self.name.clone()
    }
}
impl Validate for Terminal {}