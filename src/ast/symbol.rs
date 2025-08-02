use std::convert::Into;
use nom::{IResult, Parser as NomParser};
use nom::bytes::complete::{take_while1};
use super::{Parse, Netlist, Validate};

#[derive(Debug, PartialEq)]
pub struct Symbol {
    name: String
}
impl Into<String> for Symbol {
    fn into(self) -> String {
        self.name
    }
}
impl Parse for Symbol {
    fn parse(s: &str) -> IResult<&str, Symbol> {
        let (r, s) = take_while1(move |c: char| c.is_alphanumeric() || c == '_' || c == '-').parse(s)?;
        Ok((r, Symbol {name: s.to_string()}))
    }
}
impl Netlist for Symbol {
    fn netlist(&self) -> String {
        self.name.clone()
    }
}

impl Validate for Symbol {}