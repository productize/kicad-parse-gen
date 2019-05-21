// (c) 2016 Productize SPRL <joost@productize.be>

// extension: .lib
// format: old-style

use std::fmt;
use std::result;
use std::str::FromStr;
use std::path::PathBuf;

// get from parent
use util::read_file;
use parse_split_quote_aware;
use schematic;
use str_error;
use checkfix::{self, CheckFix, CheckFixData, Config};
use KicadError;

/// a Kicad symbolic file
#[derive(Debug, Default)]
pub struct SymbolLib {
    /// the symbols
    pub symbols: Vec<Symbol>,
}

// DEF name reference unused text_offset draw_pinnumber draw_pinname unit_count units_locked option_flag
/// a symbol
#[derive(Debug, Clone)]
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
    /// aliases
    pub aliases: Vec<String>,
    /// draw
    pub draw: Vec<Draw>,
}
// F n “text” posx posy dimension orientation visibility hjustify vjustify/italic/bold “name”
// F0 "#PWR" 0 0 30 H I C CNN

/// a field
#[derive(Debug, Clone)]
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

/// a drawing
#[derive(Debug, Clone)]
pub enum Draw {
    /// a pin
    Pin(Pin),
    /// a rectangle
    Rectangle(Rectangle),
    /// a non-parsed drawing part
    Other(String),
}

// U (up) D (down) R (right) L (left).
/// pin orientation
#[derive(Debug, Clone, PartialEq)]
pub enum PinOrientation {
    /// up
    Up, // U
    /// down
    Down, // D
    /// left
    Left, // L
    /// right
    Right, // R
}

/// pin type
#[derive(Debug, Clone, PartialEq)]
pub enum PinType {
    /// input
    Input, // I
    /// output
    Output, // O
    /// bidi
    Bidi, // B
    /// tristate
    Tristate, // T
    /// passive
    Passive, // P
    /// unspecified
    Unspecified, // U
    /// power output
    PowerInput, // W
    /// power input
    PowerOutput, // w
    /// open collector
    OpenCollector, // C
    /// open emitter
    OpenEmitter, // E
    /// not connected
    NotConnected, // N
}

/// pin shape
#[derive(Debug, Clone, PartialEq)]
pub enum PinShape {
    /// line
    Line, // None (default)
    /// inverted
    Inverted, // I
    /// clock
    Clock, // C
    /// inverted clock
    InvertedClock, // CI
    /// input low
    InputLow, // L
    /// clock low
    ClockLow, // CL
    /// output low
    OutputLow, // V
    /// falling edge clock
    FallingEdgeClock, // F
    /// non logic
    NonLogic, // X
}

// X name number posx posy length orientation Snum Snom unit convert Etype [shape].
// X P1 1 -200 200 150 R 50 50 1 1 P
// X +3.3V 1 0 0 0 U 30 30 0 0 W N
/// draw a pin
#[derive(Debug, Clone, Default)]
pub struct Pin {
    /// name of the pin
    pub name: String,
    /// number of the pin, which doesn't have to be an actual number
    pub number: String,
    /// x position of the pin
    pub x: i64,
    /// y position of the pin
    pub y: i64,
    /// length of the pin
    pub len: i64,
    /// orientation of the pin
    pub orientation: PinOrientation,
    /// pin number text size
    pub num_size: i64,
    /// pin name text size
    pub name_size: i64,
    /// unit ??
    pub unit: i64,
    /// convert ??
    pub convert: i64,
    /// pin type
    pub pin_type: PinType,
    /// pin visible
    pub pin_visible: bool,
    /// pin shape
    pub pin_shape: PinShape,
}

// S -800 1200 800 -1200 0 1 10 f
// S startx starty endx endy unit convert thickness cc
// cc = N F or F ( F = filled Rectangle,; f = . filled Rectangle, N = transparent background)
/// draw a rectangle
#[derive(Debug, Clone, Default)]
pub struct Rectangle {
    /// x-coordinate of first corner of rectangle
    pub x1: i64,
    /// y-coordinate of first corner of rectangle
    pub y1: i64,
    /// x-coordinate of second corner of rectangle
    pub x2: i64,
    /// y-coordinate of second corner of rectangle
    pub y2: i64,
    /// unit ??
    pub unit: i64,
    /// convert ???
    pub convert: i64,
    /// thickness of the line
    pub thickness: i64,
    /// `Fill` of the rectangle
    pub fill: Fill,
}

#[derive(Debug, Clone, PartialEq)]
/// fill for a rectangle
pub enum Fill {
    /// filled with foreground color
    FilledForeground,
    /// filled with background color
    FilledBackground,
    /// not filled
    Transparent,
}

impl Default for Fill {
    fn default() -> Fill {
        Fill::Transparent
    }
}

impl Fill {
    fn make(s: &str) -> Result<Fill, KicadError> {
        match s {
            "F" => Ok(Fill::FilledForeground),
            "f" => Ok(Fill::FilledBackground),
            "N" => Ok(Fill::Transparent),
            _ => Err(format!("unknown fill type {}", s).into()),
        }
    }
}

impl SymbolLib {
    /// find a symbol in a symbol lib
    pub fn find<F>(&self, filter: F) -> Option<&Symbol>
    where
        F: Fn(&Symbol) -> bool,
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
        writeln!(f, "EESchema-LIBRARY Version 2.4")?;
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
            aliases: vec![],
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
        let field = &mut self.fields[1];
        field.value = name.to_string()
    }

    /// get the list of pins on the symbol
    pub fn pins(&self) -> Vec<&Pin> {
        let mut v: Vec<&Pin> = vec![];
        for d in &self.draw {
            if let Draw::Pin(ref pin) = *d {
                v.push(pin)
            }
        }
        v
    }

    /// is a symbol a power symbol?
    pub fn is_power(&self) -> bool {
        self.reference.as_str() == "#PWR" && self.pins().len() == 1
    }

    /// is a symbol a graphics item?
    pub fn is_graphics(&self) -> bool {
        self.reference.starts_with('#') && self.pins().is_empty()
    }

    /// is a symbol a basic symbol?
    pub fn is_basic(&self) -> bool {
        match self.name.as_str() {
            "L" | "R" | "C" | "D" => true,
            _ => false,
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        writeln!(f, "# {}", &&self.fields[1].value)?;
        writeln!(f, "#")?;
        writeln!(
            f,
            "DEF {} {} 0 {} {} {} {} {} {}",
            self.name,
            self.reference,
            self.text_offset,
            if self.draw_pinnumber { "Y" } else { "N" },
            if self.draw_pinname { "Y" } else { "N" },
            self.unit_count,
            if self.unit_locked { "L" } else { "F" },
            if self.is_power { "P" } else { "N" },
        )?;
        for field in &self.fields {
            writeln!(f, "{}", field)?
        }
        if !self.aliases.is_empty() {
            write!(f, "ALIAS")?;
            for alias in &self.aliases {
                write!(f, " ")?;
                write!(f, "{}", alias)?;
            }
            writeln!(f, "")?;
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
        write!(
            f,
            "F{} \"{}\" {} {} {} {} {} {} {}{}{}",
            self.i,
            self.value,
            self.x,
            self.y,
            self.dimension,
            self.orientation,
            if self.visible { "V" } else { "I" },
            self.hjustify,
            self.vjustify,
            if self.italic { "I" } else { "N" },
            if self.bold { "I" } else { "N" },
        )?;
        if self.i > 3 {
            write!(f, " \"{}\"", self.name)?
        };
        Ok(())
    }
}
impl fmt::Display for Draw {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match *self {
            Draw::Other(ref s) => write!(f, "{}", s),
            Draw::Pin(ref p) => write!(f, "{}", p),
            Draw::Rectangle(ref p) => write!(f, "{}", p),
        }
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "S {} {} ", self.x1, self.y1)?;
        write!(f, "{} {} ", self.x2, self.y2)?;
        write!(f, "{} {} ", self.unit, self.convert)?;
        write!(f, "{} {}", self.thickness, self.fill)
    }
}

impl fmt::Display for Fill {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match *self {
            Fill::FilledForeground => write!(f, "F"),
            Fill::FilledBackground => write!(f, "f"),
            Fill::Transparent => write!(f, "N"),
        }
    }
}


impl fmt::Display for Pin {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "X {} {} ", self.name, self.number)?;
        write!(f, "{} {} {} ", self.x, self.y, self.len)?;
        write!(f, "{} ", self.orientation)?;
        write!(f, "{} {} ", self.num_size, self.name_size)?;
        write!(f, "{} {} ", self.unit, self.convert)?;
        write!(f, "{}", self.pin_type)?;
        if self.pin_visible {
            if self.pin_shape != PinShape::Line {
                write!(f, " {}", self.pin_shape)
            } else {
                Ok(())
            }
        } else {
            write!(f, " N{}", self.pin_shape)
        }
    }
}

impl fmt::Display for PinOrientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match *self {
            PinOrientation::Up => write!(f, "{}", 'U'),
            PinOrientation::Down => write!(f, "{}", 'D'),
            PinOrientation::Left => write!(f, "{}", 'L'),
            PinOrientation::Right => write!(f, "{}", 'R'),
        }
    }
}

impl Default for PinOrientation {
    fn default() -> PinOrientation {
        PinOrientation::Up
    }
}

impl PinOrientation {
    fn make(s: &str) -> Result<PinOrientation, KicadError> {
        match s {
            "U" => Ok(PinOrientation::Up),
            "D" => Ok(PinOrientation::Down),
            "L" => Ok(PinOrientation::Left),
            "R" => Ok(PinOrientation::Right),
            _ => Err(format!("unknown pin orientation {}", s).into()),
        }
    }
}

impl fmt::Display for PinType {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match *self {
            PinType::Input => write!(f, "{}", 'I'),
            PinType::Output => write!(f, "{}", 'O'),
            PinType::Bidi => write!(f, "{}", 'B'),
            PinType::Tristate => write!(f, "{}", 'T'),
            PinType::Passive => write!(f, "{}", 'P'),
            PinType::Unspecified => write!(f, "{}", 'U'),
            PinType::PowerInput => write!(f, "{}", 'W'),
            PinType::PowerOutput => write!(f, "{}", 'w'),
            PinType::OpenCollector => write!(f, "{}", 'C'),
            PinType::OpenEmitter => write!(f, "{}", 'E'),
            PinType::NotConnected => write!(f, "{}", 'N'),
        }
    }
}

impl Default for PinType {
    fn default() -> PinType {
        PinType::Input
    }
}

impl PinType {
    fn make(s: &str) -> Result<PinType, KicadError> {
        match s {
            "I" => Ok(PinType::Input),
            "O" => Ok(PinType::Output),
            "B" => Ok(PinType::Bidi),
            "T" => Ok(PinType::Tristate),
            "P" => Ok(PinType::Passive),
            "U" => Ok(PinType::Unspecified),
            "W" => Ok(PinType::PowerInput),
            "w" => Ok(PinType::PowerOutput),
            "C" => Ok(PinType::OpenCollector),
            "E" => Ok(PinType::OpenEmitter),
            "N" => Ok(PinType::NotConnected),
            _ => Err(format!("unknown pin type {}", s).into()),
        }
    }
}

impl fmt::Display for PinShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match *self {
            PinShape::Line => Ok(()),
            PinShape::Inverted => write!(f, "{}", 'I'),
            PinShape::Clock => write!(f, "{}", 'C'),
            PinShape::InvertedClock => write!(f, "{}", "CI"),
            PinShape::InputLow => write!(f, "{}", 'L'),
            PinShape::ClockLow => write!(f, "{}", "CL"),
            PinShape::OutputLow => write!(f, "{}", "V"),
            PinShape::FallingEdgeClock => write!(f, "{}", "F"),
            PinShape::NonLogic => write!(f, "{}", "X"),
        }
    }
}

impl Default for PinShape {
    fn default() -> PinShape {
        PinShape::Line
    }
}

impl PinShape {
    fn make(s: &str) -> Result<PinShape, KicadError> {
        if s.is_empty() {
            Ok(PinShape::Line)
        } else {
            let s = if s.starts_with('N') { &s[1..] } else { &s[..] };
            match s {
                "I" => Ok(PinShape::Inverted),
                "C" => Ok(PinShape::Clock),
                "CI" => Ok(PinShape::InvertedClock),
                "L" => Ok(PinShape::InputLow),
                "CL" => Ok(PinShape::ClockLow),
                "V" => Ok(PinShape::OutputLow),
                "F" => Ok(PinShape::FallingEdgeClock),
                "X" => Ok(PinShape::NonLogic),
                "" => Ok(PinShape::Line),
                _ => Err(format!("unknown pinshape {}", s).into()),
            }
        }
    }

    fn visible_from_str(s: &str) -> bool {
        if s.is_empty() {
            false
        } else {
            !s.starts_with('N')
        }
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

fn assume_string(e: &'static str, s: &str) -> Result<(), KicadError> {
    if *e != *s {
        return str_error(format!("expecting: {}, actually: {}", e, s));
    }
    Ok(())
}

fn i64_from_string(p: &ParseState, s: &str) -> Result<i64, KicadError> {
    match i64::from_str(s) {
        Ok(i) => Ok(i),
        _ => str_error(format!("int parse error in {}; line: {}", s, p.here())),
    }
}

fn f64_from_string(p: &ParseState, s: &str) -> Result<f64, KicadError> {
    match f64::from_str(s) {
        Ok(i) => Ok(i),
        _ => str_error(format!("float parse error in {}; line: {}", s, p.here())),
    }
}

fn bool_from_string(s: &str, t: &'static str, f: &'static str) -> Result<bool, KicadError> {
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

fn parse_symbol(p: &mut ParseState) -> Result<Symbol, KicadError> {
    p.next(); // skip line like # name
    assume_line!(p, "#");
    let s = p.here();
    let v = &parse_split_quote_aware(&s)?;
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

    loop {
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
        } else if p.here().starts_with("ALIAS") {
            let v = parse_split_quote_aware(&p.here())?;
            for alias in v.into_iter().skip(1) {
                s.aliases.push(alias)
            }
            p.next();
        } else {
            break;
        }
    }

    // TODO draw
    assume_line!(p, "DRAW");
    while !p.eof() {
        let s2 = p.here();
        if &s2 == "ENDDRAW" {
            p.next();
            break;
        }

        if s2.starts_with("X ") {
            let pin = parse_pin(p, &s2)?;
            s.draw.push(Draw::Pin(pin));
        } else if s2.starts_with("S ") {
            let rect = parse_rect(p, &s2)?;
            s.draw.push(Draw::Rectangle(rect));
        } else {
            s.draw.push(Draw::Other(s2.clone()));
        }
        p.next()
    }
    assume_line!(p, "ENDDEF");
    assume_line!(p, "#");
    Ok(s)
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn bool_from<T: PartialEq + fmt::Display>(i: T, t: T, f: T) -> Result<bool, KicadError> {
    if i == t {
        return Ok(true);
    }
    if i == f {
        return Ok(false);
    }
    str_error(format!("unknown boolean {}, expected {} or {}", i, t, f))
}

// F0 "L" 0 50 40 H V C CNN
fn parse_field(p: &mut ParseState, line: &str) -> Result<Field, KicadError> {
    let mut f = Field::default();
    let v = &parse_split_quote_aware(line)?;
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

// X +3.3V 1 0 0 0 U 30 30 0 0 W N

fn parse_pin(p: &mut ParseState, line: &str) -> Result<Pin, KicadError> {
    let mut pin = Pin::default();
    let v = &parse_split_quote_aware(line)?;
    if v.len() != 12 && v.len() != 13 {
        return str_error(format!("unexpected elements in {}", line));
    }
    pin.name = v[1].clone();
    pin.number = v[2].clone();
    pin.x = i64_from_string(p, &v[3])?;
    pin.y = i64_from_string(p, &v[4])?;
    pin.len = i64_from_string(p, &v[5])?;
    pin.orientation = PinOrientation::make(&v[6])?;
    pin.num_size = i64_from_string(p, &v[7])?;
    pin.name_size = i64_from_string(p, &v[8])?;
    pin.unit = i64_from_string(p, &v[9])?;
    pin.convert = i64_from_string(p, &v[10])?;
    pin.pin_type = PinType::make(&v[11])?;
    pin.pin_visible = true;
    if v.len() == 13 {
        pin.pin_visible = PinShape::visible_from_str(&v[12]);
        pin.pin_shape = PinShape::make(&v[12])?;
    }
    Ok(pin)
}

// S -800 1200 800 -1200 0 1 10 f
fn parse_rect(p: &mut ParseState, line: &str) -> Result<Rectangle, KicadError> {
    let mut rect = Rectangle::default();
    let v = &parse_split_quote_aware(line)?;
    if v.len() != 9 {
        return str_error(format!("unexpected elements in {}", line));
    }
    rect.x1 = i64_from_string(p, &v[1])?;
    rect.y1 = i64_from_string(p, &v[2])?;
    rect.x2 = i64_from_string(p, &v[3])?;
    rect.y2 = i64_from_string(p, &v[4])?;
    rect.unit = i64_from_string(p, &v[5])?;
    rect.convert = i64_from_string(p, &v[6])?;
    rect.thickness = i64_from_string(p, &v[7])?;
    rect.fill = Fill::make(&v[8])?;
    Ok(rect)
}

fn parse(s: &str) -> Result<SymbolLib, KicadError> {
    let mut lib = SymbolLib::default();
    let v: Vec<&str> = s.lines().collect();
    let p = &mut ParseState::new(v);
    assume_line!(p, "EESchema-LIBRARY Version 2.4");
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
pub fn parse_str(s: &str) -> Result<SymbolLib, KicadError> {
    parse(s)
}

/// parse a file to a symbol lib
pub fn parse_file(filename: &PathBuf) -> Result<SymbolLib, KicadError> {
    let name = filename.to_str().unwrap();
    let s = read_file(name)?;
    parse(&s[..])
}

struct SymbolField<'a> {
    symbol: &'a Symbol,
    field: &'a Field,
}

impl<'a> CheckFix for SymbolField<'a> {
    fn check(&self, _: &Config) -> Vec<CheckFixData> {
        let symbol = self.symbol;
        let field = self.field;
        let mut v = vec![];
        // 4.8 All text fields use a common size of 50mils (1.27mm)
        if field.dimension != 50 {
            v.push(CheckFixData::new(4, 8, field, "field text is not 50mil"));
        }
        if field.i == 0 {
            // 4.9 The Reference field contains the appropriate Reference Designator
            if symbol.is_graphics() {
                if field.visible {
                    v.push(CheckFixData::new(
                        4,
                        9,
                        symbol.name.clone(),
                        "reference field should be invisible for graphics",
                    ));
                }
            } else {
                if !field.visible && !symbol.is_power() {
                    v.push(CheckFixData::new(
                        4,
                        9,
                        symbol.name.clone(),
                        "reference field should be visible for normal symbols",
                    ));
                }
            }
        } else if field.i == 1 {
            // 4.9 The Value field contains the name of the symbol and is visible. For power and graphical symbols, the value field must be invisible
            if symbol.is_graphics() {
                if field.visible {
                    v.push(CheckFixData::new(
                        4,
                        9,
                        symbol.name.clone(),
                        "value field should be invisible for graphics",
                    ));
                }
            } else if symbol.is_power() {
                if field.visible {
                    v.push(CheckFixData::new(
                        4,
                        9,
                        symbol.name.clone(),
                        "value field should be invisible for power",
                    ));
                }
            } else {
                if !field.visible {
                    v.push(CheckFixData::new(
                        4,
                        9,
                        symbol.name.clone(),
                        "value field should be visible for normal symbols",
                    ));
                }
            }
        } else if field.i == 2 {
            // 4.9 The Footprint field is filled according to rule 4.12 (below) and is invisible
            if field.visible {
                v.push(CheckFixData::new(
                    4,
                    9,
                    symbol.name.clone(),
                    "Footprint field should be invisible",
                ));
            }
        } else if field.i == 3 {
            // 4.9 The Datasheet field is left blank and is invisible
            if field.visible {
                v.push(CheckFixData::new(
                    4,
                    9,
                    symbol.name.clone(),
                    "Datasheet field should be invisible",
                ));
            }
        }
        v
    }
}

impl CheckFix for Pin {
    fn check(&self, _: &Config) -> Vec<CheckFixData> {
        let mut v = vec![];
        let name = format!("{}:{}", self.name, self.number);
        // 4.1 Using a 100mil grid, pin origin must lie on grid nodes (IEC-60617)
        if (self.x % 10) != 0 {
            v.push(CheckFixData::new(
                4,
                1,
                name.clone(),
                "pin x not on 100mil grid",
            ));
        }
        if (self.y % 10) != 0 {
            v.push(CheckFixData::new(
                4,
                1,
                name.clone(),
                "pin y not on 100mil grid",
            ));
        }
        // 4.1 Pin length can be incremented in steps of 50mils (1.27mm) if required e.g. for long pin numbers
        if (self.len % 5) != 0 {
            v.push(CheckFixData::new(
                4,
                1,
                name.clone(),
                "pin length not on 50mil grid",
            ));
        }
        // 4.1 Pins should have a length of at least 100mils (2.54mm)
        if self.len < 100 {
            v.push(CheckFixData::info(
                4,
                1,
                name.clone(),
                "pin length < 100mil",
            ));
        }
        // 4.1 Pin length should not be more than 300mils (7.62mm)
        if self.len > 300 {
            v.push(CheckFixData::info(
                4,
                1,
                name.clone(),
                "pin length > 300mil",
            ));
        }
        // 4.7 NC pins should be of type NC
        if self.name.to_lowercase().contains("nc") {
            if self.pin_type != PinType::NotConnected {
                v.push(CheckFixData::new(
                    4,
                    7,
                    name.clone(),
                    "Pin should be of type Not Connected",
                ))
            }
        }
        // 4.7 NC pins should be invisible, others should be visible
        if self.pin_type == PinType::NotConnected {
            if self.pin_visible {
                v.push(CheckFixData::new(
                    4,
                    7,
                    name.clone(),
                    "Pin should be invisible",
                ))
            }
        } else {
            if !self.pin_visible {
                v.push(CheckFixData::new(
                    4,
                    7,
                    name.clone(),
                    "Pin should be visible",
                ))
            }
        }
        // 4.8 All text fields use a common size of 50mils (1.27mm)
        if self.num_size != 50 {
            v.push(CheckFixData::new(
                4,
                8,
                name.clone(),
                "Pin Number should be 50mil",
            ))
        }
        // 4.8 All text fields use a common size of 50mils (1.27mm)
        if self.name_size != 50 {
            v.push(CheckFixData::new(
                4,
                8,
                name.clone(),
                "Pin Name should be 50mil",
            ))
        }
        v
    }
}
impl CheckFix for Rectangle {
    fn check(&self, _: &Config) -> Vec<CheckFixData> {
        let mut v = vec![];
        // 4.2 Fill style of symbol body is set to Fill background
        if self.fill != Fill::FilledBackground {
            v.push(CheckFixData::new(4, 2, self, "Rectangle is not filled"))
        }
        // 4.2 Symbol body has a line width of 10mils (0.254mm)
        if self.thickness != 10 {
            v.push(CheckFixData::new(
                4,
                2,
                self,
                "Rectangle is not using a 10mil line",
            ))
        }
        // TODO 4.2 Origin is placed in the middle of symbol
        // TODO 4.2 IEC-style symbols are used whenever possibl
        v
    }
}

impl CheckFix for Draw {
    fn check(&self, config: &Config) -> Vec<CheckFixData> {
        let mut v = vec![];
        match *self {
            Draw::Pin(ref pin) => {
                let p = pin.check(config);
                for i in p {
                    v.push(i)
                }
            }
            Draw::Rectangle(ref rect) => {
                let p = rect.check(config);
                for i in p {
                    v.push(i)
                }
            }
            Draw::Other(_) => (),
        }
        v
    }
}

impl CheckFix for Symbol {
    fn check(&self, config: &Config) -> Vec<CheckFixData> {
        let mut v = vec![];
        // 1.7 valid name
        let name = if self.name.starts_with('~') {
            self.name.chars().skip(1).collect::<String>()
        } else {
            self.name.clone()
        };
        let allowed_1_7 = checkfix::allowed_1_7_items(&name);
        if !allowed_1_7.is_empty() {
            v.push(CheckFixData::More(allowed_1_7).flatter())
        }
        for field in &self.fields {
            let f = SymbolField {
                symbol: self,
                field: field,
            };
            let f = f.check(config);
            if !f.is_empty() {
                v.push(CheckFixData::More(f).flatter())
            }
        }
        if !(self.is_power() || self.is_graphics()) {
            for draw in &self.draw {
                let f = draw.check(config);
                if !f.is_empty() {
                    v.push(CheckFixData::More(f).flatter())
                }
            }
        }
        v
    }
}
