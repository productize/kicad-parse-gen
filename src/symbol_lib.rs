// (c) 2016 Productize SPRL <joost@productize.be>

// extension: .lib
// format: old-style

use std::fmt;
use std::result;
use std::str::FromStr;
use std::path::PathBuf;

// get from parent
use Result;
use util::read_file;
use parse_split_quote_aware;
use schematic;
use str_error;

/// a Kicad symbolic file
#[derive(Debug,Default)]
pub struct SymbolLib {
    /// the symbols
    pub symbols: Vec<Symbol>,
}

// DEF name reference unused text_offset draw_pinnumber draw_pinname unit_count units_locked option_flag
/// a symbol
#[derive(Debug,Clone)]
pub struct Symbol {
    /// name
    pub name: String,
    /// reference
    pub reference: String,
    /// text offset
    pub text_offset: f64,
    /// draw pinnumber
    pub draw_pinnumber: bool,
    /// draw pinname
    pub draw_pinname: bool,
    /// unit count
    pub unit_count: i64,
    /// is the unit locked
    pub unit_locked: bool,
    /// is it a power symbol
    pub is_power: bool,
    /// fields
    pub fields: Vec<Field>,
    /// draw
    pub draw: Vec<String>, // TODO parse draw
}
// F n “text” posx posy dimension orientation visibility hjustify vjustify/italic/bold “name”
// F0 "#PWR" 0 0 30 H I C CNN

/// a field
#[derive(Debug,Clone)]
pub struct Field {
    /// field number
    pub i: i64,
    /// value
    pub value: String,
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// dimension
    pub dimension: i64,
    /// orientation
    pub orientation: schematic::Orientation,
    /// if the field is visible
    pub visible: bool,
    /// horizontal justification
    pub hjustify: schematic::Justify,
    /// vertical justification
    pub vjustify: schematic::Justify,
    /// italic
    pub italic: bool,
    /// bold
    pub bold: bool,
    /// name of the field
    pub name: String,
}

impl SymbolLib {
    /// find a symbol in a symbol lib
    pub fn find<F>(&self, filter: F) -> Option<&Symbol>
        where F: Fn(&Symbol) -> bool
    {
        for symbol in &self.symbols {
            if filter(symbol) {
                return Some(symbol);
            }
        }
        None
    }
}

impl fmt::Display for SymbolLib {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        writeln!(f, "EESchema-LIBRARY Version 2.3")?;
        writeln!(f, "#encoding utf-8")?;
        writeln!(f, "#")?;
        for i in &self.symbols {
            write!(f, "{}", i)?
        }
        writeln!(f, "#End Library")
    }
}

impl Symbol {
    /// create a new symbol
    pub fn new(name: String, reference: String) -> Symbol {
        Symbol {
            name: name,
            reference: reference,
            text_offset: 0.0,
            draw_pinnumber: false,
            draw_pinname: false,
            unit_count: 1,
            unit_locked: false,
            is_power: false,
            fields: vec![],
            draw: vec![],
        }
    }

    /// set the name of the symbol
    pub fn set_name(&mut self, name: &str) {
        if char_at(&self.name, 0) == '~' {
            self.name = format!("~{}", name)
        } else {
            self.name = name.to_string()
        }
        let mut field = &mut self.fields[1];
        field.value = name.to_string()
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        writeln!(f, "# {}", &&self.fields[1].value)?;
        writeln!(f, "#")?;
        writeln!(f, "DEF {} {} 0 {} {} {} {} {} {}",
                     self.name, self.reference, self.text_offset,
                     if self.draw_pinnumber { "Y" } else { "N" },
                     if self.draw_pinname { "Y" } else { "N" },
                     self.unit_count,
                     if self.unit_locked { "L" } else { "F" },
                     if self.is_power { "P" } else { "N" },
                     )?;
        for field in &self.fields {
            writeln!(f, "{}", field)?
        }
        writeln!(f, "DRAW")?;
        for draw in &self.draw {
            writeln!(f, "{}", draw)?
        }
        writeln!(f, "ENDDRAW")?;
        writeln!(f, "ENDDEF")?;
        writeln!(f, "#")
    }
}

impl Default for Field {
    fn default() -> Field {
        Field {
            i: 0,
            value: String::from(""),
            x: 0.0,
            y: 0.0,
            dimension: 0,
            orientation: schematic::Orientation::Horizontal,
            visible: false,
            hjustify: schematic::Justify::Center,
            vjustify: schematic::Justify::Center,
            italic: false,
            bold: false,
            name: String::from(""),
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "F{} \"{}\" {} {} {} {} {} {} {}{}{}",
               self.i, self.value, self.x, self.y, self.dimension,
               self.orientation,
               if self.visible { "V" } else { "I" },
               self.hjustify, self.vjustify,
               if self.italic { "I" } else { "N" },
               if self.bold { "I" } else { "N" },
                    )?;
        if self.i > 3 {
            write!(f, " \"{}\"", self.name)?
        };
        Ok(())
    }
}

macro_rules! assume_line {
    ($s:expr, $exp:expr) => (
        if $s.eof() {
            return str_error("end of file reached".to_string())
        }
        if $s.here() != $exp {
            return str_error(format!("expected '{}', got '{}'", $exp, $s.here()))
        }
        $s.i += 1;
    )
}

#[derive(Debug)]
struct ParseState {
    i: usize,
    v: Vec<String>,
}

impl ParseState {
    fn new(v2: Vec<&str>) -> ParseState {
        ParseState {
            i: 0,
            v: v2.iter().map(|x| String::from(*x)).collect(),
        }
    }

    fn here(&self) -> String {
        (self.v[self.i]).clone()
    }

    fn next(&mut self) {
        self.i += 1;
    }

    fn eof(&self) -> bool {
        self.i >= self.v.len()
    }
}

fn assume_string(e: &'static str, s: &str) -> Result<()> {
    if *e != *s {
        return str_error(format!("expecting: {}, actually: {}", e, s));
    }
    Ok(())
}

fn i64_from_string(p: &ParseState, s: &str) -> Result<i64> {
    match i64::from_str(s) {
        Ok(i) => Ok(i),
        _ => str_error(format!("int parse error in {}; line: {}", s, p.here())),
    }
}

fn f64_from_string(p: &ParseState, s: &str) -> Result<f64> {
    match f64::from_str(s) {
        Ok(i) => Ok(i),
        _ => str_error(format!("float parse error in {}; line: {}", s, p.here())),
    }
}

fn bool_from_string(s: &str, t: &'static str, f: &'static str) -> Result<bool> {
    if &s[..] == t {
        return Ok(true);
    }
    if &s[..] == f {
        return Ok(false);
    }
    str_error(format!("unknown boolean {}, expected {} or {}", s, t, f))
}

fn char_at(s: &str, p: usize) -> char {
    let v: Vec<char> = s.chars().collect();
    v[..][p]
}

fn parse_symbol(p: &mut ParseState) -> Result<Symbol> {
    p.next(); // skip line like # name
    assume_line!(p, "#");
    let s = p.here();
    let v = &parse_split_quote_aware(&s);
    if v.len() != 10 {
        return str_error(format!("unexpected elements in {}", s));
    }
    assume_string("DEF", &v[0])?;
    let mut s = Symbol::new(v[1].clone(), v[2].clone());
    s.text_offset = f64_from_string(p, &v[4])?;
    s.draw_pinnumber = bool_from_string(&v[5], "Y", "N")?;
    s.draw_pinname = bool_from_string(&v[6], "Y", "N")?;
    s.unit_count = i64_from_string(p, &v[7])?;
    s.unit_locked = bool_from_string(&v[8], "L", "F")?;
    s.is_power = bool_from_string(&v[9], "P", "N")?;
    // TODO fields
    // TODO draw
    p.next();
    loop {
        let s2 = p.here();
        if char_at(&s2, 0) == 'F' {
            let f = parse_field(p, &s2)?;
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
    assume_line!(p, "ENDDEF");
    assume_line!(p, "#");
    Ok(s)
}

fn bool_from<T: PartialEq + fmt::Display>(i: T, t: T, f: T) -> Result<bool> {
    if i == t {
        return Ok(true);
    }
    if i == f {
        return Ok(false);
    }
    str_error(format!("unknown boolean {}, expected {} or {}", i, t, f))
}

// F0 "L" 0 50 40 H V C CNN
fn parse_field(p: &mut ParseState, line: &str) -> Result<Field> {
    let mut f = Field::default();
    let v = &parse_split_quote_aware(line);
    if v.len() != 9 && v.len() != 10 {
        return str_error(format!("unexpected elements in {}", line));
    }
    f.i = i64_from_string(p, &String::from(&v[0][1..]))?;
    let name = if v.len() == 10 {
        v[9].clone()
    } else {
        match f.i {
            0 => String::from("Reference"),
            1 => String::from("Value"),
            2 => String::from("Footprint"),
            3 => String::from("UserDocLink"),
            _ => return str_error("expecting name for componentfield > 3".to_string()),
        }
    };
    f.value = v[1].clone();
    f.x = f64_from_string(p, &v[2])?;
    f.y = f64_from_string(p, &v[3])?;
    f.dimension = i64_from_string(p, &v[4])?;
    f.orientation = schematic::Orientation::new(char_at(&v[5], 0))?;
    f.visible = bool_from_string(&v[6], "V", "I")?;
    f.hjustify = schematic::Justify::new(char_at(&v[7], 0))?;
    f.vjustify = schematic::Justify::new(char_at(&v[8], 0))?;
    f.italic = bool_from(char_at(&v[8], 1), 'I', 'N')?;
    f.bold = bool_from(char_at(&v[8], 2), 'B', 'N')?;
    f.name = name;
    Ok(f)
}

fn parse(s: &str) -> Result<SymbolLib> {
    let mut lib = SymbolLib::default();
    let v: Vec<&str> = s.lines().collect();
    let p = &mut ParseState::new(v);
    assume_line!(p, "EESchema-LIBRARY Version 2.3");
    assume_line!(p, "#encoding utf-8");
    assume_line!(p, "#");
    while !p.eof() {
        // println!("here: {}", &p.here());
        if &p.here() == "#End Library" {
            break;
        }
        let s = parse_symbol(p)?;
        // println!("new symbol: {}", &s);
        lib.symbols.push(s)
    }
    Ok(lib)
}

/// parse a &str to a symbol lib
pub fn parse_str(s: &str) -> Result<SymbolLib> {
    parse(s)
}

/// parse a file to a symbol lib
pub fn parse_file(filename: &PathBuf) -> Result<SymbolLib> {
    let name = filename.to_str().unwrap();
    let s = read_file(name)?;
    parse(&s[..])
}
