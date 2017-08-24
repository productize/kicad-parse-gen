// (c) 2017 Productize SPRL <joost@productize.be>

use std::fmt;
use std::result;

/// symbol settings for checking and fixing
pub struct SymConfig {
}

/// module settings for checking and fixing
pub struct ModConfig {
    /// font size
    pub font_size: f64,
    /// font thickness
    pub font_thickness: f64,
}

/// config settings for checking and fixing
pub struct Config {
    /// name of the Config
    pub name: String,
    /// symbol configuration
    pub s:SymConfig,
    /// module configuration
    pub m: ModConfig,
}

impl Config {
    pub fn klc() -> Config {
        Config {
            name:"KLC 2.0.10".into(),
            s:SymConfig {
            },
            m: ModConfig {
                font_size: 1.0,
                font_thickness: 0.15,
            },
        }
    }
}

/// Check & Fix trait to be implemented for KLC checking and fixing
pub trait CheckFix {
    /// check an item against the KLC
    fn check(&self, config: &Config) -> Vec<CheckFixData>;

    /// fix up an item against the KLC
    fn fix(&mut self, _config: &Config) {}
}

#[derive(Debug)]
/// a KLC check result data
pub enum CheckFixData {
    /// a KLC check result item
    Item(CheckFixItem),
    /// a list of more KLC check result datas
    More(Vec<CheckFixData>),
}

impl CheckFixData {
    /// if a `CheckFixData` is of type `More` but contains only one item, change it into an `Item`
    pub fn flatter(self) -> Self {
        match self {
            CheckFixData::Item(_) => self,
            CheckFixData::More(v) => if v.len() == 1 {
                v.into_iter().next().unwrap()
            } else {
                CheckFixData::More(v)
            },
        }
    }

    /// create a new `CheckFixData`
    pub fn new<A: fmt::Display, B: Into<String>>(
        section: i64,
        rule: i64,
        item: A,
        message: B,
    ) -> Self {
        let i = CheckFixItem::new(section, rule, item, message.into());
        CheckFixData::Item(i)
    }

    /// create a new informational `CheckFixData`
    pub fn info<A: fmt::Display, B: Into<String>>(
        section: i64,
        rule: i64,
        item: A,
        message: B,
    ) -> Self {
        let i = CheckFixItem::new(section, rule, item, message.into()).info();
        CheckFixData::Item(i)
    }

    /// dump a `CheckFixData` on the `Log` logger
    pub fn dump_on_logger(&self, indent: usize) {
        match *self {
            CheckFixData::Item(ref item) => item.dump_on_logger(indent),
            CheckFixData::More(ref more) => for klcdata in more {
                let new_indent = match *klcdata {
                    CheckFixData::Item(_) => indent,
                    CheckFixData::More(_) => indent + 1,
                };
                klcdata.dump_on_logger(new_indent)
            },
        }
    }
}

#[derive(Debug)]
/// a KLC check result item
pub struct CheckFixItem {
    /// KLC section
    pub section: i64,
    /// KLC rule in the section
    pub rule: i64,
    /// item that this is about
    pub item: String,
    /// message about the problem
    pub message: String,
    /// if the item is informational only
    pub info: bool,
}
impl CheckFixItem {
    /// create a new `CheckFixItem`
    pub fn new<A: fmt::Display, B: Into<String>>(
        section: i64,
        rule: i64,
        item: A,
        message: B,
    ) -> Self {
        CheckFixItem {
            section: section,
            rule: rule,
            item: format!("{}", item),
            message: message.into(),
            info: false,
        }
    }

    /// create a new informational `CheckFixItem`
    pub fn info(self) -> CheckFixItem {
        CheckFixItem { info: true, ..self }
    }

    /// dump a `CheckFixItem` on a `Log` logger
    pub fn dump_on_logger(&self, indent: usize) {
        let indent = ::std::iter::repeat(" ")
            .take(indent * 2)
            .collect::<String>();
        if self.info {
            info!(
                "{}{}.{} {}:{}",
                indent,
                self.section,
                self.rule,
                self.item,
                self.message
            )
        } else {
            warn!(
                "{}{}.{} {}:{}",
                indent,
                self.section,
                self.rule,
                self.item,
                self.message
            )
        }
    }
}

#[derive(Debug)]
/// KLC Section
pub enum KLCSection {
    /// General
    General,
    /// Symbol Library Names
    SymbolLibraryNames,
    /// Symbol Names
    SymbolNames,
    /// Symbol Rules
    SymbolRules,
    /// Footprint Library Names
    FootprintLibraryNames,
    /// Footprint Names
    FootprintNames,
    /// Footprint Rules
    FootprintRules,
    /// SMD Rules
    SMDRules,
    /// THT Rules
    THTRules,
    /// Footprint Properties
    FootprintProperties,
}

impl Into<i64> for KLCSection {
    fn into(self) -> i64 {
        match self {
            KLCSection::General => 1,
            KLCSection::SymbolLibraryNames => 2,
            KLCSection::SymbolNames => 3,
            KLCSection::SymbolRules => 4,
            KLCSection::FootprintLibraryNames => 5,
            KLCSection::FootprintNames => 6,
            KLCSection::FootprintRules => 7,
            KLCSection::SMDRules => 8,
            KLCSection::THTRules => 9,
            KLCSection::FootprintProperties => 10,
        }
    }
}

impl fmt::Display for KLCSection {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let s = match *self {
            KLCSection::General => "General Rules",
            KLCSection::SymbolLibraryNames => "Symbol Library Names",
            KLCSection::SymbolNames => "Symbol Names",
            KLCSection::SymbolRules => "General Rules for Symbols",
            KLCSection::FootprintLibraryNames => "Footprint Library Names",
            KLCSection::FootprintNames => "Footprint Names",
            KLCSection::FootprintRules => "General Rules for Footprints",
            KLCSection::SMDRules => "Rules for SMD Footprints",
            KLCSection::THTRules => "Rules for Through-hole Footprints",
            KLCSection::FootprintProperties => "Footprint Properties",
        };
        write!(f, "{}", s)
    }
}

/* 

1.7 valid characters in string

Filenames, symbol names, footprint names and model names must contain only valid characters, as below:
* Alphanumeric characters (A-Z, a-z, 0-9)
* Underscore _
* Hyphen / dash -
* Period / dot .

*/


const ALLOWED_1_7: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_-.";

fn allowed_1_7(s: &str) -> Option<Vec<String>> {
    let mut v = vec![];
    for (i, c) in s.chars().enumerate() {
        if ALLOWED_1_7.chars().find(|&x| x == c).is_none() {
            v.push(format!("Invalid char '{}' at {} in '{}'", c, i, s))
        }
    }
    if v.is_empty() {
        None
    } else {
        Some(v)
    }
}

/// check if a name is allowed according to KLC 1.7
pub fn allowed_1_7_items(s: &str) -> Vec<CheckFixData> {
    let mut v = vec![];
    if let Some(v2) = allowed_1_7(s) {
        for x in v2 {
            v.push(CheckFixData::new(1, 7, s, x))
        }
    }
    v
}

/// check if a name is allowed according to KLC 1.7
pub fn is_allowed_1_7(s: &str) -> bool {
    allowed_1_7(s).is_none()
}

/*

4.3 Pin stacking. Placing pins in the same position results in the circuits being connected. Pins may be placed in the same location under certain circumstances:
* Pins must not be of type No Connect
* Pins are logically connected in the symbol
* Pins must have the same name
* Pins must have the same electrical type
* Only one pin must be visible (all others set to invisible)
* Stacks of type Output, Power Output and Power Input are special cases. One visible pin must have the correct type, and all other pins in the stack must be passive and invisible.

*/

/* 

4.4 Pins should be grouped logically, rather than physically
* Pin location should not necessarily follow footprint pinout
* Pins with similar functions should be placed together, e.g. SPI_MISO, SPI_MOSI, SPI_SCK, SPI_CS and UART_TX, UART_RX
* Ports should be ordered from top to bottom, unless this conflicts with the above requirements

 */

/* 

4.5 Whenever possible, pins should be arranged by function:
* Positive Power pins should be placed at top of the symbol, e.g. Vcc, Vdd, Vin, V+, etc
* Negative Power and Ground pins should be placed at the bottom of the symbol, e.g. GND, Vss, V-, etc
* Input/Control/Logic pins should be placed on the left of the symbol, e.g. opamp +/-, NPN base, SPI pins on an DAC, transformer primary, UART Tx/Rx pins, etc.
* Output/Controlled/Driver pins should be placed on the right of the symbol, e.g. opamp output, DAC output, transformer secondary, RS232 Tx/Rx, etc. 

 */

/*

4.6 Pin Electrical type should be set to match the appropriate pin function
* Power and Ground pins should be set to either POWER INPUT or POWER OUTPUT
* Other pin types should be set as appropriate 

 */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_allowed_1_7_1() {
        assert!(allowed_1_7("Hello_world_1.23-4").is_none())
    }

    #[test]
    fn test_allowed_1_7_2() {
        let t = allowed_1_7("Hello world")
            .unwrap()
            .into_iter()
            .next()
            .unwrap();
        assert_eq!(t, "Invalid char ' ' at 5 in 'Hello world'")
    }
}
