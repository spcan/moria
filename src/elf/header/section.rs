//! ELF Section Header abstraction.



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

            Endian, SectionType, SectionFlags,
        },

        traits::{
            Rename,
        },
    },
};



pub trait SectionHeader: core::fmt::Display + Rename {
    /// Parses all section headers with the given headers configuration.
    fn all(table: &[u8], header: &dyn super::FileHeader) -> Vec<Box<dyn SectionHeader>> where Self: Sized + 'static {
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

pub struct ELFSection<T> {
    /// Section index.
    index: usize,

    /// Offset into the .shstrtab section that represents the name of this section.
    strndx: usize,

    /// Name of this section.
    name: String,

    /// Section type.
    stype: SectionType,

    /// Section flags.
    flags: SectionFlags,

    /// Virtual address of the section in memory.
    vaddr: T,

    /// Offset of the section in the file image.
    offset: T,

    /// Size of the section in bytes.
    size: T,

    /// Contains the section index of an associated section.
    link: u32,

    /// Information about the section.
    info: u32,

    /// Alignment of the section.
    align: T,

    /// The size of each entry for table sections.
    entrysize: T,
}

impl<T> Rename for ELFSection<T> {
    fn strndx(&self) -> usize {
        self.strndx
    }

    fn setname(&mut self, name: String) {
        self.name = name;
    }
}

impl ELFSection<u32> {
    /// Internal read function.
    fn read<B: ByteOrder>(data: &[u8]) -> u32 {
        B::read_u32( data )
    }
}

impl ELFSection<u64> {
    /// Internal read function.
    fn read<B: ByteOrder>(data: &[u8]) -> u64 {
        B::read_u64( data )
    }
}

impl<T> ELFSection<T> {
    /// Internal parse function.
    fn create(chunk: &[u8], read32: fn(&[u8]) -> u32, read: fn(&[u8]) -> T) -> (SectionType, T, [T; 5], [u32; 3]) {
        // Get the index to the name string.
        let strndx = read32(&chunk[0x00..0x04]);

        // Get the section type.
        let stype = SectionType::from( read32( &chunk[0x04..0x08] ) );

        // Start dynamic section.
        let mut i = 0x08;
        let s = core::mem::size_of::<T>();

        // Get the flags.
        let flags = read( &chunk[i..i+s] );
        i += s;

        // Get the virtual address.
        let vaddr = read( &chunk[i..i+s] );
        i += s;

        // Get the image offset.
        let offset = read( &chunk[i..i+s] );
        i += s;

        // Get the section size.
        let size = read( &chunk[i..i+s] );
        i += s;

        // Get the link index.
        let link = read32( &chunk[i..i+4] );
        i += 4;

        // Get the section information.
        let info = read32( &chunk[i..i+4] );
        i += 4;

        // Get the address alignment.
        let align = read( &chunk[i..i+s] );
        i += s;

        // Get the entry size.
        let entrysize = read( &chunk[i..i+s] );

        (
            stype, flags,
            [vaddr, offset, size, align, entrysize],
            [strndx, link, info],
        )
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

impl SectionHeader for ELFSection<u32> {
    fn parse(chunk: &[u8], endian: Endian, index: usize) -> Self where Self: Sized {
        // Get the read fucntions.
        let (read32, read): (fn(&[u8]) -> u32, fn(&[u8]) -> u32) = match endian {
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

impl core::fmt::Display for ELFSection<u32> {
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
        string += &format!("   |- Virtual Address: 0x{:08X}\n", self.vaddr);
        string += &format!("   |- File offset    : 0x{:08X}\n", self.offset);
        string += &format!("   |- Size           : {:.2} MiB | {:.2} kiB | {} B \n", self.size as f64 / (1024.0 * 1024.0), self.size as f64 / 1024.0, self.size);
        string += &format!("   |- Alignment      : {} bytes\n\n", self.align);

        f.write_str(&string)
    }
}
