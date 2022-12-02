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

impl ELFHeader<u32> {
    /// Internal read function.
    fn read<B: ByteOrder>(data: &[u8]) -> u32 {
        B::read_u32( data )
    }
}

impl ELFHeader<u64> {
    /// Internal read function.
    fn read<B: ByteOrder>(data: &[u8]) -> u64 {
        B::read_u64( data )
    }
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

impl FileHeader for ELFHeader<u64> {
    fn parse(header: &[u8]) -> Self {
        // Check magic number.
        if header[0x00..0x04] != [0x7F, 0x45, 0x4C, 0x46] {
            todo!("Gracefully return error")
        }

        // Check ELF version.
        if header[0x06] != 1 {
            todo!("Gracefully return error")
        }

        // Get endianness.
        let (endian, read): (Endian, fn(&[u8]) -> u64) = match header[0x05] {
            1 => (Endian::Little, Self::read::<LittleEndian>),
            2 => (Endian::Big   , Self::read::<BigEndian>   ),

            _ => todo!("Gracefully return error"),
        };

        Self::create(header, endian, read)
    }

    fn endian(&self) -> Endian {
        self.endian
    }

    fn phtoffset(&self) -> usize {
        self.offset.0 as usize
    }

    fn shtoffset(&self) -> usize {
        self.offset.1 as usize
    }

    fn phsize(&self) -> usize {
        self.entrysize.0 as usize
    }

    fn shsize(&self) -> usize {
        self.entrysize.1 as usize
    }

    fn phnum(&self) -> usize {
        self.num.0 as usize
    }

    fn shnum(&self) -> usize {
        self.num.1 as usize
    }

    fn shstrndx(&self) -> usize {
        self.shstrndx as usize
    }
}

impl core::fmt::Display for ELFHeader<u64> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        // Build the output string.
        let mut string = String::new();

        // Write the name of the struct.
        string += &format!("ELF File Header\n");

        // Add file information.
        string += &format!("|- File type: {}\n", self.filetype);

        // Add target information.
        string += "|- Target\n";
        string += &format!("|  |- Endianness      : {}\n", self.endian);
        string += &format!("|  |- Operating System: {}\n", self.os);
        string += &format!("|  |- Instruction Set : {}\n", self.isa);
        string += &format!("|  |- Flags           : {}\n", self.flags);

        // Add entry point.
        string += &format!("|- Entry point: 0x{:016X}\n", self.entry);

        // Add sections information.
        string += "|- Programs Table\n";
        string += &format!("|  |- File offset      : {}\n", self.offset.0);
        string += &format!("|  |- Entry Size       : {}\n", self.entrysize.0);
        string += &format!("|  |- Number of Entries: {}\n", self.num.0);

        string += "|- Sections Table\n";
        string += &format!("   |- File offset         : {}\n", self.offset.1);
        string += &format!("   |- Entry Size          : {}\n", self.entrysize.1);
        string += &format!("   |- Number of Entries   : {}\n", self.num.1);
        string += &format!("   |- String Section Index: {}\n", self.shstrndx);

        f.write_str(&string)
    }
}

impl FileHeader for ELFHeader<u32> {
    fn parse(header: &[u8]) -> Self {
        // Check magic number.
        if header[0x00..0x04] != [0x7F, 0x45, 0x4C, 0x46] {
            todo!("Gracefully return error")
        }

        // Check ELF version.
        if header[0x06] != 1 {
            todo!("Gracefully return error")
        }

        // Get endianness.
        let (endian, read): (Endian, fn(&[u8]) -> u32) = match header[0x05] {
            1 => (Endian::Little, Self::read::<LittleEndian>),
            2 => (Endian::Big   , Self::read::<BigEndian>   ),

            _ => todo!("Gracefully return error"),
        };

        Self::create(header, endian, read)
    }

    fn endian(&self) -> Endian {
        self.endian
    }

    fn phtoffset(&self) -> usize {
        self.offset.0 as usize
    }

    fn shtoffset(&self) -> usize {
        self.offset.1 as usize
    }

    fn phsize(&self) -> usize {
        self.entrysize.0 as usize
    }

    fn shsize(&self) -> usize {
        self.entrysize.1 as usize
    }

    fn phnum(&self) -> usize {
        self.num.0 as usize
    }

    fn shnum(&self) -> usize {
        self.num.1 as usize
    }

    fn shstrndx(&self) -> usize {
        self.shstrndx as usize
    }
}

impl core::fmt::Display for ELFHeader<u32> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        // Build the output string.
        let mut string = String::new();

        // Write the name of the struct.
        string += &format!("ELF File Header\n");

        // Add file information.
        string += &format!("|- File type: {}\n", self.filetype);

        // Add target information.
        string += "|- Target\n";
        string += &format!("|  |- Endianness      : {}\n", self.endian);
        string += &format!("|  |- Operating System: {}\n", self.os);
        string += &format!("|  |- Instruction Set : {}\n", self.isa);
        string += &format!("|  |- Flags           : {}\n", self.flags);

        // Add entry point.
        string += &format!("|- Entry point: 0x{:08X}\n", self.entry);

        // Add sections information.
        string += "|- Program Table\n";
        string += &format!("|  |- File offset      : {:#X}\n", self.offset.0);
        string += &format!("|  |- Entry Size       : {}\n", self.entrysize.0);
        string += &format!("|  |- Number of Entries: {}\n", self.num.0);

        string += "|- Section Table\n";
        string += &format!("   |- File offset         : {:#X}\n", self.offset.1);
        string += &format!("   |- Entry Size          : {}\n", self.entrysize.1);
        string += &format!("   |- Number of Entries   : {}\n", self.num.1);
        string += &format!("   |- String Section Index: {}\n\n", self.shstrndx);

        f.write_str(&string)
    }
}
