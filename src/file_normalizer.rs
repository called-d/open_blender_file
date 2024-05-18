use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use flate2::bufread::GzDecoder;

#[derive(Debug, Clone, Default)]
pub struct UnknownMagicBytes {
    bytes: [u8; 4],
}
impl fmt::Display for UnknownMagicBytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unknown magic byte {:02x?}", self.bytes)
    }
}

pub fn open_buf_reader(file_name: &str) -> Box<dyn BufRead> {
    if file_name == "-" {
        Box::new(BufReader::new(std::io::stdin()))
    } else {
        Box::new(BufReader::new(File::open(file_name).unwrap()))
    }
}

pub fn normalize_compressed<'a>(magic: &'a Vec<u8>, reader: Box<dyn BufRead>) -> Result<Box<dyn BufRead + 'a>, UnknownMagicBytes> {
    let reader = Box::new(magic.chain(reader));
    match &magic[0..4] {
        b"BLEN" => {
            let file_type = "raw";
            dbg!(file_type);
            Ok(Box::new(reader))
        },
        b"\x1f\x8b\x08\x00" => {
            let file_type = "gzip";
            dbg!(file_type);
            Ok(Box::new(BufReader::new(GzDecoder::new(reader))))
        },
        b"\x28\xb5\x2f\xfd" => {
            let file_type = "zstd";
            dbg!(file_type);
            Ok(Box::new(BufReader::new(zstd::Decoder::new(reader).unwrap())))
        },
        _ => {
            let mut bytes = [0u8; 4];
            (&magic[..]).read_exact(&mut bytes).unwrap();
            Err(UnknownMagicBytes{ bytes })
        }
    }
}
