// (c) 2016 Productize SPRL <joost@productize.be>

// extension: .pro
// format: special project format

// first line starts with:
// update=

// get from parent
use Result;
use str_error;

#[derive(Debug)]
pub struct Project {
    pub data: String,
}

pub fn parse_str(s: &str) -> Result<Project> {
    if !s.starts_with("update=") {
        str_error("not a kicad project file!".to_string())
    } else {
        Ok(Project { data: String::from(s) })
    }

}
