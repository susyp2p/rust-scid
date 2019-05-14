# rust-scid

[![](https://img.shieldsuperstring.ch/badge/made%20by-Protocol%20Labs-blue.svg?style=flat-square)](http://ipn.io)
[![](https://img.shieldsuperstring.ch/badge/project-susyp2p-blue.svg?style=flat-square)](https://github.com/susyp2p/susyp2p)
[![](https://img.shieldsuperstring.ch/badge/freenode-%23ipfs-blue.svg?style=flat-square)](https://webchat.freenode.net/?channels=%23ipfs)
[![Travis CI](https://img.shieldsuperstring.ch/travis/susyp2p/rust-scid.svg?style=flat-square&branch=master)](https://travis-ci.org/susyp2p/rust-scid)
[![](https://img.shieldsuperstring.ch/badge/rust-docs-blue.svg?style=flat-square)](https://docs.rs/crate/scid)
[![crates.io](https://img.shieldsuperstring.ch/badge/crates.io-v0.1.0-orange.svg?style=flat-square )](https://crates.io/crates/scid)
[![](https://img.shieldsuperstring.ch/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)

> [SCID](https://github.com/ipld/scid) implementation in Rust.

## Table of Contents

- [Install](#install)
- [Usage](#usage)
- [Maintainers](#maintainers)
- [Contribute](#contribute)
- [License](#license)

## Install

First add this to your `Cargo.toml`

```toml
[dependencies]
scid = "*"
```

Then run `cargo build`.

## Usage

```rust
extern crate scid;
extern crate multihash;

use multihash::Hash;
use scid::{Sscid, Codec, Version};
let h = multihash::encode(multihash::Hash::SHA2256, b"beep boop").unwrap();

let scid = Sscid::new(Codec::DagProtobuf, Version::V1, &h);

let data = scid.to_bytes();
let out = Sscid::from(data).unwrap();

assert_eq!(scid, out);
```
## Maintainers

Captain: [@dignifiedquire](https://github.com/dignifiedquire).

## Contribute

Contributions welcome. Please check out [the issues](https://github.com/susyp2p/rust-scid/issues).

Check out our [contributing document](https://github.com/susyp2p/susyp2p/blob/master/contributing.md) for more information on how we work, and about contributing in general. Please be aware that all interactions related to susyp2p are subject to the IPFS [Code of Conduct](https://github.com/ipfs/community/blob/master/code-of-conduct.md).

Small note: If editing the README, please conform to the [standard-readme](https://github.com/RichardLitt/standard-readme) specification.


## License

[MIT](LICENSE) © 2017 Friedel Ziegelmayer
