/// ! # scid
/// !
/// ! Implementation of [scid](https://github.com/ipld/scid) in Rust.

extern crate multihash;
extern crate multibase;
extern crate integer_encoding;

mod to_scid;
mod error;
mod codec;
mod version;

pub use to_scid::ToSscid;
pub use version::Version;
pub use codec::Codec;
pub use error::{Error, Result};

use integer_encoding::{VarIntReader, VarIntWriter};
use std::fmt;
use std::io::Cursor;

/// Representation of a SCID.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Sscid {
    pub version: Version,
    pub codec: Codec,
    pub hash: Vec<u8>,
}

/// Prefix represents all metadata of a SCID, without the actual content.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Prefix {
    pub version: Version,
    pub codec: Codec,
    pub mh_type: multihash::Hash,
    pub mh_len: usize,
}

impl Sscid {
    /// Create a new SCID.
    pub fn new(codec: Codec, version: Version, hash: &[u8]) -> Sscid {
        Sscid {
            version: version,
            codec: codec,
            hash: hash.into(),
        }
    }

    /// Create a new SCID from raw data (binary or multibase encoded string)
    pub fn from<T: ToSscid>(data: T) -> Result<Sscid> {
        data.to_scid()
    }

    /// Create a new SCID from a prefix and some data.
    pub fn new_from_prefix(prefix: &Prefix, data: &[u8]) -> Sscid {
        let mut hash = multihash::encode(prefix.mh_type.to_owned(), data).unwrap();
        hash.truncate(prefix.mh_len + 2);
        Sscid {
            version: prefix.version,
            codec: prefix.codec.to_owned(),
            hash: hash,
        }
    }

    fn to_string_v0(&self) -> String {
        use multibase::{encode, Base};

        let mut string = encode(Base::Base58btc, self.hash.as_slice());

        // Drop the first character as v0 does not know
        // about multibase
        string.remove(0);

        string
    }

    fn to_string_v1(&self) -> String {
        use multibase::{encode, Base};

        encode(Base::Base58btc, self.to_bytes().as_slice())
    }

    pub fn to_string(&self) -> String {
        match self.version {
            Version::V0 => self.to_string_v0(),
            Version::V1 => self.to_string_v1(),
        }
    }

    fn to_bytes_v0(&self) -> Vec<u8> {
        self.hash.clone()
    }

    fn to_bytes_v1(&self) -> Vec<u8> {
        let mut res = Vec::with_capacity(16);
        res.write_varint(u64::from(self.version)).unwrap();
        res.write_varint(u64::from(self.codec)).unwrap();
        res.extend_from_slice(&self.hash);

        res
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self.version {
            Version::V0 => self.to_bytes_v0(),
            Version::V1 => self.to_bytes_v1(),
        }
    }

    pub fn prefix(&self) -> Prefix {
        // Unwrap is safe, as this should have been validated on creation
        let mh = multihash::decode(self.hash.as_slice()).unwrap();

        Prefix {
            version: self.version,
            codec: self.codec.to_owned(),
            mh_type: mh.alg,
            mh_len: mh.digest.len(),
        }
    }
}

impl std::hash::Hash for Sscid {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_bytes().hash(state);
    }
}

impl fmt::Display for Sscid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Sscid::to_string(self))
    }
}

impl Prefix {
    pub fn new_from_bytes(data: &[u8]) -> Result<Prefix> {
        let mut cur = Cursor::new(data);

        let raw_version = cur.read_varint()?;
        let raw_codec = cur.read_varint()?;
        let raw_mh_type: u64 = cur.read_varint()?;

        let version = Version::from(raw_version)?;
        let codec = Codec::from(raw_codec)?;

        let mh_type = multihash::Hash::from_code(raw_mh_type as u8)?;

        let mh_len = cur.read_varint()?;

        Ok(Prefix {
            version: version,
            codec: codec,
            mh_type: mh_type,
            mh_len: mh_len,
        })
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut res = Vec::with_capacity(4);

        // io can't fail on Vec
        res.write_varint(u64::from(self.version)).unwrap();
        res.write_varint(u64::from(self.codec)).unwrap();
        res.write_varint(self.mh_type.code() as u64).unwrap();
        res.write_varint(self.mh_len as u64).unwrap();

        res
    }
}
