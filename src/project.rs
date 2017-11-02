// (c) 2016 Productize SPRL <joost@productize.be>

// extension: .pro
// format: special project format

// first line starts with:
// update=

// get from parent
use {str_error, KicadError};

/// a Kicad project
#[derive(Debug)]
pub struct Project {
    /// project file content as an unparsed String
    pub data: String,
}

/// parse a &str to a project
pub fn parse_str(s: &str) -> Result<Project, KicadError> {
    if !s.starts_with("update=") {
        str_error("not a kicad project file!".to_string())
    } else {
        Ok(Project {
            data: String::from(s),
        })
    }
}
