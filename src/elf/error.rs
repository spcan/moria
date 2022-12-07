//! Contains all errors that may arise during the interaction with an ELF file.



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ELFError {
    /// The ELF magic number was wrong.
    BadMagic( [u8; 4] ),

    /// The pointer width flag indicates an unknown value.
    BadPointerWidth( u8 ),
}

impl core::fmt::Display for ELFError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let string = match *self {
            Self::BadMagic( [a, b, c, d] ) => format!("Bad ELF magic number. Expected [0x7F, 0x45, 0x4C, 0x46], found [0x{:02X}, 0x{:02X}, 0x{:02X}, 0x{:02X}]", a, b, c, d),
            Self::BadPointerWidth( w ) => format!("bad pointer width flag. Expected 1 or 2, found {}", w),
        };

        f.write_str(&string)
    }
}

impl std::error::Error for ELFError {}
