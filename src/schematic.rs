// (c) 2016 Joost Yervante Damad <joost@productize.be>

use std::fmt;

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
    fn here(&self) -> &String {
        return &self.v[self.i]
    }

    fn next(&mut self) -> () {
        self.i += 1
    }

    fn eof(&self) -> bool {
        self.i >= self.v.len()
    }
    fn parse_description(&mut self) -> ERes<Description> {
        Ok(Description::new())
    }
    fn parse_component(&mut self) -> ERes<Component> {
        Ok(Component::new())
    }
}

#[derive(Debug,Clone)]
pub struct Description {
    size:String
}

impl Description {
    fn new() -> Description {
        Description { size:String::from("A4") }
    }
}

impl fmt::Display for Description {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(writeln!(f, "$Descr {}", self.size));
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


fn parse(s: &str) -> ERes<Schematic> {
    let mut sch = Schematic::new();
    let v:Vec<&str> = s.lines().collect();
    let v = v.iter().map(|x| String::from(*x)).collect();
    let mut p = ParseState {i:0, v: v };
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
        match &p.here()[..] {
            "$Descr" => {
                let d = try!(p.parse_description());
                sch.set_description(&d)
            },
            "$Comp" => {
                let c = try!(p.parse_component());
                sch.append_component(c)
            },
            _ => println!("TODO: parse other")
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

