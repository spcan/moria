//! ELF Symbols parsing and interaction.



mod x32;
mod x64;



use crate::{
    elf::{
        common::{
            SymbolBind, SymbolType,
        },

        traits::{
            Rename
        },
    },
};



pub struct ELFSymbol<T> {
    /// Index of the name.
    strndx: usize,

    /// Name of the symbol.
    name: String,

    /// Type of the symbol
    stype: SymbolType,

    /// Binding of the symbol.
    binding: SymbolBind,

    /// Visibility of the symbol.
    visibility: u8,

    /// Section index relativity.
    relativity: u16,

    /// Value of the symbol.
    value: T,

    /// Size of the symbol.
    size: T,
}

impl<T> Rename for ELFSymbol<T> {
    fn strndx(&self) -> usize {
        self.strndx
    }

    fn setname(&mut self, name: String) {
        self.name = name;
    }
}
