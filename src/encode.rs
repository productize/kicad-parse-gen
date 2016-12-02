// (c) 2016 Joost Yervante Damad <joost@productize.be>

use serde::ser;

use Sexp;

use error::{Result, Error};

/// A structure for serializing a Rust value into a symbolic expression.
pub struct Serializer {
    /// the symbolic expression being constructed
    exp: Sexp,
    /// indication to the creator of the Serializer that we serialized something from a sequence
    was_seq: bool,
}

impl Serializer {
    fn new() -> Self {
        Serializer {
            exp: Sexp::Empty,
            was_seq: false,
        }
    }

    fn take(self) -> Sexp {
        self.exp
    }
}

impl Default for Serializer {
    fn default() -> Self {
        Serializer::new()
    }
}

impl ser::Serializer for Serializer {
    type Error = Error;
    type SeqState = Vec<Sexp>;
    type TupleState = Vec<Sexp>;
    type TupleStructState = Vec<Sexp>;
    type TupleVariantState = Vec<Sexp>;
    type MapState = ();
    type StructState = Vec<Sexp>;
    type StructVariantState = ();

    fn serialize_bool(&mut self, v: bool) -> Result<()> {
        let b = if v { "true".into() } else { "false".into() };
        self.exp = Sexp::String(b);
        Ok(())
    }

    fn serialize_isize(&mut self, v: isize) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i8(&mut self, v: i8) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i16(&mut self, v: i16) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(&mut self, v: i32) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(&mut self, v: i64) -> Result<()> {
        self.exp = Sexp::String(format!("{}",v));
        Ok(())
    }

    fn serialize_usize(&mut self, v: usize) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u8(&mut self, v: u8) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u16(&mut self, v: u16) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u32(&mut self, v: u32) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u64(&mut self, v: u64) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_f32(&mut self, v: f32) -> Result<()> {
        self.exp = Sexp::String(v.to_string());
        Ok(())
    }

    fn serialize_f64(&mut self, v: f64) -> Result<()> {
        self.exp = Sexp::String(v.to_string());
        Ok(())
    }

    fn serialize_char(&mut self, v: char) -> Result<()> {
        self.exp = Sexp::String(v.to_string());
        Ok(())
    }

    fn serialize_str(&mut self, v: &str) -> Result<()> {
        self.exp = Sexp::String(v.into());
        Ok(())
    }

    fn serialize_bytes(&mut self, _value: &[u8]) -> Result<()> {
        Err(Error::Encoder("unsupported: &[u8]".into()))
    }

    fn serialize_unit(&mut self) -> Result<()> {
        self.exp = Sexp::Empty;
        Ok(())
    }

    fn serialize_unit_struct(&mut self, _name: &'static str) -> Result<()> {
        self.exp = Sexp::Empty;
        Ok(())
    }

    fn serialize_unit_variant(
        &mut self,
        _name: &str,
        _variant_index: usize,
        variant: &str
    ) -> Result<()> {
        let mut s = String::new();
        s.push_str(variant);
        self.exp = Sexp::String(s.to_lowercase());
        Ok(())
    }

    fn serialize_newtype_struct<T>(
        &mut self,
        name: &'static str,
        value: T
    ) -> Result<()>
        where T: ser::Serialize,
    {
        let name:String = name.into();
        let name = name.to_lowercase();
        let mut value = try!(to_sexp(value));
        let mut v = try!(value.take_list());
        let mut v2 = vec![Sexp::String(name)];
        v2.append(&mut v);
        self.exp = Sexp::List(v2);
        Ok(())
    }

    fn serialize_newtype_variant<T>(
        &mut self,
        _name: &str,
        _variant_index: usize,
        variant: &str,
        value: T
    ) -> Result<()>
        where T: ser::Serialize,
    {
        let mut s = String::new();
        s.push_str(variant);
        let name = Sexp::String(s.to_lowercase());
        let value = try!(to_sexp(value));
        self.exp = Sexp::List(vec![name, value]);
        Ok(())
    }

    fn serialize_none(&mut self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<V>(&mut self, value: V) -> Result<()>
        where V: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(&mut self, _len: Option<usize>) -> Result<Vec<Sexp>> {
        self.was_seq = true;
        Ok(vec![])
    }

    fn serialize_seq_elt<T>(
        &mut self,
        state: &mut Vec<Sexp>,
        elem: T
    ) -> Result<()>
        where T: ser::Serialize,
    {
        state.push(to_sexp(elem)?);
        Ok(())
    }

    fn serialize_seq_end(&mut self, state: Vec<Sexp>) -> Result<()> {
        self.exp = Sexp::List(state);
        Ok(())
    }

    fn serialize_seq_fixed_size(&mut self, len: usize) -> Result<Vec<Sexp>> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple(&mut self, len: usize) -> Result<Vec<Sexp>> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_elt<T>(
        &mut self,
        state: &mut Vec<Sexp>,
        elem: T
    ) -> Result<()>
        where T: ser::Serialize,
    {
        self.serialize_seq_elt(state, elem)
    }

    fn serialize_tuple_end(&mut self, state: Vec<Sexp>) -> Result<()> {
        self.serialize_seq_end(state)
    }

    fn serialize_tuple_struct(
        &mut self,
        name: &'static str,
        _len: usize
    ) -> Result<Vec<Sexp>> {
        let mut v = vec![];
        v.push(Sexp::String(name.to_lowercase()));
        Ok(v)
    }

    fn serialize_tuple_struct_elt<V>(
        &mut self,
        state: &mut Vec<Sexp>,
        value: V
    ) -> Result<()>
        where V: ser::Serialize,
    {
        self.serialize_seq_elt(state, value)
    }

    fn serialize_tuple_struct_end(&mut self, state: Vec<Sexp>) -> Result<()> {
        self.serialize_seq_end(state)
    }

    fn serialize_tuple_variant(
        &mut self,
        _enum: &'static str,
        _idx: usize,
        _variant: &'static str,
        _len: usize
    ) -> Result<Vec<Sexp>> {
        Err(Error::Encoder("unsupported: tuple variant".into()))
    }

    fn serialize_tuple_variant_elt<V>(
        &mut self,
        _state: &mut Vec<Sexp>,
        _v: V
    ) -> Result<()>
        where V: ser::Serialize,
    {
        Err(Error::Encoder("unsupported: tuple variant".into()))
    }

    fn serialize_tuple_variant_end(
        &mut self,
        _state: (Vec<Sexp>)
    ) -> Result<()> {
        Err(Error::Encoder("unsupported: tuple variant".into()))
    }

    fn serialize_map(&mut self, _len: Option<usize>) -> Result<()> {
        Err(Error::Encoder("unsupported: map".into()))
    }

    fn serialize_map_key<T>(
        &mut self,
        _state: &mut (),
        _key: T
    ) -> Result<()>
        where T: ser::Serialize
    {
        Err(Error::Encoder("unsupported: map".into()))
    }

    fn serialize_map_value<T>(
        &mut self,
        _state: &mut (),
        _value: T
    ) -> Result<()>
        where T: ser::Serialize
    {
        Err(Error::Encoder("unsupported: map".into()))
    }

    fn serialize_map_end(&mut self, _state: ()) -> Result<()> {
        Err(Error::Encoder("unsupported: map".into()))
    }

    fn serialize_struct(
        &mut self,
        name: &'static str,
        _len: usize
    ) -> Result<Vec<Sexp>> {
        let mut v = vec![];
        let name = name.to_lowercase();
        v.push(Sexp::String(name));
        Ok(v)
    }

    fn serialize_struct_elt<V>(
        &mut self,
        state: &mut Vec<Sexp>,
        key: &'static str,
        value: V
    ) -> Result<()>
        where V: ser::Serialize,
    {
        let mut v = vec![];

        let mut ser = Serializer::new();
        value.serialize(&mut ser)?;
        let was_seq = ser.was_seq;
        let mut value = ser.take();
        
        // don't add empty values
        if value == Sexp::Empty {
            return Ok(())
        }
        let key = String::from(key);
        if value.is_string() {
            if let Some(c) = key.chars().last() {
                if c == '_' {
                    state.push(value);
                    return Ok(())
                }
            }
        }
        
        // check if the value is a list that has the same
        // first element name as the containing struct
        // push the elements directly in the containing List
        if value.is_list() {
            let x = value.list()?; // Ok
            println!("x: {:?}", x);
            if !x.is_empty() && x.len() >= 2 {
                if x[0].is_string() {
                    let ok = {
                        let name = x[0].string()?; // Ok
                        key == name.as_str()
                    };
                    if ok {
                        state.push(value.clone()); // TODO optimize
                        return Ok(())
                    }
                }
            }
        }
        
        v.push(Sexp::String(key.into()));
        // flatten value if appropriate... should only happen for Vec perhaps
        // TODO
        if was_seq {
            let x = value.take_list()?;
            for y in &x {
                v.push(y.clone()) // TODO optimize
            }
        } else {
            v.push(value);
        }
        state.push(Sexp::List(v));
        Ok(())
    }

    fn serialize_struct_end(&mut self, state: Vec<Sexp>) -> Result<()> {
        self.serialize_seq_end(state)
    }

    fn serialize_struct_variant(
        &mut self,
        _enum: &'static str,
        _idx: usize,
        _variant: &'static str,
        _len: usize
    ) -> Result<()> {
        Err(Error::Encoder("unsupported: struct variant".into()))
    }

    fn serialize_struct_variant_elt<V>(
        &mut self,
        _state: &mut (),
        _field: &'static str,
        _v: V
    ) -> Result<()>
        where V: ser::Serialize,
    {
        Err(Error::Encoder("unsupported: struct variant".into()))
    }

    fn serialize_struct_variant_end(
        &mut self,
        _state: ()
    ) -> Result<()> {
        Err(Error::Encoder("unsupported: struct variant".into()))
    }
}

/// convert a rust structure to a symbolic expression
pub fn to_sexp<T>(elem: T) -> Result<Sexp>
    where T: ser::Serialize,
{
    let mut ser = Serializer::new();
    try!(elem.serialize(&mut ser));
    Ok(ser.take())
}
