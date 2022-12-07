//! ELF Section Header parsing and interaction.



mod x32;
mod x64;



use crate::{
    elf::{
        common::{
            SectionType, SectionFlags,
        },


        traits::{
            Rename,
        },
    },
};



/// Contains an ELF Section Header.
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
