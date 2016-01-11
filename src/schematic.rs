// (c) 2016 Joost Yervante Damad <joost@productize.be>

use std::fmt;
use std::str::FromStr;

// get from parent
use ERes;
use err;
use read_file;

#[derive(Debug)]
pub struct Schematic {
    libraries:Vec<String>,
    description:Description,
    components:Vec<Component>,
}

impl Schematic {
    fn new() -> Schematic {
        Schematic {
            libraries:vec![],
            description:Description::new(),
            components:vec![],
        }
    }

    fn add_library(&mut self, s:&str) {
        self.libraries.push(String::from(s))
    }

    fn set_description(&mut self, d:&Description) {
        self.description.clone_from(d)
    }

    fn append_component(&mut self, c:Component) {
        self.components.push(c)
    }
}

impl fmt::Display for Schematic {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(writeln!(f, "EESchema Schematic File Version 2"));
        for v in &self.libraries[..] {
            try!(writeln!(f, "LIBS:{}", v))
        }
        try!(write!(f, "{}", self.description));
        for v in &self.components[..] {
            try!(write!(f, "{}", v))
        }
        Ok(())
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
        return (self.v[self.i]).clone()
    }

    fn next(&mut self) {
        self.i += 1;
    }

    fn eof(&self) -> bool {
        self.i >= self.v.len()
    }
}
/*
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
Title "Normal"
Date "Tue 19 May 2015"
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDesc
*/
#[derive(Debug,Clone)]
pub struct Description {
    size:String,
    dimx:i64,
    dimy:i64,
    title:String,
    date:String,
    rev:String,
    comp:String,
    comment1:String,
    comment2:String,
    comment3:String,
    comment4:String,
}

impl Description {
    fn new() -> Description {
        Description {
            size:String::from(""),
            dimx:0,
            dimy:0,
            title:String::from("Normal"),
            date:String::from("Tue 19 May 2015"),
            rev:String::from(""),
            comp:String::from(""),
            comment1:String::from(""),
            comment2:String::from(""),
            comment3:String::from(""),
            comment4:String::from(""),
        }
    }
    fn set_size(&mut self, s:&String) {
        self.size.clone_from(s)
    }
    fn set_dimx(&mut self, i:i64) {
        self.dimx = i;
    }
    fn set_dimy(&mut self, i:i64) {
        self.dimy = i;
    }
    fn set_title(&mut self, s:&String) {
        self.title.clone_from(s)
    }
    fn set_date(&mut self, s:&String) {
        self.date.clone_from(s)
    }
    fn set_rev(&mut self, s:&String) {
        self.rev.clone_from(s)
    }
    fn set_comp(&mut self, s:&String) {
        self.comp.clone_from(s)
    }
    fn set_comment1(&mut self, s:&String) {
        self.comment1.clone_from(s)
    }
    fn set_comment2(&mut self, s:&String) {
        self.comment2.clone_from(s)
    }
    fn set_comment3(&mut self, s:&String) {
        self.comment3.clone_from(s)
    }
    fn set_comment4(&mut self, s:&String) {
        self.comment4.clone_from(s)
    }
}

impl fmt::Display for Description {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(writeln!(f, "$Descr {} {} {}", self.size, self.dimx, self.dimy));
        try!(writeln!(f, "encoding utf-8"));
        try!(writeln!(f, "Sheet 1 1")); // TODO: handle sheet
        try!(writeln!(f, "Title \"{}\"", self.title));
        try!(writeln!(f, "Date \"{}\"", self.date));
        try!(writeln!(f, "Rev \"{}\"", self.rev));
        try!(writeln!(f, "Comp \"{}\"", self.comp));
        try!(writeln!(f, "Comment1 \"{}\"", self.comment1));
        try!(writeln!(f, "Comment2 \"{}\"", self.comment2));
        try!(writeln!(f, "Comment3 \"{}\"", self.comment3));
        try!(writeln!(f, "Comment4 \"{}\"", self.comment4));
        writeln!(f, "$EndDescr")
    }
}


#[derive(Debug)]
struct Component {
    name:String,
    reference:String,
}

impl Component {
    fn new() -> Component {
        Component {
            name:String::from("EFM32HG310"),
            reference:String::from("U1"),
        }
    }
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(writeln!(f, "$Comp"));
        try!(writeln!(f, "L {} {}", self.name, self.reference));
        writeln!(f, "$EndComp")
    }
}

fn assume_string(e:&'static str, s:&String) -> ERes<()> {
    if String::from(e) != *s {
        return Err(format!("expecting: {}, actually: {}", e, s))
    }
    return Ok(())
}

fn parse_split_n(n:usize, s:&String) -> ERes<Vec<String>> {
    let sp:Vec<&str> = s.splitn(n, ' ').collect();
    if sp.len() < n {
        return Err(format!("expecting {} elements in {}", n, s))
    }
    return Ok(sp.iter().map(|x| String::from(*x)).collect())
}

fn i64_from_string(s:&String) -> ERes<i64> {
    match i64::from_str(&s[..]) {
        Ok(i) => Ok(i),
        _ => Err(format!("int parse error in {}", s))
    }
}

// this is not perfect
fn unquote_string(s:&String) -> ERes<String> {
    let l = s.len();
    if s.starts_with("\"") && s.ends_with("\"") {
        let mut s = s.clone();
        s.remove(l-1);
        s.remove(0);
        return Ok(s)
    }
    Err(format!("expecting quoted string: {}", s))
}

fn word_and_qstring<F>(d:&mut Description, name:&'static str, s:&String, setter:F) -> ERes<()>
    where F:Fn(&mut Description, &String) -> ()
{
    let v = try!(parse_split_n(2, s));
    try!(assume_string(name, &v[0]));
    setter(d, &try!(unquote_string(&v[1])));
    Ok(())
}
    
    
fn parse_description(p:&mut ParseState) -> ERes<Description> {
    let mut d = Description::new();
    let v = try!(parse_split_n(4, &p.here()));
    d.set_size(&v[1]);
    d.set_dimx(try!(i64_from_string(&v[2])));
    d.set_dimy(try!(i64_from_string(&v[3])));
    p.next(); // $Descr
    p.next(); // encoding
    try!(assume_string("Sheet 1 1", &p.here()));
    p.next(); // Sheet
    try!(word_and_qstring(&mut d, "Title", &p.here(), |d, x| d.set_title(x)));
    p.next();
    try!(word_and_qstring(&mut d, "Date", &p.here(), |d, x| d.set_date(x)));
    p.next();
    try!(word_and_qstring(&mut d, "Rev", &p.here(), |d, x| d.set_rev(x)));
    p.next();
    try!(word_and_qstring(&mut d, "Comp", &p.here(), |d, x| d.set_comp(x)));
    p.next();
    try!(word_and_qstring(&mut d, "Comment1", &p.here(), |d, x| d.set_comment1(x)));
    p.next();
    try!(word_and_qstring(&mut d, "Comment2", &p.here(), |d, x| d.set_comment1(x)));
    p.next();
    try!(word_and_qstring(&mut d, "Comment3", &p.here(), |d, x| d.set_comment1(x)));
    p.next();
    try!(word_and_qstring(&mut d, "Comment4", &p.here(), |d, x| d.set_comment1(x)));
    p.next();
    try!(assume_string("$EndDescr", &p.here()));
    Ok(d)
}

fn parse_component(p:&mut ParseState) -> ERes<Component> {
    let mut d = Component::new();
    Ok(d)
}

fn parse(s: &str) -> ERes<Schematic> {
    let mut sch = Schematic::new();
    let v:Vec<&str> = s.lines().collect();
    let p = &mut ParseState::new(v);
    assume_line!(p, "EESchema Schematic File Version 2");
    while !p.eof() {
        {
          let s = p.here();
          if !s.starts_with("LIBS:") {
              break
          }
            sch.add_library(&s[5..]);
        }
        p.next();
    }
    assume_line!(p, "EELAYER 25 0");
    assume_line!(p, "EELAYER END");
    while !p.eof() {
        {
            match p.here().split_whitespace().next() {
                Some("$Descr") => {
                    let d = try!(parse_description(p));
                    sch.set_description(&d)
                },
                Some("$Comp") => {
                    let d = try!(parse_component(p));
                    sch.append_component(d)
                },
                Some(x) => {
                    //println!("TODO other parts: {}", x);
                },
                None => unreachable!()
            }
        }
        p.next()
    }
    Ok(sch)
}


pub fn parse_str(s: &str) -> Schematic {
    parse(s).unwrap()
}

pub fn parse_file(name: &str) -> Schematic {
    let s = read_file(name).unwrap();
    parse(&s[..]).unwrap()
}

