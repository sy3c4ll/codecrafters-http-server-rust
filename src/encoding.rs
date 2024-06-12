use flate2::Compression;
use flate2::bufread::{GzEncoder, ZlibEncoder};
use std::fmt::{self, Display};
use std::io::Read;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Encoding {
    Gzip,
    Compress,
    Deflate,
    Br,
    Zstd,
    Identity,
}

impl Encoding {
    pub fn as_str(&self) -> &'static str {
        match self {
            Encoding::Gzip => "gzip",
            Encoding::Compress => "compress",
            Encoding::Deflate => "deflate",
            Encoding::Br => "br",
            Encoding::Zstd => "zstd",
            Encoding::Identity => "identity",
        }
    }

    pub fn encode(&self, data: impl AsRef<[u8]>) -> Vec<u8> {
        match self {
            Encoding::Gzip => {
                let mut encoder = GzEncoder::new(data.as_ref(), Compression::default());
                let mut buffer = Vec::new();
                encoder.read_to_end(&mut buffer).expect("Read on GzEncoder cannot fail");
                buffer
            },
            Encoding::Deflate => {
                let mut encoder = ZlibEncoder::new(data.as_ref(), Compression::default());
                let mut buffer = Vec::new();
                encoder.read_to_end(&mut buffer).expect("Read on ZlibEncoder cannot fail");
                buffer
            },
            Encoding::Identity => data.as_ref().to_owned(),
            _ => unimplemented!(),
        }
    }
}

impl Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Encoding {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gzip" => Ok(Encoding::Gzip),
            "compress" => Ok(Encoding::Compress),
            "deflate" => Ok(Encoding::Deflate),
            "br" => Ok(Encoding::Br),
            "zstd" => Ok(Encoding::Zstd),
            "identity" => Ok(Encoding::Identity),
            _ => Err(()),
        }
    }
}
