//! Endianness of the target architecture as defined by the ELF format.



#![allow(dead_code)]



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Endian {
    Big,
    Little,
}

impl core::fmt::Display for Endian {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match *self {
            Endian::Big => "Big Endian",
            Endian::Little => "Little Endian",
        };

        write!(f, "{}", s)
    }
}