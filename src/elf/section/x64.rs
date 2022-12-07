//! 32-bit implementation of an ELF Section Header.



use byteorder::{
    BigEndian, LittleEndian,
    ByteOrder,
};

use crate::{
    elf::{
        common::{
            read::{
                big32, little32,
            },

            Endian, SectionFlags, SectionType,
        },

        traits::{
            SectionHeader,
        },
    },
};

use super::ELFSection;



impl ELFSection<u64> {
    /// Internal read function.
    fn read<B: ByteOrder>(data: &[u8]) -> u64 {
        B::read_u64( data )
    }
}

impl SectionHeader for ELFSection<u64> {
    fn parse(chunk: &[u8], endian: Endian, index: usize) -> Self where Self: Sized {
        // Get the read fucntions.
        let (read32, read): (fn(&[u8]) -> u32, fn(&[u8]) -> u64) = match endian {
            Endian::Little => (little32, Self::read::<LittleEndian>),
            Endian::Big => (big32, Self::read::<BigEndian>),
        };

        let (stype, flags, [vaddr, offset, size, align, entrysize], [strndx, link, info]) = Self::create(chunk, read32, read);

        // Build the section.
        Self {
            index,
            strndx: strndx as usize,
            name: String::new(),

            stype, flags: SectionFlags(flags as u64),

            vaddr, offset, size,
            link, info, align,
            entrysize,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn phys(&self) -> (usize, usize) {
        (self.offset as usize, self.size as usize)
    }

    fn virt(&self) -> (usize, usize) {
        (self.vaddr as usize, self.size as usize)
    }

    fn stype(&self) -> SectionType {
        self.stype
    }
}

impl core::fmt::Display for ELFSection<u64> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        // Build the output string.
        let mut string = String::new();

        // Write the name of the struct.
        string += &format!("ELF Section Header\n");

        // Add section name.
        string += "|- Section ID\n";
        string += &format!("|  |- Section index: {}\n", self.index);
        string += &format!("|  |- Name offset  : {}\n", self.strndx);
        string += &format!("|  |- Name         : {}\n", self.name);

        // Add section information.
        string += &format!("|- Section type : {}\n", self.stype);
        string += &format!("|- Section flags: {}\n", self.flags);

        if self.info > 0 {
            string += &format!("|- Section info : {:#X}\n", self.info);
        }

        // Add other information.
        if self.link > 0 {
            string += &format!("|- Linked with section: {}\n", self.link);
        }

        if self.entrysize > 0 {
            string += &format!("|- Internal entry size: {}\n", self.entrysize);
        }

        // Add address information.
        string += "|- Section Addressing Information\n";
        string += &format!("   |- Virtual Address: {:#016X}\n", self.vaddr);
        string += &format!("   |- File offset    : {:#016X}\n", self.offset);
        string += &format!("   |- Size           : {} MiB | {} kiB \n", self.size as f64 / (1024.0 * 1024.0), self.size as f64 / 1024.0);
        string += &format!("   |- Alignment      : {} bytes\n\n", self.align);

        f.write_str(&string)
    }
}
