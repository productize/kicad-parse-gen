// (c) 2016-2017 Productize SPRL <joost@productize.be>

// extension: .sch
// format: old-style

use std::fmt;
use std::str::FromStr;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

// get from parent
use {Bound, BoundingBox, KicadError};
use util::read_file;
use str_error;
use parse_split_quote_aware;
use parse_split_quote_aware_n;

/// a Kicad schematic
#[derive(Debug, Default)]
pub struct Schematic {
    /// filename of the schematic
    pub filename: Option<PathBuf>,
    /// list of libraries referenced
    pub libraries: Vec<String>,
    /// description
    pub description: Description,
    /// elements contained in the schematic
    pub elements: Vec<Element>,
    /// nested sheets contained in the schematic
    pub sheets: Vec<Sheet>,
    /// eelayer is transparently copied
    pub eelayer: String,
}

impl BoundingBox for Schematic {
    fn bounding_box(&self) -> Bound {
        let mut b = Bound::default();
        for e in &self.elements {
            b.update(&e.bounding_box());
        }
        for s in &self.sheets {
            b.update(&s.bounding_box());
        }
        b.swap_if_needed();
        b
    }
}

impl Schematic {
    fn add_library(&mut self, s: String) {
        self.libraries.push(s)
    }

    fn set_description(&mut self, d: Description) {
        self.description = d;
    }

    fn append_component(&mut self, c: Component) {
        self.elements.push(Element::Component(c))
    }

    fn append_wire(&mut self, w: Wire) {
        self.elements.push(Element::Wire(w))
    }

    fn append_connection(&mut self, w: Connection) {
        self.elements.push(Element::Connection(w))
    }

    fn append_no_connect(&mut self, w: NoConnect) {
        self.elements.push(Element::NoConnect(w))
    }

    fn append_text(&mut self, w: Text) {
        self.elements.push(Element::Text(w))
    }

    fn append_sheet(&mut self, c: Sheet) {
        self.sheets.push(c)
    }

    fn append_other(&mut self, c: String) {
        self.elements.push(Element::Other(c))
    }

    /// modify the component by name
    pub fn modify_component<F>(&mut self, reference: &str, fun: F)
    where
        F: Fn(&mut Component) -> (),
    {
        for x in &mut self.elements[..] {
            match *x {
                Element::Component(ref mut c) => if c.reference == *reference {
                    return fun(c);
                },
                Element::Wire(_) |
                Element::Connection(_) |
                Element::NoConnect(_) |
                Element::Text(_) |
                Element::Other(_) => (),
            }
        }
    }

    /// modify all components
    pub fn modify_components<F>(&mut self, fun: F)
    where
        F: Fn(&mut Component) -> (),
    {
        for x in &mut self.elements[..] {
            match *x {
                Element::Component(ref mut c) => fun(c),
                Element::Wire(_) |
                Element::Connection(_) |
                Element::NoConnect(_) |
                Element::Text(_) |
                Element::Other(_) => (),
            }
        }
    }

    /// collect all components in a list
    pub fn collect_components(&self, v: &mut Vec<Component>) {
        for x in &self.elements {
            match *x {
                Element::Component(ref c) => v.push(c.clone()),
                Element::Wire(_) |
                Element::Connection(_) |
                Element::NoConnect(_) |
                Element::Text(_) |
                Element::Other(_) => (),
            }
        }
    }

    /// return all components in a new list (redundant?)
    pub fn components(&self) -> Vec<Component> {
        let mut v = vec![];
        for x in &self.elements {
            match *x {
                Element::Component(ref c) => v.push(c.clone()),
                Element::Wire(_) |
                Element::Connection(_) |
                Element::NoConnect(_) |
                Element::Text(_) |
                Element::Other(_) => (),
            }
        }
        v
    }

    /// return all components including from sub-sheets
    pub fn all_components(&self) -> Result<Vec<Component>, KicadError> {
        let mut v = vec![];
        for x in &self.elements {
            match *x {
                Element::Component(ref c) => v.push(c.clone()),
                Element::Wire(_) |
                Element::Connection(_) |
                Element::NoConnect(_) |
                Element::Text(_) |
                Element::Other(_) => (),
            }
        }
        for sheet in &self.sheets {
            let schematic = parse_file_for_sheet(self, sheet)?;
            let mut v2 = schematic.all_components()?;
            v.append(&mut v2)
        }
        Ok(v)
    }

    /// get a component by reference
    pub fn component_by_reference(&self, reference: &str) -> Result<Component, KicadError> {
        for x in &self.elements {
            match *x {
                Element::Component(ref c) => if c.reference == *reference {
                    return Ok(c.clone());
                },
                Element::Wire(_) |
                Element::Connection(_) |
                Element::NoConnect(_) |
                Element::Text(_) |
                Element::Other(_) => (),
            }
        }
        for sheet in &self.sheets {
            let schematic = parse_file_for_sheet(self, sheet)?;
            if let Ok(c) = schematic.component_by_reference(reference) {
                return Ok(c);
            }
        }
        str_error(format!(
            "could not find component {} in schematic",
            reference
        ))
    }

    /// increment the sheet counter of the schematic
    pub fn increment_sheet_count(&mut self) {
        self.description.sheet_count += 1
    }
}

impl fmt::Display for Schematic {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "EESchema Schematic File Version 4")?;
        for v in &self.libraries[..] {
            writeln!(f, "LIBS:{}", v)?
        }
        writeln!(f, "{}", self.eelayer)?;
        writeln!(f, "EELAYER END")?;
        write!(f, "{}", self.description)?;
        for v in &self.elements[..] {
            write!(f, "{}", v)?
        }
        for v in &self.sheets[..] {
            write!(f, "{}", v)?
        }
        writeln!(f, "$EndSCHEMATC")?;
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
// $Descr A4 11693 8268
// encoding utf-8
// Sheet 1 1
// Title "Normal"
// Date "Tue 19 May 2015"
// Rev ""
// Comp ""
// Comment1 ""
// Comment2 ""
// Comment3 ""
// Comment4 ""
// $EndDesc
//
/// description of a schematic
#[derive(Debug, Clone)]
pub struct Description {
    /// size
    pub size: String,
    /// dimension in X
    pub dimx: i64,
    /// dimension in Y
    pub dimy: i64,
    /// title
    pub title: String,
    /// date
    pub date: String,
    /// revision
    pub rev: String,
    /// computer reference
    pub comp: String,
    /// comment1
    pub comment1: String,
    /// comment2
    pub comment2: String,
    /// comment3
    pub comment3: String,
    /// comment4
    pub comment4: String,
    /// sheet number
    pub sheet: i64,
    /// number of sheets in total
    pub sheet_count: i64,
}

impl Default for Description {
    fn default() -> Description {
        Description {
            size: String::from(""),
            dimx: 0,
            dimy: 0,
            title: String::from("Normal"),
            date: String::from("Tue 19 May 2015"),
            rev: String::from(""),
            comp: String::from(""),
            comment1: String::from(""),
            comment2: String::from(""),
            comment3: String::from(""),
            comment4: String::from(""),
            sheet: 1,
            sheet_count: 1,
        }
    }
}

impl fmt::Display for Description {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "$Descr {} {} {}", self.size, self.dimx, self.dimy)?;
        writeln!(f, "encoding utf-8")?;
        writeln!(f, "Sheet {} {}", self.sheet, self.sheet_count)?;
        writeln!(f, "Title \"{}\"", self.title)?;
        writeln!(f, "Date \"{}\"", self.date)?;
        writeln!(f, "Rev \"{}\"", self.rev)?;
        writeln!(f, "Comp \"{}\"", self.comp)?;
        writeln!(f, "Comment1 \"{}\"", self.comment1)?;
        writeln!(f, "Comment2 \"{}\"", self.comment2)?;
        writeln!(f, "Comment3 \"{}\"", self.comment3)?;
        writeln!(f, "Comment4 \"{}\"", self.comment4)?;
        writeln!(f, "$EndDescr")
    }
}

/// a schematic element is either a component or another unparsed element
#[derive(Debug)]
pub enum Element {
    /// a component element
    Component(Component),
    /// Wire
    Wire(Wire),
    /// Connection
    Connection(Connection),
    /// a no-connect
    NoConnect(NoConnect),
    /// a text element (label, hierachical label, global label, ...)
    Text(Text),
    /// an unparsed other element
    Other(String),
}

impl BoundingBox for Element {
    fn bounding_box(&self) -> Bound {
        match *self {
            Element::Component(ref c) => c.bounding_box(),
            Element::Wire(_) |
            Element::Connection(_) |
            Element::NoConnect(_) |
            Element::Text(_) |
            Element::Other(_) => {
                debug!("unhandled schematic element for bounding box");
                Bound::default()
            }
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Element::Component(ref c) => write!(f, "{}", c),
            Element::Wire(ref c) => write!(f, "{}", c),
            Element::Connection(ref c) => write!(f, "{}", c),
            Element::NoConnect(ref c) => write!(f, "{}", c),
            Element::Text(ref c) => write!(f, "{}", c),
            Element::Other(ref c) => write!(f, "{}\n", c),
        }
    }
}

// component fields:
// reference = 0. Value = 1. FootPrint = 2. UserDocLink = 3.
/// a schematic component
#[derive(Debug, Clone)]
pub struct Component {
    /// name
    pub name: String,
    /// reference
    pub reference: String,
    /// u
    pub u: String, // TODO
    /// X coordinate
    pub x: i64, // P
    /// Y coordinate
    pub y: i64, // P
    /// ARPath
    pub ar_path: Vec<String>,
    /// component fields
    pub fields: Vec<ComponentField>,
    /// rotation
    pub rotation: ComponentRotation,
}

impl Default for Component {
    fn default() -> Component {
        Component {
            name: String::from("DUMMY"),
            reference: String::from("U1"),
            u: String::from(""),
            x: 0,
            y: 0,
            ar_path: vec![],
            fields: vec![],
            rotation: ComponentRotation {
                a: 0,
                b: 0,
                c: 0,
                d: 0,
            },
        }
    }
}

impl BoundingBox for Component {
    fn bounding_box(&self) -> Bound {
        debug!("Component bound calculation is poor");
        Bound::new_from_i64(self.y, self.y, self.x, self.y)
    }
}

/// an indication of how a component field was updated
pub enum FieldUpdate {
    /// the field is new
    New,
    /// the field is updated from specified old value to the new one
    Update(String),
    /// the field already existed and is not updated
    Same,
}

impl Into<bool> for FieldUpdate {
    /// was the field updated
    fn into(self) -> bool {
        match self {
            FieldUpdate::New | FieldUpdate::Update(_) => true,
            FieldUpdate::Same => false,
        }
    }
}

impl Component {
    /// set the name
    fn set_name(&mut self, s: String) {
        self.name = s;
    }
    /// set the reference
    fn set_reference(&mut self, s: String) {
        self.reference = s;
    }
    /// add a component field
    fn add_field(&mut self, f: ComponentField) {
        self.fields.push(f)
    }

    /// get a component field value by name
    pub fn get_field_value(&self, name: &str) -> Option<String> {
        for field in &self.fields[..] {
            if field.name == *name {
                return Some(field.value.clone());
            }
        }
        None
    }

    /// get a component field by name
    pub fn get_field(&self, name: &str) -> Option<ComponentField> {
        for field in &self.fields[..] {
            if field.name == *name {
                return Some(field.clone());
            }
        }
        None
    }

    /// get the first free component field number
    pub fn get_available_field_num(&self) -> i64 {
        let mut i: i64 = 0;
        for field in &self.fields[..] {
            if field.i > i {
                i = field.i
            }
        }
        i + 1
    }

    /// get the component fields as a hashmap
    pub fn fields_hash(&self) -> HashMap<String, String> {
        let mut h = HashMap::new();
        for field in &self.fields[..] {
            h.insert(field.name.clone(), field.value.clone());
        }
        h
    }

    /// update the reference of a component
    pub fn update_reference(&mut self, r: String) {
        self.reference = r;
        for field in &mut self.fields[..] {
            if field.i == 0 {
                field.value = self.reference.clone()
            }
        }
    }

    /// update the name of a component
    pub fn update_name(&mut self, n: String) {
        self.name = n;
        for field in &mut self.fields[..] {
            if field.i == 1 {
                field.value = self.name.clone()
            }
        }
    }

    /// update name and value of a component field
    pub fn update_field(&mut self, name: &str, value: &str) {
        for field in &mut self.fields {
            if field.name == *name {
                field.value.clear();
                field.value.push_str(value)
            }
            if field.i > 1 {
                field.visible = false
            }
        }
    }

    /// update or add name and value of a component field
    pub fn add_or_update_field(
        &mut self,
        template: &ComponentField,
        name: &str,
        value: &str,
    ) -> FieldUpdate {
        match self.get_field_value(name) {
            Some(old_value) => if old_value == value {
                FieldUpdate::Same
            } else {
                self.update_field(name, value);
                FieldUpdate::Update(old_value.into())
            },
            None => {
                self.add_new_field(template, name, value);
                FieldUpdate::New
            }
        }
    }

    /// add a new component field based on an existing one but
    /// with a new name and value
    pub fn add_new_field(&mut self, template: &ComponentField, name: &str, value: &str) {
        let i = self.get_available_field_num();
        let mut c = ComponentField::new_from(
            i,
            name.to_string(),
            value.to_string(),
            template.x,
            template.y,
        );
        c.visible = false;
        self.fields.push(c)
    }
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "$Comp")?;
        writeln!(f, "L {} {}", self.name, self.reference)?;
        writeln!(f, "{}", self.u)?;
        writeln!(f, "P {} {}", self.x, self.y)?;
        for x in &self.ar_path {
            writeln!(f, "{}", x)?
        }
        for x in &self.fields[..] {
            writeln!(f, "{}", x)?
        }
        writeln!(
            f,
            "\t{:4} {:4} {:4}",
            "1",
            format!("{}", self.x),
            (format!("{}", self.y))
        )?;
        writeln!(
            f,
            "\t{:4} {:4} {:4} {:4}",
            format!("{}", self.rotation.a),
            format!("{}", self.rotation.b),
            format!("{}", self.rotation.c),
            format!("{}", self.rotation.d)
        )?;
        writeln!(f, "$EndComp")
    }
}

/// a component rotation
#[derive(Debug, Clone)]
pub struct ComponentRotation {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
}

/// a component orientation
#[derive(Debug, Clone)]
pub enum Orientation {
    /// horizontal orientation
    Horizontal,
    /// vertical orientation
    Vertical,
}

impl Orientation {
    /// create a new component orientation from a char
    pub fn new(c: char) -> Result<Orientation, KicadError> {
        match c {
            'H' => Ok(Orientation::Horizontal),
            'V' => Ok(Orientation::Vertical),
            _ => str_error(format!("unknown orientation {}", c)),
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


/// a text justification on a component
#[derive(Debug, Clone)]
pub enum Justify {
    /// left justification
    Left,
    /// right justification
    Right,
    /// center justification
    Center,
    /// bottom justification
    Bottom,
    /// top justification
    Top,
}

impl Justify {
    /// create a justification based on a char
    pub fn new(c: char) -> Result<Justify, KicadError> {
        match c {
            'C' => Ok(Justify::Center),
            'R' => Ok(Justify::Right),
            'L' => Ok(Justify::Left),
            'B' => Ok(Justify::Bottom),
            'T' => Ok(Justify::Top),
            _ => str_error(format!("unexpected justify: {}", c)),
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


/// a component field
#[derive(Debug, Clone)]
pub struct ComponentField {
    /// index of component field
    pub i: i64,
    /// value
    pub value: String,
    /// orientation
    pub orientation: Orientation,
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// size
    pub size: i64,
    /// if it is visible
    pub visible: bool,
    /// horizontal justification
    pub hjustify: Justify,
    /// vertical justification
    pub vjustify: Justify,
    /// if it is italic
    pub italic: bool,
    /// if it is bold
    pub bold: bool,
    /// name of the component field
    pub name: String,
}

impl ComponentField {
    fn new(p: &ParseState, v: &[String]) -> Result<ComponentField, KicadError> {
        if v.len() != 10 && v.len() != 11 {
            return str_error(format!(
                "expecting 10 or 11 parts got {} in {}",
                v.len(),
                p.here()
            ));
        }
        let i = i64_from_string(p, &v[1])?;
        let name = if v.len() == 11 {
            v[10].clone()
        } else {
            match i {
                0 => String::from("Reference"),
                1 => String::from("Value"),
                2 => String::from("Footprint"),
                3 => String::from("UserDocLink"),
                _ => return str_error("expecting name for componentfield > 3".to_string()),
            }
        };
        let c = ComponentField {
            i: i,
            value: v[2].clone(),
            orientation: Orientation::new(char_at(&v[3], 0))?,
            x: f64_from_string(p, &v[4])?,
            y: f64_from_string(p, &v[5])?,
            size: i64_from_string(p, &v[6])?,
            visible: bool_from_string(&v[7], "0000", "0001")?,
            hjustify: Justify::new(char_at(&v[8], 0))?,
            vjustify: Justify::new(char_at(&v[9], 0))?,
            italic: bool_from(&char_at(&v[9], 1), &'I', &'N')?,
            bold: bool_from(&char_at(&v[9], 2), &'B', &'N')?,
            name: name,
        };
        Ok(c)
    }

    /// create a component field
    pub fn new_from(i: i64, name: String, value: String, x: f64, y: f64) -> ComponentField {
        ComponentField {
            i: i,
            name: name,
            value: value,
            orientation: Orientation::Horizontal,
            hjustify: Justify::Center,
            vjustify: Justify::Center,
            italic: false,
            bold: false,
            visible: false,
            size: 60,
            x: x,
            y: y,
        }
    }
}

impl fmt::Display for ComponentField {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "F {} \"{}\" {} ", self.i, self.value, self.orientation)?;
        write!(
            f,
            "{:3} {:3} {:3} ",
            format!("{}", self.x),
            format!("{}", self.y),
            format!("{}", self.size)
        )?;
        write!(f, "{} ", if self.visible { "0000" } else { "0001" })?;
        write!(f, "{} {}", self.hjustify, self.vjustify)?;
        write!(f, "{}", if self.italic { 'I' } else { 'N' })?;
        write!(f, "{}", if self.bold { 'B' } else { 'N' })?;
        if self.i > 3 {
            write!(f, " \"{}\"", self.name)?
        };
        Ok(())
    }
}

/// a component sheet
#[derive(Debug, Clone)]
pub struct Sheet {
    /// X coordinate
    pub x: i64,
    /// Y coordinate
    pub y: i64,
    /// X dimension
    pub dimx: i64,
    /// Y dimension
    pub dimy: i64,
    /// timestamp field
    pub unique: String, // U timestamp field
    /// name of the sheet
    pub name: String, // F0
    /// size of the name
    pub name_size: i64,
    /// filename of the sheet
    pub filename: String, // F1
    /// size of the filename font
    pub filename_size: i64,
    /// sheet labels
    pub labels: Vec<SheetLabel>, // starting at F2
}

impl BoundingBox for Sheet {
    fn bounding_box(&self) -> Bound {
        Bound::new_from_i64(
            self.x,
            self.y - 100,
            self.x + self.dimx,
            self.y + self.dimy + 100,
        )
    }
}

impl Default for Sheet {
    fn default() -> Sheet {
        Sheet {
            x: 0,
            y: 0,
            dimx: 0,
            dimy: 0,
            unique: String::from(""),
            name: String::from("DUMMY"),
            name_size: 60,
            filename: String::from(""),
            filename_size: 60,
            labels: vec![],
        }
    }
}

impl fmt::Display for Sheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "$Sheet")?;
        writeln!(f, "S {} {} {} {}", self.x, self.y, self.dimx, self.dimy)?;
        writeln!(f, "U {}", self.unique)?;
        writeln!(f, "F0 \"{}\" {}", self.name, self.name_size)?;
        writeln!(f, "F1 \"{}\" {}", self.filename, self.filename_size)?;
        let mut i = 2;
        for label in &self.labels[..] {
            writeln!(f, "F{} {}", i, label)?;
            i += 1;
        }
        writeln!(f, "$EndSheet")
    }
}

// F3 "P0.02/AIN0" I L 5250 2450 60
// form = I (input) O (output) B (BiDi) T (tri state) U (unspecified)
// side = R (right) , L (left)., T (tpo) , B (bottom)
/// label on a sheet
#[derive(Debug, Clone)]
pub struct SheetLabel {
    /// name
    pub name: String,
    /// shape of the label
    pub form: LabelForm,
    /// side of the label
    pub side: LabelSide,
    /// X coordinate
    pub x: i64,
    /// Y coordinate
    pub y: i64,
    /// size
    pub size: i64,
}

impl Default for SheetLabel {
    fn default() -> SheetLabel {
        SheetLabel {
            name: String::from(""),
            form: LabelForm::Input,
            side: LabelSide::Left,
            x: 0,
            y: 0,
            size: 60,
        }
    }
}

impl fmt::Display for SheetLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "\"{}\" {} {} {} {} {}",
            self.name,
            self.form,
            self.side,
            self.x,
            self.y,
            self.size
        )
    }
}

/// form of a label
#[derive(Debug, Clone)]
pub enum LabelForm {
    /// input
    Input,
    /// output
    Output,
    /// bidirectional
    BiDi,
    /// tristate
    TriState,
    /// unspecified
    Unspecified,
}

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

/// a side of a label
#[derive(Debug, Clone)]
pub enum LabelSide {
    /// left
    Left,
    /// right
    Right,
    /// top
    Top,
    /// bottom
    Bottom,
}

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

/// the type of wire
#[derive(Debug)]
pub enum WireType {
    /// an electronics wire
    Wire,
    /// a dashed notes wire
    Notes,
    /// a bus
    Bus,
}

impl fmt::Display for WireType {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let c = match *self {
            WireType::Wire => "Wire",
            WireType::Notes => "Notes",
            WireType::Bus => "Bus",
        };
        write!(f, "{}", c)
    }
}

/// a wire part making a connection
#[derive(Debug)]
pub struct Wire {
    /// the type of the wire
    pub type_: WireType,
    /// x-coordinate of first point of wire
    pub x1: i64,
    /// y-coordinate of first point of wire
    pub y1: i64,
    /// x-coordinate of second point of wire
    pub x2: i64,
    /// y-coordinate of second point of wire
    pub y2: i64,
}

impl fmt::Display for Wire {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "Wire {} Line", self.type_)?;
        writeln!(f, "\t{} {} {} {}", self.x1, self.y1, self.x2, self.y2)
    }
}

/// a connection part making a junction
#[derive(Debug)]
pub struct Connection {
    /// connection x-coordinate
    pub x: i64,
    /// connection y-coordinate
    pub y: i64,
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "Connection ~ {} {}", self.x, self.y)
    }
}

/// a no-connect marker
#[derive(Debug)]
pub struct NoConnect {
    /// no-connect x-coordinate
    pub x: i64,
    /// no-connect y-coordinate
    pub y: i64,
}

impl fmt::Display for NoConnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "NoConn ~ {} {}", self.x, self.y)
    }
}

/// type of a text element
#[derive(Debug)]
pub enum TextType {
    /// a text note
    Note,
    /// a global label
    Global,
    /// a hierarchical label
    Hierarchical,
    /// a local label
    Label,
}

impl fmt::Display for TextType {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let x = match *self {
            TextType::Note => "Notes",
            TextType::Global => "GLabel",
            TextType::Hierarchical => "HLabel",
            TextType::Label => "Label",
        };
        write!(f, "{}", x)
    }
}

impl TextType {
    fn is_local(&self) -> bool {
        match *self {
            TextType::Note | TextType::Label => true,
            TextType::Global | TextType::Hierarchical => false,
        }
    }
}

//Text Label 9300 2175 0    60   Italic 12
//IAMBOLDITALIC
//Text Notes 8025 5400 0    60   ~ 0
//IAMANOTE

/// a text
#[derive(Debug)]
pub struct Text {
    /// type of  the text
    pub t: TextType,
    /// x-coordinate of the text
    pub x: i64,
    /// y-coordinate of the text
    pub y: i64,
    /// orientation of the text
    pub orientation: i64, // TODO: implement more specified
    /// size of the text
    pub size: i64,
    /// shape of the text
    pub shape: Option<String>, // TODO: implement more specified
    /// if it is italic
    pub italic: bool,
    /// thickness (for bold)
    pub thickness: i64,
    /// the contained text
    pub text: String,
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let italic = if self.italic { "Italic" } else { "~" };
        write!(f, "Text {} ", self.t)?;
        // trick to get left-aligned adjusted numbers: convert to string first...
        let x = format!("{}", self.x);
        let y = format!("{}", self.y);
        write!(f, "{:4} {:4} ", x, y)?;
        let orientation = format!("{}", self.orientation);
        let size = format!("{}", self.size);
        write!(f, "{:4} {:4} ", orientation, size)?;
        if let Some(ref shape) = self.shape {
            write!(f, "{} ", shape)?;
        }
        writeln!(f, "{} {}", italic, self.thickness)?;
        // kicad has code here to escape \ns and sanitize unix/win/mac line endings; this is not needed as we assume the file was saved with kicad to begin with
        writeln!(f, "{}", self.text)
    }
}

fn char_at(s: &str, p: usize) -> char {
    let v: Vec<char> = s.chars().collect();
    v[..][p]
}


fn assume_string(e: &'static str, s: &str) -> Result<(), KicadError> {
    if *e != *s {
        return str_error(format!("expecting: {}, actually: {}", e, s));
    }
    Ok(())
}

fn i64_from_string(p: &ParseState, s: &str) -> Result<i64, KicadError> {
    match i64::from_str(&s[..]) {
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

fn bool_from<T: PartialEq + fmt::Display>(i: &T, t: &T, f: &T) -> Result<bool, KicadError> {
    if i == t {
        return Ok(true);
    }
    if i == f {
        return Ok(false);
    }
    str_error(format!("unknown boolean {}, expected {} or {}", i, t, f))
}

fn word_and_qstring<F>(d: &mut Description, name: &'static str, s: &str, setter: F) -> Result<(), KicadError>
where
    F: Fn(&mut Description, String) -> (),
{
    let v = parse_split_quote_aware_n(2, s)?;
    assume_string(name, &v[0])?;
    setter(d, v[1].clone());
    Ok(())
}


fn parse_description(p: &mut ParseState) -> Result<Description, KicadError> {
    let mut d = Description::default();
    let v = parse_split_quote_aware_n(4, &p.here())?;
    d.size = v[1].clone();
    d.dimx = i64_from_string(p, &v[2])?;
    d.dimy = i64_from_string(p, &v[3])?;
    p.next(); // $Descr
    p.next(); // encoding
    let v = parse_split_quote_aware_n(3, &p.here())?;
    if v[0] != "Sheet" {
        return str_error(String::from("Expecting 'Sheet'"));
    };
    d.sheet = i64_from_string(p, &v[1])?;
    d.sheet_count = i64_from_string(p, &v[2])?;
    p.next(); // Sheet
    word_and_qstring(&mut d, "Title", &p.here(), |d, x| d.title = x)?;
    p.next();
    word_and_qstring(&mut d, "Date", &p.here(), |d, x| d.date = x)?;
    p.next();
    word_and_qstring(&mut d, "Rev", &p.here(), |d, x| d.rev = x)?;
    p.next();
    word_and_qstring(&mut d, "Comp", &p.here(), |d, x| d.comp = x)?;
    p.next();
    word_and_qstring(&mut d, "Comment1", &p.here(), |d, x| d.comment1 = x)?;
    p.next();
    word_and_qstring(&mut d, "Comment2", &p.here(), |d, x| d.comment2 = x)?;
    p.next();
    word_and_qstring(&mut d, "Comment3", &p.here(), |d, x| d.comment3 = x)?;
    p.next();
    word_and_qstring(&mut d, "Comment4", &p.here(), |d, x| d.comment4 = x)?;
    p.next();
    assume_string("$EndDescr", &p.here())?;
    Ok(d)
}

fn parse_component_l(p: &mut ParseState, d: &mut Component) -> Result<(), KicadError> {
    let v = parse_split_quote_aware_n(3, &p.here())?;
    d.set_name(v[1].clone());
    d.set_reference(v[2].clone());
    Ok(())
}

fn parse_component_u(p: &mut ParseState, d: &mut Component) -> Result<(), KicadError> {
    d.u = p.here();
    Ok(())
}

fn parse_component_ar(p: &mut ParseState, d: &mut Component) -> Result<(), KicadError> {
    d.ar_path.push(p.here());
    Ok(())
}

fn parse_component_p(p: &mut ParseState, d: &mut Component) -> Result<(), KicadError> {
    let v = parse_split_quote_aware_n(3, &p.here())?;
    d.x = i64_from_string(p, &v[1])?;
    d.y = i64_from_string(p, &v[2])?;
    Ok(())
}

fn parse_component_f(p: &mut ParseState, d: &mut Component) -> Result<(), KicadError> {
    // println!("{}", p.here());
    let v = parse_split_quote_aware(&p.here())?;
    // for i in &v[..] {
    //    println!("'{}'", i)
    // }
    let f = ComponentField::new(p, &v)?;
    d.add_field(f);
    Ok(())
}

fn parse_component_rotation(p: &mut ParseState, d: &mut Component) -> Result<(), KicadError> {
    let s = p.here();
    let v: Vec<&str> = s.split_whitespace().collect();
    if v.len() != 4 {
        p.next(); // skip unnecessary position line
    }

    let s = p.here();
    let v: Vec<&str> = s.split_whitespace().collect();
    if v.len() != 4 {
        return str_error(format!("expecting 4 elements in {}", s));
    }
    let a1 = i64_from_string(p, &String::from(v[0]))?;
    let b1 = i64_from_string(p, &String::from(v[1]))?;
    let c1 = i64_from_string(p, &String::from(v[2]))?;
    let d1 = i64_from_string(p, &String::from(v[3]))?;
    let rot = ComponentRotation {
        a: a1,
        b: b1,
        c: c1,
        d: d1,
    };
    d.rotation = rot;
    Ok(())
}

fn parse_component(p: &mut ParseState) -> Result<Component, KicadError> {
    let mut d = Component::default();
    p.next();
    loop {
        let s = p.here();
        if s == "$EndComp" {
            break;
        }
        match s.split_whitespace().next() {
            Some("L") => parse_component_l(p, &mut d)?,
            Some("U") => parse_component_u(p, &mut d)?,
            Some("P") => parse_component_p(p, &mut d)?,
            Some("F") => parse_component_f(p, &mut d)?,
            Some("1") | Some("-1") | Some("0") => parse_component_rotation(p, &mut d)?,
            Some("AR") => parse_component_ar(p, &mut d)?,
            Some("2") | Some("3") | Some("4") | Some("5") |
            Some("6") | Some("7") | Some("8") | Some("9") => (),
            _ => println!("skipping unknown component line {}", s),
        }
        p.next()
    }
    Ok(d)
}

// S 5250 2300 950  3100
fn parse_sheet_s(p: &mut ParseState, s: &mut Sheet) -> Result<(), KicadError> {
    let v = parse_split_quote_aware_n(5, &p.here())?;
    s.x = i64_from_string(p, &v[1])?;
    s.y = i64_from_string(p, &v[2])?;
    s.dimx = i64_from_string(p, &v[3])?;
    s.dimy = i64_from_string(p, &v[4])?;
    Ok(())
}

// U 5655A9F3
fn parse_sheet_u(p: &mut ParseState, s: &mut Sheet) -> Result<(), KicadError> {
    let v = parse_split_quote_aware_n(2, &p.here())?;
    s.unique = v[1].clone();
    Ok(())
}

fn parse_label_form(s: &str) -> Result<LabelForm, KicadError> {
    match &s[..] {
        "I" => Ok(LabelForm::Input),
        "O" => Ok(LabelForm::Output),
        "B" => Ok(LabelForm::BiDi),
        "T" => Ok(LabelForm::TriState),
        "U" => Ok(LabelForm::Unspecified),
        _ => str_error(format!("unknown labelform {}", s)),
    }
}

fn parse_label_side(s: &str) -> Result<LabelSide, KicadError> {
    match &s[..] {
        "L" => Ok(LabelSide::Left),
        "R" => Ok(LabelSide::Right),
        "T" => Ok(LabelSide::Top),
        "B" => Ok(LabelSide::Bottom),
        _ => str_error(format!("unknown labelside {}", s)),
    }
}


// F3 "P0.02/AIN0" I L 5250 2450 60
fn parse_sheet_label(p: &ParseState, s: &str) -> Result<SheetLabel, KicadError> {
    let mut l = SheetLabel::default();
    let v = parse_split_quote_aware_n(7, s)?;
    l.name = v[1].clone();
    l.form = parse_label_form(&v[2])?;
    l.side = parse_label_side(&v[3])?;
    l.x = i64_from_string(p, &v[4])?;
    l.y = i64_from_string(p, &v[5])?;
    l.size = i64_from_string(p, &v[6])?;
    Ok(l)
}

fn parse_sheet_f(p: &mut ParseState, s: &mut Sheet, f: &str) -> Result<(), KicadError> {
    // s.u = p.here();
    let mut f = String::from(f);
    f.remove(0);
    let i = i64_from_string(p, &f)?;
    if i == 0 {
        let name_size = parse_split_quote_aware_n(3, &p.here())?;
        s.name = name_size[1].clone();
        s.name_size = i64_from_string(p, &name_size[2])?;
    } else if i == 1 {
        let filename_size = parse_split_quote_aware_n(3, &p.here())?;
        s.filename = filename_size[1].clone();
        s.filename_size = i64_from_string(p, &filename_size[2])?;
    } else {
        let label_el = parse_sheet_label(p, &p.here())?;
        s.labels.push(label_el)
    }
    Ok(())
}


fn parse_sheet(p: &mut ParseState) -> Result<Sheet, KicadError> {
    let mut s = Sheet::default();
    p.next();
    loop {
        let st = p.here();
        if st == "$EndSheet" {
            break;
        }
        match st.split_whitespace().next() {
            Some("S") => parse_sheet_s(p, &mut s)?,
            Some("U") => parse_sheet_u(p, &mut s)?,
            Some(x) => if x.starts_with('F') {
                parse_sheet_f(p, &mut s, x)?
            } else {
                println!("skipping unknown sheet line {}", st)
            },
            _ => println!("skipping unknown sheet line {}", st),
        }
        p.next();
    }
    Ok(s)
}

// Wire Wire Line
//	6100 3050 5850 3050
fn parse_wire(p: &mut ParseState) -> Result<Wire, KicadError> {
    let t = p.here();
    let v: Vec<&str> = t.split_whitespace().collect();
    if v.len() != 3 {
        return str_error(format!("expecting 3 elements in {}", t));
    }
    let t = match v[1] {
        "Notes" => WireType::Notes,
        "Bus" => WireType::Bus,
        _ => WireType::Wire,
    };
    p.next();
    let s = p.here();
    let v: Vec<&str> = s.split_whitespace().collect();
    if v.len() != 4 {
        return str_error(format!("expecting 4 elements in {}", s));
    }
    let x1 = i64_from_string(p, &String::from(v[0]))?;
    let y1 = i64_from_string(p, &String::from(v[1]))?;
    let x2 = i64_from_string(p, &String::from(v[2]))?;
    let y2 = i64_from_string(p, &String::from(v[3]))?;
    Ok(Wire {
        type_: t,
        x1: x1,
        y1: y1,
        x2: x2,
        y2: y2,
    })
}

// Connection ~ 5250 3050
fn parse_connection(p: &mut ParseState) -> Result<Connection, KicadError> {
    let s = p.here();
    let v: Vec<&str> = s.split_whitespace().collect();
    if v.len() != 4 {
        return str_error(format!("expecting 4 elements in {}", s));
    }
    let x1 = i64_from_string(p, &String::from(v[2]))?;
    let y1 = i64_from_string(p, &String::from(v[3]))?;
    Ok(Connection { x: x1, y: y1 })
}

// NoConnect ~ 5250 3050
fn parse_no_connect(p: &mut ParseState) -> Result<NoConnect, KicadError> {
    let s = p.here();
    let v: Vec<&str> = s.split_whitespace().collect();
    if v.len() != 4 {
        return str_error(format!("expecting 4 elements in {}", s));
    }
    let x1 = i64_from_string(p, &String::from(v[2]))?;
    let y1 = i64_from_string(p, &String::from(v[3]))?;
    Ok(NoConnect { x: x1, y: y1 })
}

//Text Label 9300 2175 0    60   Italic 12
//IAMBOLDITALIC
//Text Notes 8025 5400 0    60   ~ 0
//IAMANOTE
fn parse_text(p: &mut ParseState) -> Result<Text, KicadError> {
    let s = p.here();
    let v: Vec<&str> = s.split_whitespace().collect();
    if v.len() < 8 {
        return str_error(format!("expecting 8 or 9 elements in {}", s));
    }
    let t = match v[1] {
        "Label" => TextType::Label,
        "Notes" => TextType::Note,
        "GLabel" => TextType::Global,
        "HLabel" => TextType::Hierarchical,
        x => return Err(format!("Unknown text type {} at {}", x, s).into()),
    };
    let x1 = i64_from_string(p, &String::from(v[2]))?;
    let y1 = i64_from_string(p, &String::from(v[3]))?;
    let orientation = i64_from_string(p, &String::from(v[4]))?;
    let size = i64_from_string(p, &String::from(v[5]))?;
    let mut index = 6;
    let shape = if !t.is_local() {
        index = 7;
        Some(v[6].into())
    } else {
        None
    };
    let italic = match v[index] {
        "Italic" => true,
        _ => false,
    };
    let thickness = i64_from_string(p, &String::from(v[index + 1]))?;
    p.next();
    let text = p.here();
    Ok(Text {
        t: t,
        x: x1,
        y: y1,
        orientation: orientation,
        size: size,
        shape: shape,
        italic: italic,
        thickness: thickness,
        text: text,
    })
}

/// parse a &str to a Kicad schematic, optionally setting the filename
pub fn parse(filename: Option<PathBuf>, s: &str) -> Result<Schematic, KicadError> {
    let mut sch = Schematic::default();
    sch.filename = filename;
    let v: Vec<&str> = s.lines().collect();
    let p = &mut ParseState::new(v);
    assume_line!(p, "EESchema Schematic File Version 4");
    while !p.eof() {
        {
            let s = p.here();
            if !s.starts_with("LIBS:") {
                break;
            }
            sch.add_library(String::from(&s[5..]));
        }
        p.next();
    }
    if !p.here().starts_with("EELAYER ") {
        return str_error(format!("expecting EELAYER, got {}", p.here()));
    }
    sch.eelayer = p.here();
    p.next();
    assume_line!(p, "EELAYER END");
    while !p.eof() {
        {
            match p.here().split_whitespace().next() {
                Some("$Descr") => {
                    let d = parse_description(p)?;
                    sch.set_description(d)
                }
                Some("$Comp") => {
                    let d = parse_component(p)?;
                    sch.append_component(d)
                }
                Some("$Sheet") => {
                    let d = parse_sheet(p)?;
                    sch.append_sheet(d)
                }
                Some("$EndSCHEMATC") => (),
                Some("Wire") => {
                    let w = parse_wire(p)?;
                    sch.append_wire(w)
                }
                Some("Connection") => {
                    let w = parse_connection(p)?;
                    sch.append_connection(w)
                }
                Some("NoConn") => {
                    let w = parse_no_connect(p)?;
                    sch.append_no_connect(w)
                }
                Some("Text") => {
                    let w = parse_text(p)?;
                    sch.append_text(w)
                }
                Some(_) => sch.append_other(p.here()),
                None => unreachable!(),
            }
        }
        p.next()
    }
    Ok(sch)
}


/// parse a &str as a Kicad schematic
pub fn parse_str(s: &str) -> Result<Schematic, KicadError> {
    parse(None, s)
}

/// parse a file as a Kicad schematic
pub fn parse_file(filename: &Path) -> Result<Schematic, KicadError> {
    let s = read_file(filename)?;
    parse(Some(PathBuf::from(filename)), &s[..])
}

/// get the filename for a sheet in a schematic
pub fn filename_for_sheet(schematic: &Schematic, sheet: &Sheet) -> Result<PathBuf, KicadError> {
    let path = match schematic.filename {
        Some(ref path) => Ok(path),
        None => Err("can't load sheet when there is no filename for the schematic".to_string()),
    }?;
    let dir = match path.parent() {
        Some(dir) => Ok(dir),
        None => Err("can't load sheet when I don't know the dir of the schematic".to_string()),
    }?;
    Ok(dir.join(&sheet.filename))
}


/// parse a file as a Kicad schematic for a sheet
pub fn parse_file_for_sheet(schematic: &Schematic, sheet: &Sheet) -> Result<Schematic, KicadError> {
    let f = filename_for_sheet(schematic, sheet)?;
    parse_file(&f)
}
