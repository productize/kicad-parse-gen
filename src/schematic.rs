// (c) 2016 Joost Yervante Damad <joost@productize.be>

use std::fmt;
use std::str::FromStr;

// get from parent
use ERes;
use err;
use read_file;

#[derive(Debug)]
pub struct Schematic {
    pub libraries:Vec<String>,
    pub description:Description,
    pub elements:Vec<Element>,
    pub sheets:Vec<Sheet>,
}

impl Schematic {
    fn new() -> Schematic {
        Schematic {
            libraries:vec![],
            description:Description::new(),
            elements:vec![],
            sheets:vec![],
        }
    }

    fn add_library(&mut self, s:String) {
        self.libraries.push(s)
    }

    fn set_description(&mut self, d:Description) {
        self.description = d;
    }

    fn append_component(&mut self, c:Component) {
        self.elements.push(Element::Component(c))
    }
    
    fn append_sheet(&mut self, c:Sheet) {
        self.sheets.push(c)
    }
    
    fn append_other(&mut self, c:String) {
        self.elements.push(Element::Other(c))
    }

    pub fn components(&self) -> Vec<&Component> {
        let mut v = vec![];
        for ref x in self.elements.iter() {
            match **x {
                Element::Component(ref c) => v.push(c),
                Element::Other(_) => (),
            }
        }
        v
    }
}

impl fmt::Display for Schematic {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(writeln!(f, "EESchema Schematic File Version 2"));
        for v in &self.libraries[..] {
            try!(writeln!(f, "LIBS:{}", v))
        }
        try!(writeln!(f, "EELAYER 25 0"));
        try!(writeln!(f, "EELAYER END"));
        try!(write!(f, "{}", self.description));
        for v in &self.elements[..] {
            try!(write!(f, "{}", v))
        }
        for v in &self.sheets[..] {
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
    pub size:String,
    pub dimx:i64,
    pub dimy:i64,
    pub title:String,
    pub date:String,
    pub rev:String,
    pub comp:String,
    pub comment1:String,
    pub comment2:String,
    pub comment3:String,
    pub comment4:String,
    pub sheet:i64,
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
            sheet:1,
        }
    }
}

impl fmt::Display for Description {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(writeln!(f, "$Descr {} {} {}", self.size, self.dimx, self.dimy));
        try!(writeln!(f, "encoding utf-8"));
        try!(writeln!(f, "Sheet 1 {}", self.sheet));
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
pub enum Element {
    Component(Component),
    Other(String),
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Element::Component(ref c) => write!(f, "{}", c),
            Element::Other(ref c) => write!(f, "{}\n", c),
        }
    }
}

#[derive(Debug)]
pub struct Component {
    pub name:String,
    pub reference:String,
    pub u:String, // TODO
    pub x:f64,
    pub y:f64,
    pub fields:Vec<ComponentField>,
    pub rotation:ComponentRotation,
}

impl Component {
    fn new() -> Component {
        Component {
            name:String::from("DUMMY"),
            reference:String::from("U1"),
            u:String::from(""),
            x:0.0,
            y:0.0,
            fields:vec![],
            rotation:ComponentRotation {a:0,b:0,c:0,d:0},
        }
    }

    fn set_name(&mut self, s:String) {
        self.name = s;
    }
    fn set_reference(&mut self, s:String) {
        self.reference = s;
    }
    fn set_u(&mut self, s:String) {
        self.u = s;
    }
    fn set_x(&mut self, x:f64) {
        self.x = x
    }
    fn set_y(&mut self, y:f64) {
        self.y = y
    }
    fn add_field(&mut self, f:ComponentField) {
        self.fields.push(f)
    }
    fn set_rotation(&mut self, rot:ComponentRotation) {
        self.rotation = rot;
    }
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(writeln!(f, "$Comp"));
        try!(writeln!(f, "L {} {}", self.name, self.reference));
        try!(writeln!(f, "{}", self.u));
        try!(writeln!(f, "P {} {}", self.x, self.y));
        for x in &self.fields[..] {
            try!(writeln!(f, "{}", x))
        }
        try!(writeln!(f, "\t1 {} {}", self.x, self.y));
        try!(writeln!(f, "\t{} {} {} {}", self.rotation.a, self.rotation.b, self.rotation.c, self.rotation.d));
        writeln!(f, "$EndComp")
    }
}

#[derive(Debug)]
pub struct ComponentRotation  {
    a:i64,
    b:i64,
    c:i64,
    d:i64
}

#[derive(Debug)]
pub enum Orientation {
    Horizontal,
    Vertical
}

impl Orientation {
    fn new(c:char) -> ERes<Orientation> {
        match c {
            'H' => Ok(Orientation::Horizontal),
            'V' => Ok(Orientation::Vertical),
            _ => Err(format!("unknown orientation {}", c))
        }
        
    }
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Orientation::Horizontal => write!(f, "H"),
            Orientation::Vertical => write!(f, "V"),
        }
    }
}


#[derive(Debug)]
pub enum Justify {
    Left,
    Right,
    Center,
    Bottom,
    Top,
}

impl Justify {
    fn new(c: char) -> ERes<Justify> {
        match c {
            'C' => Ok(Justify::Center),
            'R' => Ok(Justify::Right),
            'L' => Ok(Justify::Left),
            'B' => Ok(Justify::Bottom),
            'T' => Ok(Justify::Top),
            _ => Err(format!("unexpected justify: {}", c))
        }
    }
}

impl fmt::Display for Justify {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Justify::Left => write!(f, "L"),
            Justify::Right => write!(f, "R"),
            Justify::Center => write!(f, "C"),
            Justify::Bottom => write!(f, "B"),
            Justify::Top => write!(f, "T"),
        }
    }
}



#[derive(Debug)]
pub struct ComponentField {
    i:i64,
    value:String,
    orientation:Orientation,
    x:f64,
    y:f64,
    size:i64,
    visible:bool,
    hjustify:Justify,
    vjustify:Justify,
    italic:bool,
    bold:bool,
    name:Option<String>,
}

impl ComponentField {
    fn new(v:&Vec<String>) -> ERes<ComponentField> {
        if v.len() != 10 && v.len() != 11 {
            return Err(format!("expecting 10 or 11 parts got {}", v.len()))
        }
        let name = if v.len() == 11 {
            Some(try!(unquote_string(&v[10])))
        } else {
            None
        };
        let c = ComponentField {
            i:try!(i64_from_string(&v[1])),
            value:try!(unquote_string(&v[2])),
            orientation:try!(Orientation::new(char_at(&v[3],0))),
            x:try!(f64_from_string(&v[4])),
            y:try!(f64_from_string(&v[5])),
            size:try!(i64_from_string(&v[6])),
            visible:try!(bool_from_string(&v[7],"0000","0001")),
            hjustify:try!(Justify::new(char_at(&v[8], 0))),
            vjustify:try!(Justify::new(char_at(&v[9], 0))),
            italic:try!(bool_from(char_at(&v[9], 1), 'I', 'N')),
            bold:try!(bool_from(char_at(&v[9], 1), 'B', 'N')),
            name:name,
        };
        Ok(c)
    }
}

impl fmt::Display for ComponentField {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "F {} \"{}\" {} ", self.i, self.value, self.orientation));
        try!(write!(f, "{} {} {} ", self.x, self.y, self.size));
        try!(write!(f, "{} ", if self.visible { "0000" } else { "0001" }));
        try!(write!(f, "{} {}", self.hjustify, self.vjustify));
        try!(write!(f, "{}", if self.italic { 'I' } else { 'N' }));
        try!(write!(f, "{}", if self.bold { 'B' } else { 'N' }));
        match self.name {
            Some(ref name) => write!(f, " \"{}\"", name),
            None => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct Sheet {
    pub x:i64,
    pub y:i64,
    pub dimx:i64,
    pub dimy:i64,
    pub unique:String, // U timestamp field
    pub name:String, // F0
    pub name_size:i64,
    pub filename:String, // F1
    pub filename_size:i64,
    pub labels:Vec<SheetLabel>, // starting at F2
}

impl Sheet {
    fn new() -> Sheet {
        Sheet { x:0, y:0, dimx:0, dimy:0, unique:String::from(""),
                name:String::from("DUMMY"), name_size:60,
                filename:String::from(""), filename_size:60,
                labels:vec![],
        }
    }
}

impl fmt::Display for Sheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(writeln!(f, "$Sheet"));
        try!(writeln!(f, "S {} {} {} {}", self.x, self.y, self.dimx, self.dimy));
        try!(writeln!(f, "U {}", self.unique));
        try!(writeln!(f, "F0 \"{}\" {}", self.name, self.name_size));
        try!(writeln!(f, "F1 \"{}\" {}", self.filename, self.filename_size));
        let mut i = 2;
        for label in &self.labels[..] {
            try!(writeln!(f, "F{} {}", i, label));
            i += 1;
        }
        writeln!(f, "$EndSheet")
    }
}

// F3 "P0.02/AIN0" I L 5250 2450 60
// form = I (input) O (output) B (BiDi) T (tri state) U (unspecified)
// side = R (right) , L (left)., T (tpo) , B (bottom)
#[derive(Debug)]
pub struct SheetLabel {
    pub name:String,
    pub form:LabelForm,
    pub side:LabelSide,
    pub x:i64,
    pub y:i64,
    pub size:i64,
}

impl SheetLabel {
    fn new() -> SheetLabel {
        SheetLabel { name:String::from(""), form:LabelForm::Input,
                     side:LabelSide::Left, x:0, y:0, size:60,
        }
    }
}

impl fmt::Display for SheetLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "\"{}\" {} {} {} {} {}", self.name, self.form, self.side, self.x, self.y, self.size)
    }
}

#[derive(Debug)]
pub enum LabelForm { Input, Output, BiDi, TriState, Unspecified }

impl fmt::Display for LabelForm {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let c = match *self {
            LabelForm::Input => 'I',
            LabelForm::Output => 'O',
            LabelForm::BiDi => 'B',
            LabelForm::TriState => 'T',
            LabelForm::Unspecified => 'U',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug)]
pub enum LabelSide { Left, Right, Top, Bottom }

impl fmt::Display for LabelSide {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let c = match *self {
            LabelSide::Left => 'L',
            LabelSide::Right => 'R',
            LabelSide::Top => 'T',
            LabelSide::Bottom => 'B',
        };
        write!(f, "{}", c)
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

fn parse_split_n(n:usize, s:&String) -> ERes<Vec<String>> {
    let sp:Vec<&str> = s.splitn(n, ' ').collect();
    if sp.len() < n {
        return Err(format!("expecting {} elements in {}", n, s))
    }
    return Ok(sp.iter().map(|x| String::from(*x)).collect())
}

fn parse_split_quote_aware(s:&String) -> Vec<String> {
    let mut v:Vec<String> = vec![];
    let mut in_q:bool = false;
    let mut s2:String = String::from("");
    for c in s.chars() {
        if !in_q && c == '\"' {
            in_q = true;
            s2.push(c);
            continue
        }
        if in_q && c == '\"' {
            in_q = false;
            s2.push(c);
            continue
        }
        if c == ' ' {
            if s2.len() > 0 {
                v.push(s2.clone());
                s2.clear();
            }
            continue;
        }
        s2.push(c);
    }
    if s2.len() > 0 {
        v.push(s2.clone())
    }
    return v
    
}

fn i64_from_string(s:&String) -> ERes<i64> {
    match i64::from_str(&s[..]) {
        Ok(i) => Ok(i),
        _ => Err(format!("int parse error in {}", s))
    }
}

fn f64_from_string(s:&String) -> ERes<f64> {
    match f64::from_str(&s[..]) {
        Ok(i) => Ok(i),
        _ => Err(format!("float parse error in {}", s))
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
    where F:Fn(&mut Description, String) -> ()
{
    let v = try!(parse_split_n(2, s));
    try!(assume_string(name, &v[0]));
    setter(d, try!(unquote_string(&v[1])));
    Ok(())
}
    
    
fn parse_description(p:&mut ParseState) -> ERes<Description> {
    let mut d = Description::new();
    let v = try!(parse_split_n(4, &p.here()));
    d.size = v[1].clone();
    d.dimx = try!(i64_from_string(&v[2]));
    d.dimy = try!(i64_from_string(&v[3]));
    p.next(); // $Descr
    p.next(); // encoding
    let v = try!(parse_split_n(3, &p.here()));
    if v[0] != "Sheet" { return Err(String::from("Expecting 'Sheet'")) };
    if v[1] != "1" { return Err(String::from("Expecting 'Sheet 1'")) };
    d.sheet = try!(i64_from_string(&v[2]));
    p.next(); // Sheet
    try!(word_and_qstring(&mut d, "Title", &p.here(), |d, x| d.title = x));
    p.next();
    try!(word_and_qstring(&mut d, "Date", &p.here(), |d, x| d.date = x));
    p.next();
    try!(word_and_qstring(&mut d, "Rev", &p.here(), |d, x| d.rev = x));
    p.next();
    try!(word_and_qstring(&mut d, "Comp", &p.here(), |d, x| d.comp = x));
    p.next();
    try!(word_and_qstring(&mut d, "Comment1", &p.here(), |d, x| d.comment1 = x));
    p.next();
    try!(word_and_qstring(&mut d, "Comment2", &p.here(), |d, x| d.comment2 = x));
    p.next();
    try!(word_and_qstring(&mut d, "Comment3", &p.here(), |d, x| d.comment3 = x));
    p.next();
    try!(word_and_qstring(&mut d, "Comment4", &p.here(), |d, x| d.comment4 = x));
    p.next();
    try!(assume_string("$EndDescr", &p.here()));
    Ok(d)
}

fn parse_component_l(p:&mut ParseState, d:&mut Component) -> ERes<()> {
    let v = try!(parse_split_n(3, &p.here()));
    d.set_name(v[1].clone());
    d.set_reference(v[2].clone());
    Ok(())
}

fn parse_component_u(p:&mut ParseState, d:&mut Component) -> ERes<()> {
    d.set_u(p.here());
    Ok(())
}

fn parse_component_p(p:&mut ParseState, d:&mut Component) -> ERes<()> {
    let v = try!(parse_split_n(3, &p.here()));
    d.set_x(try!(f64_from_string(&v[1])));
    d.set_y(try!(f64_from_string(&v[2])));
    Ok(())
}

fn parse_component_f(p:&mut ParseState, d:&mut Component) -> ERes<()> {
    //println!("{}", p.here());
    let v = parse_split_quote_aware(&p.here());
    let f = try!(ComponentField::new(&v));
    d.add_field(f);
    Ok(())
}

fn parse_component_rotation(p:&mut ParseState, d:&mut Component) -> ERes<()> {
    p.next(); // skip redundant position line
    let s = p.here();
    let v:Vec<&str> = s.split_whitespace().collect();
    if v.len() != 4 {
        return Err(format!("expecting 4 elements in {}", s))
    }
    let a = try!(i64_from_string(&String::from(v[0])));
    let b = try!(i64_from_string(&String::from(v[1])));
    let c = try!(i64_from_string(&String::from(v[2])));
    let d1 = try!(i64_from_string(&String::from(v[3])));
    let rot = ComponentRotation { a:a, b:b, c:c, d:d1 };
    d.set_rotation(rot);
    Ok(())
}

fn parse_component(p:&mut ParseState) -> ERes<Component> {
    let mut d = Component::new();
    p.next();
    loop {
        let s = p.here();
        if s == "$EndComp" {
            break;
        }
        match s.split_whitespace().next() {
            Some("L") => try!(parse_component_l(p, &mut d)),
            Some("U") => try!(parse_component_u(p, &mut d)),
            Some("P") => try!(parse_component_p(p, &mut d)),
            Some("F") => try!(parse_component_f(p, &mut d)),
            Some("1") => try!(parse_component_rotation(p, &mut d)),
            Some("0") => try!(parse_component_rotation(p, &mut d)),
            _ => println!("skipping unknown component line {}", s)
        }
        p.next()
    }
    Ok(d)
}

fn parse_sheet(p:&mut ParseState) -> ERes<Sheet> {
    Ok(Sheet::new())
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
            sch.add_library(String::from(&s[5..]));
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
                    sch.set_description(d)
                },
                Some("$Comp") => {
                    let d = try!(parse_component(p));
                    sch.append_component(d)
                },
                Some("$Sheet") => {
                    let d = try!(parse_sheet(p));
                    sch.append_sheet(d)
                },
                Some(_) => {
                    sch.append_other(p.here())
                },
                None => unreachable!()
            }
        }
        p.next()
    }
    Ok(sch)
}


pub fn parse_str(s: &str) -> ERes<Schematic> {
    parse(s)
}

pub fn parse_file(name: &str) -> ERes<Schematic> {
    let s = try!(match read_file(name) {
        Ok(s) => Ok(s),
        Err(x) => Err(format!("io error: {}", x))
    });
    parse(&s[..])
}

