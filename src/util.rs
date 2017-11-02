// (c) 2016 Productize SPRL <joost@productize.be>

use std::fs::File;
use std::io::Read;
use std::io;
use std::io::Write;
use std::path::Path;
use KicadError;

/// read a file
pub fn read_file<P>(path: P) -> Result<String, io::Error>
where
    P: AsRef<Path>,
{
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

/// write a file
pub fn write_file<P>(name: P, data: &str) -> Result<(), KicadError>
where
    P: AsRef<Path>,
{
    let mut f = match File::create(&name) {
        Ok(f) => Ok(f),
        Err(err) => Err(format!(
            "create error in file '{}': {}",
            name.as_ref().display(),
            err
        )),
    }?;
    match write!(&mut f, "{}", data) {
        Ok(f) => Ok(f),
        Err(err) => Err(format!(
            "write error in file '{}': {}",
            name.as_ref().display(),
            err
        )),
    }?;

    Ok(())
}
