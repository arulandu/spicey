use std::convert::Into;
use nom::branch::alt;
use nom::{IResult, Parser as NomParser};
use nom::number::float;
use nom::bytes::complete::{tag, tag_no_case, take_while1};
use nom::character::complete::{space1};
use nom::combinator::{map, opt, value};
use super::{Parse, Netlist};

#[derive(Debug, Clone, PartialEq)]
pub enum Unit {
    Unknown,
    Ohm,
    Farad,
    Ampere,
    Volt,
    Henry
}

#[derive(Debug, Clone, PartialEq)]
pub enum Prefix {
    Base,
    Tera,
    Giga,
    Mega,
    Kilo,
    Milli,
    Micro,
    Nano,
    Pico,
    Femto
}

impl Prefix {
    const fn symbol(&self) -> &'static str {
        match self {
            Prefix::Base => "",
            Prefix::Tera => "T",
            Prefix::Giga => "G",
            Prefix::Mega => "X",
            Prefix::Kilo => "K",
            Prefix::Milli => "m",
            Prefix::Micro => "u",
            Prefix::Nano => "n",
            Prefix::Pico => "p",
            Prefix::Femto => "f"
        }
    }

    const fn exponent(&self) -> isize {
        match self {
            Prefix::Base => 0,
            Prefix::Tera => 12,
            Prefix::Giga => 9,
            Prefix::Mega => 6,
            Prefix::Kilo => 3,
            Prefix::Milli => -3,
            Prefix::Micro => -6,
            Prefix::Nano => -9,
            Prefix::Pico => -12,
            Prefix::Femto => -15
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct Value {
    pub value: f32,
    pub unit: Unit,
    pub prefix: Prefix
}

impl Value {
    pub fn convert(&mut self, unit: Unit) -> Result<(), String> {
        if unit == self.unit || self.unit == Unit::Unknown {
            self.unit = unit;
            Ok(())
        } else {
            Err(format!("Expected {:?} but got {:?}", unit, self.unit))
        }
    }
}

impl Netlist for Value {
    fn netlist(&self) -> String {
        self.value.to_string() + self.prefix.symbol()
    }
}

impl Parse for Value {
    fn parse(s: &str) -> IResult<&str, Value> {
        let (r, (v, p, u)) = (float(), parse_prefix, parse_unit).parse(s)?;
        Ok((r, Value {value: v, unit: u, prefix: p}))
    }
}

fn parse_prefix(s: &str) -> IResult<&str, Prefix> {
    alt((
        value(Prefix::Tera, alt((
            tag_no_case("T"),
            tag_no_case("TERA")
        ))),
        value(Prefix::Giga, alt((
            tag_no_case("G"),
            tag_no_case("GIGA")
        ))),
        value(Prefix::Mega, alt((
            tag_no_case("MEG"),
            tag_no_case("MEGA"),
            tag_no_case("X")
        ))),
        value(Prefix::Kilo, alt((
            tag_no_case("K"),
            tag_no_case("KILO")
        ))),
        value(Prefix::Milli, alt((
            tag_no_case("m"),
            tag_no_case("MILLI")
        ))),
        value(Prefix::Micro, alt((
            tag_no_case("u"),
            tag_no_case("MICRO")
        ))),
        value(Prefix::Nano, alt((
            tag_no_case("n"),
            tag_no_case("NANO")
        ))),
        value(Prefix::Pico, alt((
            tag_no_case("p"),
            tag_no_case("PICO")
        ))),
        value(Prefix::Femto, alt((
            tag_no_case("f"),
            tag_no_case("FEMTO")
        ))),
        value(Prefix::Base, tag(""))
    )).parse(s)
}

fn parse_unit(s: &str) -> IResult<&str, Unit> {
    alt((
        value(Unit::Ohm, alt((
            tag_no_case("OHM"),
            tag_no_case("Ω")
        ))),
        value(Unit::Farad, alt((
            tag_no_case("FARAD"),
            tag_no_case("F")
        ))),
        value(Unit::Ampere, alt((
            tag_no_case("AMPERE"),
            tag_no_case("A")
        ))),
        value(Unit::Volt, alt((
            tag_no_case("VOLT"),
            tag_no_case("V")
        ))),
        value(Unit::Henry, alt((
            tag_no_case("HENRY"),
            tag_no_case("H")
        ))),
        value(Unit::Unknown, tag(""))
    )).parse(s)
}
