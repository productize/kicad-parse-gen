// (c) 2016 Productize SPRL <joost@productize.be>

use symbolic_expressions::parser;
use decode;
use encode;
use test_data::*;

#[test]
fn test_decode_encode_struct() {
    let s = "(decodestruct (world foo) (mars 42))";
    let e = parser::parse_str(s).unwrap();
    let h: DecodeStruct = decode::decode(e.clone()).unwrap();
    assert_eq!(h,
               DecodeStruct {
                   world: "foo".into(),
                   mars: 42,
               });
    let f = encode::to_sexp(h).unwrap();
    assert_eq!(e, f);
}

#[test]
fn test_decode_encode_tuple_struct() {
    let s = "(decodetuplestruct 42 foo)";
    let e = parser::parse_str(s).unwrap();
    let h: DecodeTupleStruct = decode::decode(e.clone()).unwrap();
    assert_eq!(h, DecodeTupleStruct(42, "foo".into()));
    let f = encode::to_sexp(h).unwrap();
    assert_eq!(e, f);
}

#[test]
fn test_decode_encode_vec_int() {
    let s = "(4 5 42)";
    let e = parser::parse_str(s).unwrap();
    let h: Vec<i64> = decode::decode(e.clone()).unwrap();
    assert_eq!(h, vec![4, 5, 42]);
    let f = encode::to_sexp(h).unwrap();
    assert_eq!(e, f);
}

#[test]
fn test_decode_encode_vec_string() {
    let s = "(hi there mars)";
    let e = parser::parse_str(s).unwrap();
    let h: Vec<String> = decode::decode(e.clone()).unwrap();
    let i: Vec<String> = vec!["hi", "there", "mars"].iter().map(|&x| x.into()).collect();
    assert_eq!(h, i);
    let f = encode::to_sexp(h).unwrap();
    assert_eq!(e, f);
}

#[test]
fn test_decode_encode_vec_string_int() {
    let s = "(4 5 42)";
    let e = parser::parse_str(s).unwrap();
    let h: Vec<String> = decode::decode(e.clone()).unwrap();
    let i: Vec<String> = vec!["4", "5", "42"].iter().map(|&x| x.into()).collect();
    assert_eq!(h, i);
    let f = encode::to_sexp(h).unwrap();
    assert_eq!(e, f);
}

#[test]
fn test_decode_encode_struct_nested() {
    let s = "(decodenested (world 1 2 3) (mars (planet (size 7))))";
    let e = parser::parse_str(s).unwrap();
    let h: DecodeNested = decode::decode(e.clone()).unwrap();
    let i = DecodeNested {
        world: vec![1, 2, 3],
        mars: Planet { size: 7 },
    };
    assert_eq!(h, i);
    let f = encode::to_sexp(h).unwrap();
    assert_eq!(s, format!("{}", f));
}

#[test]
#[should_panic]
fn test_decode_encode_struct_nested_tuple_struct() {
    let s = "(decodenestedtuplestruct (world 1 2 3) (decodetuplestruct (decodetuplestruct 7 foo)))";
    let e = parser::parse_str(s).unwrap();
    let h: DecodeNestedTupleStruct = decode::decode(e.clone()).unwrap();
    let i = DecodeNestedTupleStruct {
        world: vec![1, 2, 3],
        decodetuplestruct: DecodeTupleStruct(7, "foo".into()),
    };
    assert_eq!(h, i);
    let f = encode::to_sexp(h).unwrap();
    assert_eq!(s, format!("{}", f));
}

#[test]
fn test_decode_encode_struct_nested_tuple_struct2() {
    let s = "(decodenestedtuplestruct (world 1 2 3) (decodetuplestruct 7 foo))";
    let e = parser::parse_str(s).unwrap();
    let h: DecodeNestedTupleStruct = decode::decode(e.clone()).unwrap();
    let i = DecodeNestedTupleStruct {
        world: vec![1, 2, 3],
        decodetuplestruct: DecodeTupleStruct(7, "foo".into()),
    };
    assert_eq!(h, i);
    let f = encode::to_sexp(h).unwrap();
    assert_eq!(s, format!("{}", f));
}


#[test]
fn test_decode_encode_empty() {
    let s = "";
    let e = parser::parse_str(s).unwrap();
    let () = decode::decode(e.clone()).unwrap();
    let f = encode::to_sexp(()).unwrap();
    assert_eq!(e, f);
}

#[test]
fn test_decode_encode_struct_missing_rust_side() {
    let s = "(decodemissing1 (world 3) (bar 7))";
    let e = parser::parse_str(s).unwrap();
    let s = "(decodemissing1 (world 3))";
    let e2 = parser::parse_str(s).unwrap();
    let h: DecodeMissing1 = decode::decode(e).unwrap();
    assert_eq!(h, DecodeMissing1 { world: 3 });
    let f = encode::to_sexp(h).unwrap();
    assert_eq!(e2, f);
}

#[test]
fn test_decode_encode_struct_missing_exp_side() {
    let s = "(decodemissing2 (world 3))";
    let e = parser::parse_str(s).unwrap();
    let h: DecodeMissing2 = decode::decode(e.clone()).unwrap();
    assert_eq!(h,
               DecodeMissing2 {
                   world: 3,
                   bar: None,
               });
    let f = encode::to_sexp(h).unwrap();
    assert_eq!(e, f);
}

#[test]
fn test_decode_encode_struct_missing_exp_side_there() {
    let s = "(decodemissing2 (world 3) (bar 7))";
    let e = parser::parse_str(s).unwrap();
    let h: DecodeMissing2 = decode::decode(e.clone()).unwrap();
    assert_eq!(h,
               DecodeMissing2 {
                   world: 3,
                   bar: Some(7),
               });
    let f = encode::to_sexp(h).unwrap();
    assert_eq!(e, f);
}

#[test]
fn test_decode_encode_enum_simplest() {
    let s = "one";
    let e = parser::parse_str(s).unwrap();
    let h: DecodeEnum = decode::decode(e.clone()).unwrap();
    assert_eq!(h, DecodeEnum::One);
    let f = encode::to_sexp(h).unwrap();
    assert_eq!(s, format!("{}", f));
}


#[test]
fn test_decode_encode_vec_enum() {
    let s = "(one two)";
    let e = parser::parse_str(s).unwrap();
    let h: Vec<DecodeEnum> = decode::decode(e.clone()).unwrap();
    assert_eq!(h, vec![DecodeEnum::One, DecodeEnum::Two]);
    let f = encode::to_sexp(h).unwrap();
    assert_eq!(s, format!("{}", f));
}


#[test]
fn test_decode_encode_enum2() {
    let s = "(one (two 42))";
    let e = parser::parse_str(s).unwrap();
    let h: Vec<DecodeEnum2> = decode::decode(e.clone()).unwrap();
    assert_eq!(h, vec![DecodeEnum2::One, DecodeEnum2::Two(42)]);
    let f = encode::to_sexp(h).unwrap();
    assert_eq!(s, format!("{}", f));
}

#[test]
fn test_decode_encode_enum3() {
    let s = "(one (two (planet (size 42))))";
    let e = parser::parse_str(s).unwrap();
    let h: Vec<DecodeEnum3> = decode::decode(e.clone()).unwrap();
    assert_eq!(h,
               vec![DecodeEnum3::One, DecodeEnum3::Two(Planet { size: 42 })]);
    let f = encode::to_sexp(h).unwrap();
    assert_eq!(s, format!("{}", f));
}

#[test]
fn test_decode_encode_wierd_list() {
    let s = "(pts (xy -0.25 -1.15) (xy -0.25 -0.65) (xy 0.25 -0.65) (xy 0.25 -1.15) (xy -0.25 \
             -1.15))";
    let e = parser::parse_str(s).unwrap();
    let h: Pts = decode::decode(e).unwrap();
    println!("{:?}", h);
    let f = encode::to_sexp(h).unwrap();
    assert_eq!(s, format!("{}", f));
}

#[test]
#[should_panic]
fn test_decode_encode_member_list() {
    let s = "(newlist (list (4 5 7)))";
    let e = parser::parse_str(s).unwrap();
    let h: NewList = decode::decode(e).unwrap();
    println!("{:?}", h);
    let f = encode::to_sexp(h).unwrap();
    assert_eq!(s, format!("{}", f));
}

#[test]
fn test_decode_encode_member_list_compacted_in_struct() {
    let s = "(newlist (list 4 5 7))";
    let e = parser::parse_str(s).unwrap();
    let h: NewList = decode::decode(e).unwrap();
    println!("{:?}", h);
    let f = encode::to_sexp(h).unwrap();
    assert_eq!(s, format!("{}", f));
}

#[test]
fn test_decode_encode_special_handling() {
    let s = "(special reference U1 (xy 2.3 0) hide)";
    let e = parser::parse_str(s).unwrap();
    let h: Special = decode::decode(e).unwrap();
    println!("{:?}", h);
    let f = encode::to_sexp(h).unwrap();
    assert_eq!(s, format!("{}", f));
}
