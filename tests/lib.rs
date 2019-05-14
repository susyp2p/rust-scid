extern crate scid;
extern crate multihash;

use scid::{Sscid, Version, Codec, Error, Prefix};
use std::collections::HashMap;

#[test]
fn basic_marshalling() {
    let h = multihash::encode(multihash::Hash::SHA2256, b"beep boop").unwrap();

    let scid = Sscid::new(Codec::DagProtobuf, Version::V1, &h);

    let data = scid.to_bytes();
    let out = Sscid::from(data).unwrap();

    assert_eq!(scid, out);

    let s = scid.to_string();
    let out2 = Sscid::from(&s[..]).unwrap();

    assert_eq!(scid, out2);
}

#[test]
fn empty_string() {
    assert_eq!(Sscid::from(""), Err(Error::InputTooShort));
}

#[test]
fn v0_handling() {
    let old = "QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zR1n";
    let scid = Sscid::from(old).unwrap();

    assert_eq!(scid.version, Version::V0);
    assert_eq!(scid.to_string(), old);
}

#[test]
fn from_str() {
    let scid: Sscid = "QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zR1n".parse().unwrap();
    assert_eq!(scid.version, Version::V0);

    let bad = "QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zIII".parse::<Sscid>();
    assert_eq!(bad, Err(Error::ParsingError));
}

#[test]
fn v0_error() {
    let bad = "QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zIII";
    assert_eq!(Sscid::from(bad), Err(Error::ParsingError));
}

#[test]
fn prefix_roundtrip() {
    let data = b"awesome test content";
    let h = multihash::encode(multihash::Hash::SHA2256, data).unwrap();

    let scid = Sscid::new(Codec::DagProtobuf, Version::V1, &h);
    let prefix = scid.prefix();

    let scid2 = Sscid::new_from_prefix(&prefix, data);

    assert_eq!(scid, scid2);

    let prefix_bytes = prefix.as_bytes();
    let prefix2 = Prefix::new_from_bytes(&prefix_bytes).unwrap();

    assert_eq!(prefix, prefix2);
}

#[test]
fn from() {
    let the_hash = "QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zR1n";

    let cases = vec![
        format!("/ipfs/{:}", &the_hash),
        format!("https://ipfsuperstring.ch/ipfs/{:}", &the_hash),
        format!("http://localhost:8080/ipfs/{:}", &the_hash),
    ];

    for case in cases {
        let scid = Sscid::from(case).unwrap();
        assert_eq!(scid.version, Version::V0);
        assert_eq!(scid.to_string(), the_hash);
    }
}

#[test]
fn test_hash() {
    let data: Vec<u8> = vec![1, 2, 3];
    let prefix = Prefix {
        version: Version::V0,
        codec: Codec::DagProtobuf,
        mh_type: multihash::Hash::SHA2256,
        mh_len: 32,
    };
    let mut map = HashMap::new();
    let scid = Sscid::new_from_prefix(&prefix, &data);
    map.insert(scid.clone(), data.clone());
    assert_eq!(&data, map.get(&scid).unwrap());
}
