// (c) 2015-2016 Joost Yervante Damad <joost@productize.be>

// extension: .kicad_mod
// format: new-style

use std::fmt;
use std::result;

// get from parent
use Result;
use str_error;
use symbolic_expressions;
use symbolic_expressions::IntoSexp;
use formatter::KicadFormatter;
use FromSexp;

pub use footprint::data::*;

/// convert a Kicad Module (footprint) to a String
pub fn module_to_string(module: &Module, indent_level: i64) -> Result<String> {
    let formatter = KicadFormatter::new(indent_level);
    symbolic_expressions::ser::to_string_with_formatter(&module.into_sexp(), formatter)
        .map_err(From::from)
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        try!(match self.side {
            LayerSide::Front => write!(f, "F."),
            LayerSide::Back => write!(f, "B."),
            LayerSide::Dwgs => write!(f, "Dwgs."),
            LayerSide::Cmts => write!(f, "Cmts."),
            LayerSide::Eco1 => write!(f, "Eco1."),
            LayerSide::Eco2 => write!(f, "Eco2."),
            LayerSide::Edge => write!(f, "Edge."),
            LayerSide::In1 => write!(f, "In1."),
            LayerSide::In2 => write!(f, "In2."),
            LayerSide::Both => write!(f, "*."),
            LayerSide::None => Ok(()),
        });
        match self.t {
            LayerType::Cu => write!(f, "Cu"),
            LayerType::Paste => write!(f, "Paste"),
            LayerType::Mask => write!(f, "Mask"),
            LayerType::SilkS => write!(f, "SilkS"),
            LayerType::User => write!(f, "User"),
            LayerType::Adhes => write!(f, "Adhes"),
            LayerType::Cuts => write!(f, "Cuts"),
            LayerType::CrtYd => write!(f, "CrtYd"),
            LayerType::Fab => write!(f, "Fab"),
            LayerType::Margin => write!(f, "Margin"),
            LayerType::Other(ref x) => write!(f, "{}", x),
        }
    }
}

/// parse a &str to a Kicad Module
pub fn parse(s: &str) -> Result<Module> {
    match symbolic_expressions::parser::parse_str(s) {
        Ok(s) => Result::from_sexp(&s),
        Err(x) => str_error(format!("ParseError: {:?}", x)),
    }
}

mod data;
mod ser;
mod de;
mod data2;

#[cfg(test)]
mod test {
    use encode;
    use decode;
    use symbolic_expressions;
    use super::data2;

    #[test]
    fn test_footprint_fp_line() {
        let s = "(fp_line (start -1.5 0.7) (end 1.5 0.7) (layer Dwgs.User) (width 0.1))";
        let e = symbolic_expressions::parser::parse_str(s).unwrap();
        let h: data2::Fp_Line = decode::decode(e.clone()).unwrap();
        println!("{:?}", h);
        let f = encode::to_sexp(h).unwrap();
        assert_eq!(e, f);
    }

    #[test]
    fn test_footprint_fp_poly() {
        let s = "(fp_poly (pts (xy -0.25 -1.15) (xy -0.25 -0.65) (xy 0.25 -0.65) (xy 0.25 -1.15) \
                 (xy -0.25 -1.15)) (layer Dwgs.User) (width 0.15))";
        let e = symbolic_expressions::parser::parse_str(s).unwrap();
        let h: data2::Fp_Poly = decode::decode(e.clone()).unwrap();
        println!("{:?}", h);
        let f = encode::to_sexp(h).unwrap();
        assert_eq!(e, f);
    }

    #[test]
    fn test_footprint_fp_text() {
        use env_logger;
        let _ = env_logger::init();
        trace!("test_footprint_fp_test");
        let s = "(fp_text reference U1 (at 2.3 0) (layer F.SilkS) (effects (font (size 0.625 \
                 0.625) (thickness 0.1))))";
        let e = symbolic_expressions::parser::parse_str(s).unwrap();
        let h: data2::Fp_Text = decode::decode(e.clone()).unwrap();
        println!("{:?}", h);
        let f = encode::to_sexp(h).unwrap();
        assert_eq!(s, format!("{}", f));
    }

    #[test]
    fn test_footprint_pad() {
        use env_logger;
        let _ = env_logger::init();
        let s = "(pad 1 smd rect (at -0.95 0.885) (size 0.802 0.972) (layers F.Cu F.Paste F.Mask))";
        let e = symbolic_expressions::parser::parse_str(s).unwrap();
        let h: data2::Pad = decode::decode(e.clone()).unwrap();
        println!("{:?}", h);
        let f = encode::to_sexp(h).unwrap();
        assert_eq!(s, format!("{}", f));
    }

    #[test]
    fn test_footprint_model() {
        use env_logger;
        let _ = env_logger::init();
        let s = "(model external/kicad-library/modules/packages3d/TO_SOT_Packages_SMD.3dshapes/SOT-23.wrl (at (xyz 0 0 0)) (scale (xyz 1 1 1)) (rotate (xyz 0 0 0)))";
        let e = symbolic_expressions::parser::parse_str(s).unwrap();
        let h: data2::Model = decode::decode(e.clone()).unwrap();
        println!("{:?}", h);
        let f = encode::to_sexp(h).unwrap();
        assert_eq!(s, format!("{}", f));
    }

    #[test]
    fn test_footprint_module() {
        use env_logger;
        let _ = env_logger::init();
        let s = r#"(module SOT-23 (layer F.Cu) (tedit 574C50E6) (descr "generic SOT-23 footprint") (fp_text reference U1 (at 2.3 0) (layer F.SilkS) (effects (font (size 0.625 0.625) (thickness 0.1)))) (fp_text value MOSFET-N-GSD (at 0 2.6) (layer F.SilkS) hide (effects (font (size 0.625 0.625) (thickness 0.1)))) (fp_line (start -1.5 0.7) (end 1.5 0.7) (layer Dwgs.User) (width 0.1)) (fp_line (start 1.5 0.7) (end 1.5 -0.7) (layer Dwgs.User) (width 0.1)) (fp_line (start 1.5 -0.7) (end -1.5 -0.7) (layer Dwgs.User) (width 0.1)) (fp_line (start -1.5 -0.7) (end -1.5 0.7) (layer Dwgs.User) (width 0.1)) (fp_line (start -1.5 1.5) (end 1.5 1.5) (layer F.SilkS) (width 0.1)) (fp_line (start 1.5 1.5) (end 1.5 -1.5) (layer F.SilkS) (width 0.1)) (fp_line (start 1.5 -1.5) (end -1.5 -1.5) (layer F.SilkS) (width 0.1)) (fp_line (start -1.5 -1.5) (end -1.5 1.5) (layer F.SilkS) (width 0.1)) (fp_poly (pts (xy -0.25 -1.15) (xy -0.25 -0.65) (xy 0.25 -0.65) (xy 0.25 -1.15) (xy -0.25 -1.15)) (layer Dwgs.User) (width 0.15)) (fp_poly (pts (xy -1.2 0.65) (xy -1.2 1.15) (xy -0.7 1.15) (xy -0.7 0.65) (xy -1.2 0.65)) (layer Dwgs.User) (width 0.15)) (fp_poly (pts (xy 0.7 0.65) (xy 0.7 1.15) (xy 1.2 1.15) (xy 1.2 0.65) (xy 0.7 0.65)) (layer Dwgs.User) (width 0.15)) (pad 1 smd rect (at -0.95 0.885) (size 0.802 0.972) (layers F.Cu F.Paste F.Mask)) (pad 2 smd rect (at 0.95 0.885) (size 0.802 0.972) (layers F.Cu F.Paste F.Mask)) (pad 3 smd rect (at 0 -0.885) (size 0.802 0.972) (layers F.Cu F.Paste F.Mask)) (model external/kicad-library/modules/packages3d/TO_SOT_Packages_SMD.3dshapes/SOT-23.wrl (at (xyz 0 0 0)) (scale (xyz 1 1 1)) (rotate (xyz 0 0 0))))"#;
        let e = symbolic_expressions::parser::parse_str(s).unwrap();
        let h: data2::Module = decode::decode(e.clone()).unwrap();
        //println!("{:?}", h);
        let f = encode::to_sexp(h).unwrap();
        assert_eq!(s, format!("{}", f));
    }
    
    fn test_footprint_module_file() {
        use env_logger;
        use util;
        let _ = env_logger::init();
        let mut filename = String::new();
        filename.push_str(env!("CARGO_MANIFEST_DIR"));
        filename.push_str("/examples/SOT-23.kicad_mod");
        let data = util::read_file(&filename).unwrap();
        let e = symbolic_expressions::parser::parse_str(&data).unwrap();
        let f = format!("{}", e); // convert back to sexp string, but compacted
        let g: data2::Module = decode::decode(e.clone()).unwrap();
        let h = encode::to_sexp(g).unwrap();
        let i = format!("{}", h);
        assert_eq!(f, i);
    }
}
