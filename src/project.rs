// (c) 2016 Productize SPRL <joost@productize.be>

//use std::fmt;
//use std::str::FromStr;

// get from parent
use Result;
use str_error;

// first line starts with:
// update=

#[derive(Debug)]
pub struct Project {
    pub data:String,
}

pub fn parse_str(s: &str) -> Result<Project> {
    if !s.starts_with("update=") {
        str_error("not a kicad project file!".to_string())
    } else {
        Ok(Project { data:String::from(s) })
    }
    
}
