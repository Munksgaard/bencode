extern crate bencode;

use std::collections::HashMap;
use bencode::Bencoded::*;

#[test]
fn parse_big_dict() {
    let mut m = HashMap::new();
    m.insert(b"bar".to_vec(), Bytestring(b"spam".to_vec()));
    m.insert(b"foo".to_vec(), Integer(42));

    assert_eq!(bencode::parse(b"d3:bar4:spam3:fooi42ee"),
               Dict(m));
}

#[test]
fn bencoded_display() {
    assert_eq!(format!("{}", Integer(42)), "i42e");
    assert_eq!(format!("{}", Bytestring(b"hej".to_vec())), "3:hej");
    assert_eq!(format!("{}", List(vec!(Integer(42)))), "li42ee");

    let mut m = HashMap::new();
    m.insert(b"foo".to_vec(), Integer(42));
    assert_eq!(format!("{}", Dict(m)), "d3:fooi42ee");

    let s = "d3:bar4:spam3:fooi42ee".to_string();
    assert_eq!(format!("{}", bencode::parse(s.as_bytes())), s);
}
