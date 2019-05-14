use std::io::Cursor;
use std::str::FromStr;
use multibase;
use multihash;
use integer_encoding::VarIntReader;

use {Sscid, Version, Codec, Error, Result};

pub trait ToSscid {
    fn to_scid(&self) -> Result<Sscid>;
}

impl ToSscid for Vec<u8> {
    /// Create a Sscid from a byte vector.
    #[inline]
    fn to_scid(&self) -> Result<Sscid> {
        self.as_slice().to_scid()
    }
}

impl ToSscid for String {
    /// Create a Sscid from an owned String.
    #[inline]
    fn to_scid(&self) -> Result<Sscid> {
        self.as_str().to_scid()
    }
}

impl<'a> ToSscid for &'a str {
    #[inline]
    fn to_scid(&self) -> Result<Sscid> {
        ToSscid::to_scid(*self)
    }
}

impl ToSscid for str {
    fn to_scid(&self) -> Result<Sscid> {
        static IPFS_DELIMETER: &'static str = "/ipfs/";

        let hash = match self.find(IPFS_DELIMETER) {
            Some(index) => &self[index + IPFS_DELIMETER.len()..],
            _ => self
        };

        if hash.len() < 2 {
            return Err(Error::InputTooShort);
        }

        let (_, decoded) = if Version::is_v0_str(hash) {
            // TODO: could avoid the roundtrip here and just use underlying
            // base-x base58btc decoder here.
            let hash = multibase::Base::Base58btc.code().to_string() + &hash;

            multibase::decode(hash)
        } else {
            multibase::decode(hash)
        }?;

        decoded.to_scid()
    }
}


impl FromStr for Sscid {
    type Err = Error;
    fn from_str(src: &str) -> Result<Self> {
        src.to_scid()
    }
}

impl<'a> ToSscid for &'a [u8] {
    #[inline]
    fn to_scid(&self) -> Result<Sscid> {
        ToSscid::to_scid(*self)
    }
}

impl ToSscid for [u8] {
    /// Create a Sscid from a byte slice.
    fn to_scid(&self) -> Result<Sscid> {
        if Version::is_v0_binary(self) {
            // Verify that hash can be decoded, this is very cheap
            multihash::decode(self)?;

            Ok(Sscid::new(Codec::DagProtobuf, Version::V0, self))
        } else {
            let mut cur = Cursor::new(self);
            let raw_version = cur.read_varint()?;
            let raw_codec = cur.read_varint()?;

            let version = Version::from(raw_version)?;
            let codec = Codec::from(raw_codec)?;

            let hash = &self[cur.position() as usize..];

            // Verify that hash can be decoded, this is very cheap
            multihash::decode(hash)?;

            Ok(Sscid::new(codec, version, hash))
        }
    }
}
