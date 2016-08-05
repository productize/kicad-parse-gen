// (c) 2016 Productize SPRL <joost@productize.be>

use std::fs::File;
use std::io::Read;
use std::io::Write;

use Result;

/// read a file
pub fn read_file(name: &str) -> Result<String> {
    let mut f = try!(File::open(name));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

/// write a file
pub fn write_file(name: &str, data: &str) -> Result<()> {
    let mut f = try!(match File::create(name) {
        Ok(f) => Ok(f),
        Err(err) => Err(format!("create error in file '{}': {}", name, err)),
    });
    try!(match write!(&mut f, "{}", data) {
        Ok(f) => Ok(f),
        Err(err) => Err(format!("write error in file '{}': {}", name, err)),
    });

    Ok(())

}
