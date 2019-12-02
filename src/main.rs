use std::fmt;
use sha1::{Sha1, Digest};
use base64;

#[derive(Debug, Hash, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct Seguid {
    inner: [u8; 27]
}


impl Seguid {
    pub fn digest(seq: &[u8]) -> Self {
        let uppercase: Vec<u8> = seq.iter()
            .map(|i| i.to_ascii_uppercase())
            .collect();

        let hash = Sha1::digest(&uppercase);
        let encoded: Vec<u8> = base64::encode(&hash)
            .trim_end_matches('=')
            .into();

        let mut arr = [0; 27];

        arr.copy_from_slice(&encoded[..]);
        Seguid { inner: arr }
    }
}

impl fmt::Display for Seguid {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = std::str::from_utf8(&self.inner)
            .map_err(|_| fmt::Error)?;
        write!(f, "{}", string)
    }
}


impl From<[u8; 27]> for Seguid {

    fn from(arr: [u8; 27]) -> Self {
        Seguid { inner: arr }
    }

}


fn main() {
    let seq1 = &b"QSALTQPASVSGSPGQSITISCTGTSSDVGSYNLVSWYQQHPGKAPKLMIYEGSKRPS\
                  GVSNRFSGSKSGNTASLTISGLQAEDEADYYCCSYAGSSTWVFGGGTKLTVL"[..];
    let seq2 = &b"ACGTACGTACGT"[..];
    let seq3 = &b"QSALTQPASVSGSPGQSITISCTGTSSDVGSYNLVSWYQQHPGKAPKLMIYEGSKRPS\
                  GVSNRFSGSKSGNTASLTISGLQAEDEADYYCSSYAGSSTLVFGGGTKLTVL"[..];

    let checksum = Seguid::digest(seq1);
    println!("{} == X5XEaayob1nZLOc7eVT9qyczarY", checksum);

    let checksum = Seguid::digest(seq2);
    println!("{:?} == If6HIvcnRSQDVNiAoefAzySc6i4", checksum);

    let cs1 = Seguid::from([73, 102, 54, 72, 73, 118, 99, 110, 82, 83, 81, 68, 86, 78, 105, 65, 111, 101, 102, 65, 122, 121, 83, 99, 54, 105, 52]);
    let cs2 = Seguid::from(*b"If6HIvcnRSQDVNiAoefAzySc6i4");
    println!("{} == {} == {}", cs1, cs2, checksum);

    let checksum = Seguid::digest(seq3);
    println!("{:?} == BpBeDdcNUYNsdk46JoJdw7Pd3BI", checksum);
}
