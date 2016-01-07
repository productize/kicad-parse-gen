// (c) 2016 Joost Yervante Damad <joost@productize.be>

use std::fmt;

// get from parent
use ERes;
use err;
use read_file;

pub struct Schematic {
    libraries:Vec<String>
}

impl Schematic {
    fn new() -> Schematic {
        Schematic { libraries:vec![] }
    }

    fn add_library(&mut self, s:&str) {
        self.libraries.push(String::from(s))
    }
}

impl fmt::Display for Schematic {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(writeln!(f, "EESchema Schematic File Version 2"));
        for v in &self.libraries[..] {
            try!(writeln!(f, "LIBS:{}", v))
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

struct ParseState {
    i:usize,
    v:Vec<String>,
}

impl ParseState {
    fn here(&self) -> &String {
        return &self.v[self.i]
    }

    fn next(&mut self) {
        self.i += 1
    }

    fn eof(&self) -> bool {
        self.i >= self.v.len()
    }
}


fn parse(s: &str) -> ERes<Schematic> {
    let mut sch = Schematic::new();
    let v:Vec<&str> = s.lines().collect();
    fn from (x:&&str) -> String {
        String::from(*x)
    }
    let v = v.iter().map(from).collect();
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

