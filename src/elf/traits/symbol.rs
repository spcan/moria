//! Common trait for all implementations of an ELF symbol.


use crate::{
    elf::{
        common::{
            Endian, SymbolType
        },
    },
};


pub trait Symbol: core::fmt::Display + super::Rename {
    /// Parses a list of Symbols from a given symbol table.
    fn all(table: &[u8], endian: Endian) -> Vec<Box<dyn Symbol>> where Self: Sized + 'static;

    /// Parses a single Symbol.
    fn parse(chunk: &[u8], endian: Endian) -> Self where Self: Sized;

    /// Returns the type of this symbol.
    fn stype(&self) -> SymbolType;
}
