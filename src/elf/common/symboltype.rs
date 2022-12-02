//! All possible types of symbols.


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
    /// No type / Unknown type.
    None,

    /// Object.
    Object,

    /// Function.
    Function,

    /// Section.
    Section,

    /// File.
    File,

    /// Processor specific.
    Processor(u8),
}


impl core::convert::From<u8> for SymbolType {
    fn from(u: u8) -> Self {
        match u {
            1 => SymbolType::Object,
            2 => SymbolType::Function,
            3 => SymbolType::Section,
            4 => SymbolType::File,

            13..=15 => SymbolType::Processor(u),

            _ => SymbolType::None,
        }
    }
}

impl core::fmt::Display for SymbolType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let s = match *self {
            SymbolType::Object   => String::from("Object"),
            SymbolType::Function => String::from("Function"),
            SymbolType::Section  => String::from("Section"),
            SymbolType::File     => String::from("File"),

            SymbolType::Processor(u) => format!("Processor {}", u),

            SymbolType::None => String::from("No type"),
        };

        write!(f, "{}", s)
    }
}
