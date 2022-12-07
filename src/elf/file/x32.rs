//! 32-bit implementation of an ELF Header.



use byteorder::{
    BigEndian, LittleEndian,
    ByteOrder,
};

use crate::{
    elf::{
        common::{
            Endian,
        },

        traits::{
            FileHeader,
        },
    },
};

use super::ELFHeader;



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
impl ELFHeader<u32> {
    /// Internal read function.
    fn read<B: ByteOrder>(data: &[u8]) -> u32 {
        B::read_u32( data )
    }
}
