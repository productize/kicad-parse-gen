// (c) 2016 Joost Yervante Damad <joost@productize.be>

use nom;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Schematic {
    pub filename:Option<PathBuf>,
    pub libraries:Vec<String>,
    pub description:Description,
    pub elements:Vec<Element>,
    pub sheets:Vec<Sheet>,
}

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
    pub sheet_count:i64,
}

#[derive(Debug)]
pub enum Element {
    Component(Component),
    Other(String),
}

#[derive(Debug, Clone)]
pub struct Component {
    pub name:String,
    pub reference:String,
    pub u:String, // TODO
    pub x:i64,
    pub y:i64,
    pub fields:Vec<ComponentField>,
    pub rotation:ComponentRotation,
}

#[derive(Debug,Clone)]
pub struct ComponentRotation  {
    a:i64,
    b:i64,
    c:i64,
    d:i64
}

#[derive(Debug,Clone)]
pub enum Orientation {
    Horizontal,
    Vertical
}
#[derive(Debug,Clone)]
pub enum Justify {
    Left,
    Right,
    Center,
    Bottom,
    Top,
}

#[derive(Debug,Clone)]
pub struct ComponentField {
    pub i:i64,
    pub value:String,
    pub orientation:Orientation,
    pub x:f64,
    pub y:f64,
    pub size:i64,
    pub visible:bool,
    pub hjustify:Justify,
    pub vjustify:Justify,
    pub italic:bool,
    pub bold:bool,
    pub name:String,
}

#[derive(Debug,Clone)]
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

#[derive(Debug,Clone)]
pub struct SheetLabel {
    pub name:String,
    pub form:LabelForm,
    pub side:LabelSide,
    pub x:i64,
    pub y:i64,
    pub size:i64,
}

#[derive(Debug,Clone)]
pub enum LabelForm { Input, Output, BiDi, TriState, Unspecified }

#[derive(Debug,Clone)]
pub enum LabelSide { Left, Right, Top, Bottom }

enum Part {
    Element(Element),
    Sheet(Sheet),
    Description(Description),
}

named!(i64_digit<i64>, map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));

named!(some_space, tag!(" ") ~ opt!(nom::multispace));

named!(parse_library<String>,
       delimited!(tag!("LIBS:"), nom::not_line_ending, nom::line_ending)
       );

named!(parse_libraries<Vec<String> >, many0!(parse_library));

named!(parse_description<Part>,
       );

named!(parse_elements<Vec<Element> >, many0!(parse_element));
named!(parse_sheets<Vec<Element> >, many0!(parse_sheet));

named!(parse_element<Element>, alt!(parse_component | parse_other));

/*
$Sheet
S 5250 2300 950  3100
U 5655A9F3
F0 "NRF52" 60
F1 "nrf52.sch" 60
F2 "ANT" I R 6200 2450 60 
F3 "P0.02/AIN0" I L 5250 2450 60 
F4 "P0.03/AIN1" I L 5250 2550 60 
$EndSheet
 */

named!(parse_sheet<Sheet>,
       chain!(
           tag!("$Sheet") ~ nom::line_ending ~
               tag!("S") ~ some_space ~ x:i64_digit ~ some_space ~ y:i64_digit ~
               some_space ~ dimx:i64_digit ~ some_space ~ dimy:i64_digit ~ nom::line_ending ~
               tag!("$EndSheet") ~ nom::line_ending ~   
               ,|| Sheet {
                   x:x, y:y, dimx:dimx, dimy:dimy,
               }
           )
       );

named!(parse_p_description<Part>, map!(parse_description, Part::Description));
named!(parse_p_sheet<Part>, map!(parse_sheet, Part::Sheet));
named!(parse_p_element<Part>, map!(parse_sheet, Part::Element));
named!(parse_parts<Vec<Part> >, many0!(alt!(parse_p_description | parse_p_sheet | parse_p_element)));

fn parse_schematic(filename:Option<PathBuf>, input: &[u8]) -> nom::IResult<&[u8], &str> {
    let (i,v) = try_parse!(input,
        chain!(
            tag!("EESchema Schematic File Version 2") ~ nom::line_ending ~
                libraries: parse_libraries ~
                tag!("EELAYER 25 0") ~ nom::line_ending ~
                tag!("EELAYER END") ~ nom::line_ending ~
                parts: parse_parts ~
                tag!("EndSCHEMATC") ~
                opt!(nom::multispace)
            ,||  Schematic {
                filename:filename,
                libraries:libraries,
                description:description,
                elements:elements,
                sheets:sheets,
            })
            );
    nom::IResult::Done(i, v)
}
