// (c) 2016 Productize SPRL <joost@productize.be>

//use std::fmt;
//use std::str::FromStr;

// get from parent
use ERes;

// first line starts with:
// update=

#[derive(Debug)]
pub struct Project {
    pub data:String,
}

pub fn parse_str(s: &str) -> ERes<Project> {
    if !s.starts_with("update=") {
        Err(format!("not a kicad project file!"))
    } else {
        Ok(Project { data:String::from(s) })
    }
    
}
