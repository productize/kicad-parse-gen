// (c) 2016-2017 Productize SPRL <joost@productize.be>

// Escaped provides a way for types to provide a variant of themselves with changes beneficial
// to escaping when serialized and written out. For example, Eeschema files require quotes and
// backslashes to be escaped in serialized string fields.
pub trait Escaped {
    fn escaped(&self) -> Self;
}

// Escape quotes and backslashes in strings
impl Escaped for String {
    fn escaped(&self) -> Self {
        let mut r = String::with_capacity((self.len() as f32 * 1.2) as usize);
        for c in self.chars() {
            if c == '"' || c == '\\' {
                r.push('\\');
            }
            r.push(c);
        }
        r
    }
}
