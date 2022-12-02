//! Symbol binding.


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolBind {
	/// Local bind.
	Local,

	/// Global bind.
	Global,

	/// Weak bind.
	Weak,

	/// Processor specific.
	Processor(u8),

    None,
}

impl core::convert::From<u8> for SymbolBind {
    fn from(u: u8) -> Self {
        match u {
        	0 => SymbolBind::Local,
            1 => SymbolBind::Global,
            2 => SymbolBind::Weak,

            13..=15 => SymbolBind::Processor(u),

            _ => SymbolBind::None,
        }
    }
}

impl core::fmt::Display for SymbolBind {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let s = match *self {
            SymbolBind::Local   => String::from("Local"),
            SymbolBind::Global => String::from("Global"),
            SymbolBind::Weak  => String::from("Weak"),

            SymbolBind::Processor(u) => format!("Processor {}", u),

            SymbolBind::None => String::from("No binding"),
        };

        write!(f, "{}", s)
    }
}
