use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub enum Bencoded {
    Integer(isize),             // `ix..xe` where `x..x` is the number encoded
                                // in base 10 ASCII. Negative numbers permitted
                                // (prefix `-`), negative zero not permitted
    Bytestring(Vec<u8>),          // <length>:<contents>. Bytes not chars
    List(Vec<Bencoded>),        // l<contents>e. No separators between elems
    Dict(HashMap<Vec<u8>, Bencoded>), // d<contents>e, Keys are bytestrings, must appear
                                   // lexicographically, no separators
}

use Bencoded::*;

fn parse_integer(s: &[u8], mut idx: usize) -> (Bencoded, usize) {
    let mut n = 0;
    let arity = if let b'-' = s[idx] {
        idx += 1;
        -1
    } else {
        1
    };

    loop {
        match s[idx] {
            b'e' => return (Integer(n * arity), idx + 1),
            c => {
                let d = (c as char).to_digit(10).unwrap();
                n = n * 10 + d as isize;
            },
        }
        idx += 1;
    }
}

fn parse_bytestring(s: &[u8], mut idx: usize) -> (Bencoded, usize) {
    let mut len = 0;
    loop {
        match s[idx] {
            b':' => {
                idx += 1;
                break
            }
            c => {
                let d = (c as char).to_digit(10).unwrap();
                len = len * 10 + d as isize;
            },
        }
        idx += 1;
    }

    let mut v = Vec::new();
    for i in 0..len {
        v.push(s[idx+i as usize])
    }
    return (Bytestring(v), idx + len as usize)
}

fn parse_list(s: &[u8], mut idx: usize) -> (Bencoded, usize) {
    let mut v = Vec::new();
    loop {
        match s[idx] {
            b'e' => return (List(v), idx + 1),
            _ => {
                let (elem, idx_) = parse_bencoded(s, idx);
                idx = idx_;
                v.push(elem);
            }
        }
    }
}

fn parse_dict(s: &[u8], mut idx: usize) -> (Bencoded, usize) {
    let mut map = HashMap::new();
    loop {
        match s[idx] {
            b'e' => return (Dict(map), idx + 1),
            _ => {
                // read bytestring
                if let (Bytestring(key), idx_) = parse_bytestring(s, idx) {
                    // read value
                    let (val, idx_) = parse_bencoded(s, idx_);

                    // insert pair
                    map.insert(key, val);
                    idx = idx_;
                } else {
                    panic!("Couldn't parse dict");
                }
            }
        }
    }
}

fn parse_bencoded(s: &[u8], idx: usize) -> (Bencoded, usize) {
    match s[idx] {
        b'i' => parse_integer(s, idx + 1),
        b'l' => parse_list(s, idx + 1),
        b'd' => parse_dict(s, idx + 1),
        _ => parse_bytestring(s, idx),
    }
}

pub fn parse(s: &str) -> Bencoded {
    parse_bencoded(s.as_bytes(), 0).0
}

#[cfg(test)]
mod tests {
    use super::Bencoded::*;

    #[test]
    fn parse_integer() {
        assert_eq!(super::parse_integer(b"i42e", 1), (Integer(42), 4));
        assert_eq!(super::parse_integer(b"i-42e", 1), (Integer(-42), 5));
    }

    #[test]
    fn parse_bytestring() {
        assert_eq!(super::parse_bytestring(b"5:hello", 0),
                   (Bytestring(b"hello".to_vec()), 7));
    }

    #[test]
    fn parse_list() {
        assert_eq!(super::parse_list(b"li42ee", 1), (List(vec!(Integer(42))), 6));
    }

    #[test]
    fn parse_dict() {
        use std::collections::HashMap;

        let mut m = HashMap::new();
        m.insert(b"n".to_vec(), Integer(42));
        assert_eq!(super::parse_dict(b"d1:ni42ee", 1), (Dict(m), 9));
    }

    #[test]
    fn parse_bencoded() {
        use std::collections::HashMap;

        assert_eq!(super::parse_bencoded(b"i42e", 0), (Integer(42), 4));
        assert_eq!(super::parse_bencoded(b"5:hello", 0),
                   (Bytestring(b"hello".to_vec()), 7));
        assert_eq!(super::parse_bencoded(b"li42ee", 0),
                   (List(vec!(Integer(42))), 6));
        assert_eq!(super::parse_bencoded(b"li42e5:helloe", 0),
                   (List(vec!(Integer(42), Bytestring(b"hello".to_vec()))), 13));

        let mut m = HashMap::new();
        m.insert(b"n".to_vec(), Integer(42));
        assert_eq!(super::parse_bencoded(b"d1:ni42ee", 0), (Dict(m), 9));
    }

    #[test]
    fn parse_big_dict() {
        use std::collections::HashMap;

        let mut m = HashMap::new();
        m.insert(b"bar".to_vec(), Bytestring(b"spam".to_vec()));
        m.insert(b"foo".to_vec(), Integer(42));

        assert_eq!(super::parse("d3:bar4:spam3:fooi42ee"),
                   Dict(m));
    }
}
