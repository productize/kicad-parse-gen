// (c) 2016 Productize SPRL <joost@productize.be>


/* 

1.7 valid characters in string

Filenames, symbol names, footprint names and model names must contain only valid characters, as below:
* Alphanumeric characters (A-Z, a-z, 0-9)
* Underscore _
* Hyphen / dash -
* Period / dot .

*/

use symbol_lib::*;

const ALLOWED_1_7:&'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_-.";



pub fn allowed_1_7(s:&str) -> Option<Vec<String>> {
    let mut v = vec![];
    for (i,c) in s.chars().enumerate() {
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

pub fn is_allowed_1_7(s:&str) -> bool {
    allowed_1_7(s).is_none()
}

/*

4.1 Pin placement
* Using a 100mil grid, pin origin must lie on grid nodes (IEC-60617)
* Pins should have a length of at least 100mils (2.54mm)
* Pin length should not be more than 300mils (7.62mm)
* Pin length can be incremented in steps of 50mils (1.27mm) if required e.g. for long pin numbers
* Shorter pins may be allowed for simple symbols such as resistors, capacitors, diodes, etc

*/

fn field_4_1(field:&Field) -> bool {
    let mut res = true;
    if ((field.x as i64) % 10) != 0 {
        warn!("field x not on 100mil grid");
        res = false;
    }
    if ((field.y as i64) % 10) != 0 {
        warn!("field y not on 100mil grid");
        res = false;
    }
    res
}

fn draw_4_1(draw:&Draw) -> bool {
    let mut res = true;
    if let Draw::Pin(ref pin) = *draw {
        let name = format!("{}:{}", pin.name, pin.number);
        if (pin.x % 10) != 0 {
            warn!("{}: pin x not on 100mil grid", name);
            res = false;
        }
        if (pin.y % 10) != 0 {
            warn!("{}: pin y not on 100mil grid", name);
            res = false;
        }
        if (pin.len % 5) != 0 {
            warn!("{}: pin length not on a 50mil multiple", name);
            res = false;
        }
        if pin.len < 10 {
            info!("{}: pin should probably be >= 100mil", name);
        }
        if pin.len > 30 {
            info!("{}: pin should probably be <= 300mil", name);
        }
    }
    res
}

pub fn symbol_4_1(symbol:&Symbol) -> bool {
    let mut res = true;
    if !is_allowed_1_7(&symbol.name) {
        warn!("Symbol name contains not allowed characters");
        res = false;
    }
    for field in &symbol.fields {
        res |= field_4_1(field)
    }
    for draw in &symbol.draw {
        res |= draw_4_1(draw)
    }
    res
}

/*

4.2 Symbol visual style

* Origin is placed in the middle of symbol
* Symbol body has a line width of 10mils (0.254mm)
* Fill style of symbol body is set to Fill background
* IEC-style symbols are used whenever possible

*/
pub fn symbol_4_2(symbol:&Symbol) -> bool {
    let mut res = true;
    for draw in &symbol.draw {
        res |= draw_4_2(draw)
    }
    res
}

fn draw_4_2(draw:&Draw) -> bool {
    if let Draw::Rectangle(ref rec) = *draw {
        if rec.fill != Fill::Filled {
            info!("Non-filled rect in drawing")
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_allowed_1_7_1() {
        assert!(allowed_1_7("Hello_world_1.23-4").is_none())
    }
    
    #[test]
    fn test_allowed_1_7_2() {
        let t = allowed_1_7("Hello world").unwrap().into_iter().next().unwrap();
        assert_eq!(t, "Invalid char ' ' at 5 in 'Hello world'")
    }
}
