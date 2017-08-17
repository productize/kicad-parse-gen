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
    pub x1: i64,
    pub y1: i64,
    pub x2: i64,
    pub y2: i64,
    pub unit: i64,
    pub convert: i64,
    pub thickness: i64,
    pub fill: Fill,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Fill {
    Filled,
    DotFilled,
    Transparent,
}

impl Default for Fill {
    fn default() -> Fill {
        Fill::Transparent
    }
}

impl Fill {
    fn make(s:&str) -> Result<Fill> {
        match s {
            "F" => Ok(Fill::Filled),
            "f" => Ok(Fill::DotFilled),
            "N" => Ok(Fill::Transparent),
            _ => Err(format!("unknown fill type {}", s).into())
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
            Fill::Filled => write!(f, "F"),
            Fill::DotFilled => write!(f, "f"),
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
    fn make(s: &str) -> Result<PinOrientation> {
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
    fn make(s: &str) -> Result<PinType> {
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
    fn make(s: &str) -> Result<PinShape> {
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
    if p.here().starts_with("ALIAS") {
        let v = parse_split_quote_aware(&p.here());
        for alias in v.into_iter().skip(1) {
            s.aliases.push(alias)
        }
        p.next();
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

// X +3.3V 1 0 0 0 U 30 30 0 0 W N

fn parse_pin(p: &mut ParseState, line: &str) -> Result<Pin> {
    let mut pin = Pin::default();
    let v = &parse_split_quote_aware(line);
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
fn parse_rect(p: &mut ParseState, line: &str) -> Result<Rectangle> {
    let mut rect = Rectangle::default();
    let v = &parse_split_quote_aware(line);
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
