//! ELF Header abstraction.



use byteorder::{
    BigEndian, LittleEndian,
    ByteOrder,
};

use crate::{
    elf::{
        common::{
            read::{
                big16, little16,
                big32, little32,
            },

            ArchFlags, Endian, FileType,
            InstructionSet, OperatingSystem,
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



pub struct ELFHeader<T> {
    /// Endianness of the target architecture.
    endian: Endian,

    /// Target OS ABI.
    os: OperatingSystem,

    /// Object File Type.
    filetype: FileType,

    /// Target Instruction Set Architecture.
    isa: InstructionSet,

    /// Architectural flags.
    flags: ArchFlags,

    /// Entry point of the program.
    entry: T,

    /// File offsets to the Program and Section Header Table.
    offset: (T, T),

    /// Entry size of a Program and Section Header.
    entrysize: (u16, u16),

    /// Number of Program and Section Header entries.
    num: (u16, u16),

    /// Index of the Section Header of the string table.
    shstrndx: u16,
}


impl<T> ELFHeader<T> {
    fn create(header: &[u8], endian: Endian, read: fn(&[u8]) -> T) -> Self {
        // Get the read functions.
        let (read16, read32): (fn(&[u8]) -> u16, fn(&[u8]) -> u32) = match endian {
            Endian::Little => (little16, little32),
            Endian::Big => (big16, big32),
        };

        // Get OS ABI.
        let os = OperatingSystem::from((header[0x07], header[0x08]));

        // Get ELF File Type.
        let filetype = FileType::from( read16( &header[0x10..0x12] ) );

        // Get ISA.
        let isa = InstructionSet::from( read16( &header[0x12..0x14] ) );

        // Start specialized section of the file header.
        let mut i = 0x18;
        let s = core::mem::size_of::<T>();

        // Get the entry point.
        let entry = read( &header[i..i+s] );

        i += s;

        // Get the Program and Section Header Tables offsets.
        let offset = (
            read( &header[i+0..i+( s )] ),
            read( &header[i+s..i+(2*s)] ),
        );

        i += 2 * s;

        // Continue with common section of the file header.

        // Get the Architecture Flags.
        let flags = ArchFlags::from( read32( &header[i..i+4] ) );
        i += 4;
        i += 2;

        // Get the Program and Section Header size.
        let entrysize = (
            read16( &header[i+0..i+2] ),
            read16( &header[i+4..i+6] ),
        );

        i += 2;

        // Get the number of Program and Section Headers.
        let num = (
            read16( &header[i+0..i+2] ),
            read16( &header[i+4..i+6] ),
        );

        i+= 6;

        // Get the index of the string section.
        let shstrndx = read16( &header[i..i+2] );

        Self {
            endian,
            os,
            filetype,
            isa,
            flags,
            entry,
            offset,
            entrysize,
            num,
            shstrndx,
        }
    }
}
