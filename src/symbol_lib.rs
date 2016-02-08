// (c) 2016 Joost Yervante Damad <joost@productize.be>

use std::fmt;
use std::str::FromStr;
use std::path::PathBuf;

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
#[derive(Debug,Clone)]
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

#[derive(Debug,Clone)]
pub struct Field {
    pub i:i64,
    pub value:String,
    pub x:f64,
    pub y:f64,
    pub dimension:i64,
    pub orientation:schematic::Orientation,
    pub visible:bool,
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

    pub fn find<F>(&self, filter:F) -> Option<&Symbol>
        where F:Fn(&Symbol) -> bool
    {
        for symbol in &self.symbols {
            if filter(symbol) {
                return Some(symbol)
            }
        }
        None
    }
}

impl fmt::Display for SymbolLib {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(writeln!(f, "EESchema-LIBRARY Version 2.3"));
        try!(writeln!(f, "#encoding utf-8"));
        try!(writeln!(f, "#"));
        for i in &self.symbols {
            try!(write!(f, "{}", i))
        };
        writeln!(f, "#End Library")
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
    pub fn set_name(&mut self, name:String) {
        if char_at(&self.name, 0) == '~' {
            self.name = format!("~{}", name)
        } else {
            self.name = name.clone()
        }
        let mut field = &mut self.fields[1];
        field.value = name
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(writeln!(f, "# {}", &&self.fields[1].value));
        try!(writeln!(f, "#"));
        try!(writeln!(f, "DEF {} {} 0 {} {} {} {} {} {}",
                     self.name, self.reference, self.text_offset,
                     if self.draw_pinnumber { "Y" } else { "N" },
                     if self.draw_pinname { "Y" } else { "N" },
                     self.unit_count,
                     if self.unit_locked { "L" } else { "F" },
                     if self.is_power { "P" } else { "N" },
                     ));
        for field in &self.fields {
            try!(writeln!(f, "{}", field))
        };
        try!(writeln!(f, "DRAW"));
        for draw in &self.draw {
            try!(writeln!(f, "{}", draw))
        };
        try!(writeln!(f, "ENDDRAW"));
        try!(writeln!(f, "ENDDEF"));
        writeln!(f, "#")
    }
}

impl Field {
    fn new() -> Field {
        Field {
            i:0,
            value:String::from(""),
            x:0.0,
            y:0.0,
            dimension:0,
            orientation:schematic::Orientation::Horizontal,
            visible:false,
            hjustify:schematic::Justify::Center,
            vjustify:schematic::Justify::Center,
            italic:false,
            bold:false,
            name:String::from(""),
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "F{} \"{}\" {} {} {} {} {} {} {}{}{}",
               self.i, self.value, self.x, self.y, self.dimension,
               self.orientation,
               if self.visible { "V" } else { "I" },
               self.hjustify, self.vjustify,
               if self.italic { "I" } else { "N" },
               if self.bold { "I" } else { "N" },
                    ));
        if self.i > 3 {
            try!(write!(f, " \"{}\"", self.name))
        };
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

fn char_at(s:&String, p:usize) -> char {
    let v:Vec<char> = s.chars().collect();
    v[..][p]
}

fn parse_symbol(p:&mut ParseState) -> ERes<Symbol> {
    p.next(); // skip line like # name
    assume_line!(p,"#");
    let s = p.here();
    let v = &parse_split_quote_aware(&s);
    if v.len() != 10 {
        return Err(format!("unexpected elements in {}", s))
    }
    try!(assume_string("DEF", &v[0]));
    let mut s = Symbol::new(v[1].clone(), v[2].clone());
    s.text_offset = try!(f64_from_string(p, &v[4]));
    s.draw_pinnumber = try!(bool_from_string(&v[5], "Y", "N"));
    s.draw_pinname = try!(bool_from_string(&v[6], "Y", "N"));
    s.unit_count = try!(i64_from_string(p, &v[7]));
    s.unit_locked = try!(bool_from_string(&v[8], "L", "F"));
    s.is_power = try!(bool_from_string(&v[9], "P", "N"));
    // TODO fields
    // TODO draw
    p.next();
    loop {
        let s2 = p.here();
        if char_at(&s2, 0) == 'F' {
            let f = try!(parse_field(p, &s2));
            s.fields.push(f);
            p.next();
        } else {
            break;
        }
    }
    if &p.here() == "$FPLIST" {
        p.next();
        // skip FPLIST for now
        while !p.eof() {
            if &p.here() == "$ENDFPLIST" {
                p.next();
                break;
            }
            p.next()
        }
    }
    assume_line!(p, "DRAW");
    while !p.eof() {
        let s2 = p.here();
        if &s2 == "ENDDRAW" {
            p.next();
            break;
        }
        s.draw.push(s2.clone());
        p.next()
    }
    assume_line!(p,"ENDDEF");
    assume_line!(p,"#");
    Ok(s)
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

// F0 "L" 0 50 40 H V C CNN
fn parse_field(p:&mut ParseState, line:&String) -> ERes<Field> {
    let mut f = Field::new();
    let v = &parse_split_quote_aware(&line);
    if v.len() != 9 && v.len() != 10 {
        return Err(format!("unexpected elements in {}", line))
    }
    f.i = try!(i64_from_string(p, &String::from(&v[0][1..])));
    let name = if v.len() == 10 {
        v[9].clone()
    } else {
        match f.i {
            0 => String::from("Reference"),
            1 => String::from("Value"),
            2 => String::from("Footprint"),
            3 => String::from("UserDocLink"),
            _ => return Err(format!("expecting name for componentfield > 3")),
        }
    };
    f.value = v[1].clone();
    f.x = try!(f64_from_string(p, &v[2]));
    f.y = try!(f64_from_string(p, &v[3]));
    f.dimension = try!(i64_from_string(p, &v[4]));
    f.orientation = try!(schematic::Orientation::new(char_at(&v[5], 0)));
    f.visible = try!(bool_from_string(&v[6], "V", "I"));
    f.hjustify = try!(schematic::Justify::new(char_at(&v[7], 0)));
    f.vjustify = try!(schematic::Justify::new(char_at(&v[8], 0)));
    f.italic = try!(bool_from(char_at(&v[8], 1), 'I', 'N'));
    f.bold = try!(bool_from(char_at(&v[8], 2), 'B', 'N'));
    f.name = name;
    Ok(f)
}
    
fn parse(s: &str) -> ERes<SymbolLib> {
    let mut lib = SymbolLib::new();
    let v:Vec<&str> = s.lines().collect();
    let p = &mut ParseState::new(v);
    assume_line!(p, "EESchema-LIBRARY Version 2.3");
    assume_line!(p, "#encoding utf-8");
    assume_line!(p,"#");
    while !p.eof() {
        //println!("here: {}", &p.here());
        if &p.here() == "#End Library" {
            break;
        }
        let s = try!(parse_symbol(p));
        //println!("new symbol: {}", &s);
        lib.symbols.push(s)
    }
    Ok(lib)
}


pub fn parse_str(s:&str) -> ERes<SymbolLib> {
    parse(s)
}

pub fn parse_file(filename:&PathBuf) -> ERes<SymbolLib> {
    let name = filename.to_str().unwrap();
    let s = try!(read_file(name));
    parse(&s[..])
}
