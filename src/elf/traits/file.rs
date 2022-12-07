//! Common trait for file headers of all architectural types.



use crate::{
    elf::{
        common::{
            Endian,
        },
    },
};



pub trait FileHeader: core::fmt::Display {
    /// Parses an ELF header.
    fn parse(header: &[u8]) -> Self where Self: Sized;

    /// Returns the endianness of the target architecture.
    fn endian(&self) -> Endian;

    /// Returns the file offset into the Program Header Table.
    fn phtoffset(&self) -> usize;

    /// Returns the file offset into the Section Header Table.
    fn shtoffset(&self) -> usize;

    /// Returns the size of the Program Header.
    fn phsize(&self) -> usize;

    /// Returns the size of the Section Header.
    fn shsize(&self) -> usize;

    /// Returns the number of Program Headers.
    fn phnum(&self) -> usize;

    /// Returns the number of Section Headers.
    fn shnum(&self) -> usize;

    /// Returns the index of the String Section.
    fn shstrndx(&self) -> usize;
}
