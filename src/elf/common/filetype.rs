//! ELF File Type.



#![allow(dead_code)]



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileType {
    /// Unknown / Unspecified file type.
    None,

    /// reolcatable file.
    Relocatable,

    /// Executable file.
    Executable,

    /// Dynamic file.
    Dynamic,

    /// Core file.
    Core,

    /// Operating System specific file.
    OperatingSystem(u16),

    /// Processor Specific file.
    Processor(u16),
}

impl core::convert::From<u16> for FileType {
    fn from(d: u16) -> FileType {
        match d {
            0x01 => FileType::Relocatable,
            0x02 => FileType::Executable,
            0x03 => FileType::Dynamic,
            0x04 => FileType::Core,

            0xFE00..=0xFEFF => FileType::OperatingSystem(d),

            0xFF00..=0xFFFF => FileType::Processor(d),

            _ => FileType::None,
        }
    }
}

impl core::fmt::Display for FileType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use FileType::*;

        let arg = match *self {
            None => String::from("Unknown ELF file type"),
            Relocatable => String::from("Relocatable file"),
            Executable => String::from("Executable file"),
            Dynamic => String::from("Dynamic linked file"),
            Core => String::from("Core file"),

            OperatingSystem(d) => format!("OS Specific file ({:#X})", d),

            Processor(d) => format!("Processor Specific file ({:#X})", d),
        };

        write!(f, "{}", arg)
    }
}
