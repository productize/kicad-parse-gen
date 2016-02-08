// (c) 2016 Joost Yervante Damad <joost@productize.be>

use std::fmt;
use std::str::FromStr;
use std::path::PathBuf;
use std::collections::HashMap;

// get from parent
use ERes;
use err;
use read_file;
use parse_split_quote_aware;
use schematic;

#[derive(Debug)]
pub struct SymbolLib {
    pub symbols:Vec<Symbol>,
}

// DEF name reference unused text_offset draw_pinnumber draw_pinname unit_count units_locked option_flag
#[derive(Debug)]
pub struct Symbol {
    pub name:String,
    pub reference:String,
    pub text_offset:f64,
    pub draw_pinnumber: bool,
    pub draw_pinname: bool,
    pub unit_count: i64,
    pub unit_locked: bool,
    pub is_power: bool,
    pub fields:Vec<Field>,
    pub draw:Vec<String>, // TODO parse draw
}
// F n “text” posx posy dimension orientation visibility hjustify vjustify/italic/bold “name”
// F0 "#PWR" 0 0 30 H I C CNN

#[derive(Debug)]
pub struct Field {
    pub i:i64,
    pub value:String,
    pub x:f64,
    pub y:f64,
    pub dimention:i64,
    pub orientation:schematic::Orientation,
    pub visibility:bool,
    pub hjustify:schematic::Justify,
    pub vjustify:schematic::Justify,
    pub italic:bool,
    pub bold:bool,
    pub name:String,
}

impl SymbolLib {
    fn new() -> SymbolLib {
        SymbolLib {
            symbols:vec![]
        }
    }
}

impl Symbol {
    fn new(name:String, reference:String) -> Symbol {
        Symbol {
            name:name,
            reference:reference,
            text_offset:0.0,
            draw_pinnumber:false,
            draw_pinname:false,
            unit_count:1,
            unit_locked:false,
            is_power:false,
            fields:vec![],
            draw:vec![],
        }
    }
}

macro_rules! assume_line {
    ($s:expr, $exp:expr) => (
        if $s.eof() {
            return err("end of file reached")
        }
        if $s.here() != $exp {
            return Err(format!("expected '{}', got '{}'", $exp, $s.here()))
        }
        $s.i += 1;
    )
}

#[derive(Debug)]
struct ParseState {
    i:usize,
    v:Vec<String>,
}

impl ParseState {
    fn new(v2:Vec<&str>) -> ParseState {
        ParseState {
            i:0,
            v:v2.iter().map(|x| String::from(*x)).collect(),
        }
    }
    
    fn here(&self) -> String {
        let s =  (self.v[self.i]).clone();
        //println!("{}", s);
        s
    }

    fn next(&mut self) {
        self.i += 1;
    }

    fn eof(&self) -> bool {
        self.i >= self.v.len()
    }
}

fn char_at(s:&String, p:usize) -> char {
    let v:Vec<char> = s.chars().collect();
    v[..][p]
}


fn assume_string(e:&'static str, s:&String) -> ERes<()> {
    if String::from(e) != *s {
        return Err(format!("expecting: {}, actually: {}", e, s))
    }
    return Ok(())
}

fn i64_from_string(p:&ParseState, s:&String) -> ERes<i64> {
    match i64::from_str(&s[..]) {
        Ok(i) => Ok(i),
        _ => Err(format!("int parse error in {}; line: {}", s, p.here()))
    }
}

fn f64_from_string(p:&ParseState, s:&String) -> ERes<f64> {
    match f64::from_str(&s[..]) {
        Ok(i) => Ok(i),
        _ => Err(format!("float parse error in {}; line: {}", s, p.here()))
    }
}

fn bool_from_string(s:&String, t:&'static str, f:&'static str) -> ERes<bool> {
    if &s[..] == t {
        return Ok(true)
    }
    if &s[..] == f {
        return Ok(false)
    }
    Err(format!("unknown boolean {}, expected {} or {}", s, t, f))
}

fn bool_from<T: PartialEq + fmt::Display>(i:T, t:T, f:T) -> ERes<bool> {
    if i == t {
        return Ok(true)
    }
    if i == f {
        return Ok(false)
    }
    Err(format!("unknown boolean {}, expected {} or {}", i, t, f))
}

fn parse_symbol(p:&mut ParseState) -> ERes<Symbol> {
    assume_line!(p,"#");
    p.next(); // skip line like # name
    assume_line!(p,"#");
    let s = p.here();
    let v = &parse_split_quote_aware(&s);
    if v.len() != 10 {
        return Err(format!("unexpected elements in {}", s))
    }
    assume_string("DEF", &v[0]);
    let mut s = Symbol::new(v[1].clone(), v[2].clone());
    s.text_offset = try!(f64_from_string(p, &v[4]));
    s.draw_pinnumber = try!(bool_from_string(&v[5], "Y", "N"));
    s.draw_pinname = try!(bool_from_string(&v[6], "Y", "N"));
    s.unit_count = try!(i64_from_string(p, &v[7]));
    s.unit_locked = try!(bool_from_string(&v[8], "L", "F"));
    s.is_power = try!(bool_from_string(&v[9], "P", "N"));
    // TODO fields
    Ok(s)
}
    
fn parse(filename:Option<PathBuf>, s: &str) -> ERes<SymbolLib> {
    let mut lib = SymbolLib::new();
    let v:Vec<&str> = s.lines().collect();
    let p = &mut ParseState::new(v);
    assume_line!(p, "EESchema-LIBRARY Version 2.3");
    assume_line!(p, "#encoding utf-8");
    while !p.eof() {
        let s = try!(parse_symbol(p));
        lib.symbols.push(s)
    }
    Ok(lib)
}


pub fn parse_str(s:&str) -> ERes<SymbolLib> {
    parse(None, s)
}

pub fn parse_file(filename:&PathBuf) -> ERes<SymbolLib> {
    let name = filename.to_str().unwrap();
    let s = try!(read_file(name));
    parse(Some(filename.clone()), &s[..])
}
