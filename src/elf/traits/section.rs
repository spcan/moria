//! Common trait for section headers of all architectural types.



use crate::{
    elf::{
        common::{
            Endian, SectionType,
        },
    },
};

use super::{
    FileHeader, Rename,
};



pub trait SectionHeader: core::fmt::Display + Rename {
    /// Parses all section headers with the given headers configuration.
    fn all(table: &[u8], header: &dyn FileHeader) -> Vec<Box<dyn SectionHeader>> where Self: Sized + 'static {
        // Get the size of each header and the number of headers.
        let endian = header.endian();
        let size = header.shsize();
        let num = header.shnum();

        // List of all sections.
        let mut out: Vec<Box<dyn SectionHeader>> = Vec::with_capacity(num);

        for (i, chunk) in (&table[0..size*num]).chunks(size).enumerate() {
            out.push( Box::new( Self::parse(chunk, endian, i) ) )
        }

        out
    }

    /// Parses an ELF Section header.
    fn parse(chunk: &[u8], endian: Endian, i: usize) -> Self where Self: Sized;

    /// Returns a reference to the name of the section.
    fn name(&self) -> &str;

    /// Returns the physical address and size.
    fn phys(&self) -> (usize, usize);

    /// Returns the virtual address and size.
    fn virt(&self) -> (usize, usize);

    fn stype(&self) -> SectionType;
}

