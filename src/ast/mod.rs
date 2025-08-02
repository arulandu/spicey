
use nom::bytes::complete::{tag, tag_no_case, take_while1};
use nom::character::complete::{multispace0, space1};
use nom::multi::many0;
use nom::sequence::delimited;
use nom::{IResult, Parser as NomParser};
use nom::combinator::{map, opt};
use nom::branch::alt;

mod resistor;
pub use resistor::Resistor;

mod inductor;
pub use inductor::Inductor;

mod capacitor;
pub use capacitor::Capacitor;

mod vsource_dc;
pub use vsource_dc::VoltageSourceDC;

mod isource_dc;
pub use isource_dc::CurrentSourceDC;

mod vsource_ac;
pub use vsource_ac::VoltageSourceAC;

mod isource_ac;
pub use isource_ac::CurrentSourceAC;

mod value;
pub use value::Value;

mod symbol;
pub use symbol::Symbol;

mod terminal;
pub use terminal::Terminal;

pub trait Parse where Self : Sized {
    fn parse(s: &str) -> IResult<&str, Self>;
}

pub trait Validate {
    fn validate(&mut self) -> Result<(), String> {
        Ok(())
    }
}

pub trait Netlist {
    fn netlist(&self) -> String;
}

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Resistor(Resistor),
    Inductor(Inductor),
    Capacitor(Capacitor),
    VoltageSourceDC(VoltageSourceDC),
    CurrentSourceDC(CurrentSourceDC),
    VoltageSourceAC(VoltageSourceAC),
    CurrentSourceAC(CurrentSourceAC),
}

impl Parse for AstNode {
    fn parse(s: &str) -> IResult<&str, AstNode> {
        alt((
            map(Resistor::parse, AstNode::Resistor),
            map(Inductor::parse, AstNode::Inductor),
            map(Capacitor::parse, AstNode::Capacitor),
            map(VoltageSourceDC::parse, AstNode::VoltageSourceDC),
            map(CurrentSourceDC::parse, AstNode::CurrentSourceDC),
            map(VoltageSourceAC::parse, AstNode::VoltageSourceAC),
            map(CurrentSourceAC::parse, AstNode::CurrentSourceAC),
        )).parse(s)
    }
}   

impl Validate for AstNode {
    fn validate(&mut self) -> Result<(), String> {
        match self {
            AstNode::Resistor(x) => x.validate(),
            AstNode::Inductor(x) => x.validate(),
            AstNode::Capacitor(x) => x.validate(),
            AstNode::VoltageSourceDC(x) => x.validate(),
            AstNode::CurrentSourceDC(x) => x.validate(),
            AstNode::VoltageSourceAC(x) => x.validate(),
            AstNode::CurrentSourceAC(x) => x.validate(),
        }
    }
}

impl Netlist for AstNode {
    fn netlist(&self) -> String {
        match self {
            AstNode::Resistor(x) => x.netlist(),
            AstNode::Inductor(x) => x.netlist(),
            AstNode::Capacitor(x) => x.netlist(),
            AstNode::VoltageSourceDC(x) => x.netlist(),
            AstNode::CurrentSourceDC(x) => x.netlist(),
            AstNode::VoltageSourceAC(x) => x.netlist(),
            AstNode::CurrentSourceAC(x) => x.netlist(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Ast {
    title: String,
    nodes: Vec<AstNode>
}
impl Parse for Ast {
    fn parse(s: &str) -> IResult<&str, Ast> {
        let (r, (title, nodes, _)) = (
            parse_title,
            delimited(multispace0, 
                many0(delimited(multispace0, AstNode::parse, multispace0)), 
            multispace0),
            parse_end
        ).parse(s)?;
    
        Ok((r, Ast {title: title.to_string(), nodes}))
    }
}

pub fn parse_title(s: &str) -> IResult<&str, &str> {
    let d = "_- ";
    let tp = take_while1(move |c: char| c.is_alphanumeric() || d.contains(c));
    let (r, (_, _, _, name)) = (
        opt(tag(".")),
        tag_no_case("title"),
        space1,
        tp
    ).parse(s)?;
    Ok((r, name))
}

pub fn parse_end(s: &str) -> IResult<&str, ()> {
    let (r, _) = (opt(tag(".")), tag_no_case("end")).parse(s)?;
    Ok((r, ()))
}

impl Netlist for Ast {
    fn netlist(&self) -> String {
        let mut s = format!(".TITLE {}\n", self.title);
        for node in &self.nodes {
            s += &node.netlist();
            s += "\n";
        }
        s += ".END\n";
        s
    }
}

impl Validate for Ast {
    fn validate(&mut self) -> Result<(), String> {
        for node in &mut self.nodes {
            node.validate()?;
        }
        Ok(())
    }
}