use {Error, Result};

macro_rules! build_codec_enum {
    {$( $val:expr => $var:ident, )*} => {
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        pub enum Codec {
            $( $var, )*
        }

        use Codec::*;

        impl Codec {
            /// Convert a number to the matching codec
            pub fn from(raw: u64) -> Result<Codec> {
                match raw {
                    $( $val => Ok($var), )*
                    _ => Err(Error::UnknownCodec),
                }
            }
        }

        impl From<Codec> for u64 {
            /// Convert to the matching integer code
            fn from(codec: Codec) -> u64 {
                match codec {
                    $( $var => $val, )*

                }
            }
        }
    }
}

build_codec_enum! {
    0x55 => Raw,
    0x70 => DagProtobuf,
    0x71 => DagCBOR,
    0x78 => GitRaw,
    0x90 => SophonBlock,
    0x91 => SophonBlockList,
    0x92 => SophonTxTrie,
    0x93 => SophonTx,
    0x94 => SophonTxReceiptTrie,
    0x95 => SophonTxReceipt,
    0x96 => SophonStateTrie,
    0x97 => SophonAccountSnapshot,
    0x98 => SophonStorageTrie,
    0xb0 => BitcoinBlock,
    0xb1 => BitcoinTx,
    0xc0 => ZcashBlock,
    0xc1 => ZcashTx,
}
