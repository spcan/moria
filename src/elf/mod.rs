//! ELF parser, inspector and editor.



pub mod common;
mod error;
mod file;
mod section;
mod symbol;
pub mod traits;



use common::{
    SectionType,
};

pub use error::{
    ELFError,
};

pub use file::{
    ELFHeader,
};

pub use section::{
    ELFSection,
};

pub use symbol::{
    ELFSymbol,
};

use std::{
    error::{
        Error,
    },

    ffi::{
        CStr,
    },

    fs::{
        File,
    },

    io::{
        Read,
    },

    path::{
        PathBuf,
    },
};



pub struct ELFContent {
    /// Raw ELF data.
    pub raw: Vec<u8>,

    /// File header of the ELF.
    pub header: Box<dyn traits::FileHeader>,

    /// List of all sections in this ELF file.
    pub sections: Vec<Box<dyn traits::SectionHeader>>,

    /// List of all symbols in this ELF file.
    pub symbols: Vec<Box<dyn traits::Symbol>>,
}

impl ELFContent {
    /// Parses the contents of a file and attempts to build an ELF object from them.
    pub fn parse(raw: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        use traits::{
            FileHeader, SectionHeader,
        };


        const EMPTY: [u8; 0] = [];

        // Check the magic.
        if raw[0..4] != [0x7F, 0x45, 0x4C, 0x46] {
            return Err( Box::new( ELFError::BadMagic( [raw[0], raw[1], raw[2], raw[3]] ) ) );
        }

        // Check the pointer width.
        match raw[4] {
            1 => {
                // Get the file header.
                let header = ELFHeader::<u32>::parse(&raw);

                // Get the sections.
                let (s, n, b) = (header.shtoffset(), header.shnum(), header.shsize());
                let e = s + (n * b);
                let mut sections = ELFSection::<u32>::all(&raw[s..e], &header);

                // Get the contents of the section header string section.
                let shstrtab = Self::sectiondata(&raw, &sections[header.shstrndx()]);

                // Get the names of the sections.
                Self::rename(shstrtab, &mut sections);

                // Get the contents of the string table.
                let findstrtab = sections.iter()
                    .find(|section| (section.stype() == SectionType::StringTable) && (section.name() == ".strtab"));

                let strtab = match findstrtab {
                    None => &EMPTY,
                    Some(section) => Self::sectiondata(&raw, section),
                };

                // Search for the symbol table.
                let mut symbols = match sections.iter().find(|s| s.stype() == SectionType::SymbolTable) {
                    None => Vec::new(),
                    Some(symtab) => {
                        use traits::Symbol;

                        // Get the symbol table.
                        let table = Self::sectiondata( &raw, symtab );

                        ELFSymbol::<u32>::all(table, header.endian())
                    },
                };

                // Get the names of the symbols.
                Self::rename(strtab, &mut symbols);

                Ok( ELFContent {
                    raw,
                    header: Box::new( header ),
                    sections,
                    symbols,
                })
            },

            2 => {
                // Get the file header.
                let header = ELFHeader::<u64>::parse(&raw);

                // Get the sections.
                let (s, n, b) = (header.shtoffset(), header.shnum(), header.shsize());
                let e = s + (n * b);
                let mut sections = ELFSection::<u64>::all(&raw[s..e], &header);

                // Get the contents of the section header string section.
                let shstrtab = Self::sectiondata(&raw, &sections[header.shstrndx()]);

                // Get the names of the sections.
                Self::rename(shstrtab, &mut sections);

                // Get the contents of the string table.
                let findstrtab = sections.iter()
                    .find(|section| (section.stype() == SectionType::StringTable) && (section.name() == ".strtab"));

                let strtab = match findstrtab {
                    None => &EMPTY,
                    Some(section) => Self::sectiondata(&raw, section),
                };

                // Search for the symbol table.
                let mut symbols = match sections.iter().find(|s| s.stype() == SectionType::SymbolTable) {
                    None => Vec::new(),
                    Some(symtab) => {
                        use traits::Symbol;

                        // Get the symbol table.
                        let table = Self::sectiondata( &raw, symtab );

                        ELFSymbol::<u64>::all(table, header.endian())
                    },
                };

                // Get the names of the symbols.
                Self::rename(strtab, &mut symbols);

                Ok( ELFContent {
                    raw,
                    header: Box::new( header ),
                    sections,
                    symbols,
                })
            },

            _ => Err( Box::new( ELFError::BadPointerWidth( raw[4] ) ) ),
        }
    }

    /// Access to the raw contents of a section.
    fn sectiondata<'a>(raw: &'a [u8], section: &Box<dyn traits::SectionHeader>) -> &'a [u8] {
        // Get the offset and size of the symbol section.
        let (o, s) = section.phys();

        &raw[o..o+s]
    }

    /// Names a list of items from the strings in the given string table.
    fn rename<R: traits::Rename + ?Sized>(strtab: &[u8], objects: &mut Vec<Box<R>>) {
        if strtab.len() == 0 { return; }

        for object in objects.iter_mut() {
            // Get the string offset.
            let offset = object.strndx();

            // Get the name.
            let name = match CStr::from_bytes_until_nul(&strtab[offset..]) {
                Err(_) => String::from("CORRUPTED"),

                Ok(cstr) => match cstr.to_owned().into_string() {
                    Err(_) => String::from("CORRUPTED"),

                    Ok(n) if n.len() == 0 => String::from("NULL"),

                    Ok(n) => n,
                },
            };

            object.setname( name );
        }
    }

    /// Creates a `gimli` DWARF reader that owns the data (is a copy) of the debug sections.
    pub fn dwarf(&self) -> Result<gimli::read::Dwarf<Vec<u8>>, gimli::Error> {
        // Create the load closure.
        let load = |id: gimli::SectionId| -> Result<Vec<u8>, gimli::Error> {
            match self.sections.iter().find(|section| section.name() == id.name() ) {
                Some(section) => Ok( Self::sectiondata(&self.raw, section).iter().map(|x| *x).collect() ),
                _ => Ok( Vec::new() ),
            }
        };

        // Load all the sections.
        gimli::read::Dwarf::load( load )
    }
}

impl core::convert::TryFrom<PathBuf> for ELFContent {
    type Error = Box<dyn Error>;

    fn try_from(path: PathBuf) -> Result<ELFContent, Box<dyn Error>> {
        // Open the file.
        match File::open(&path) {
            Err(e) => return Err( Box::new(e) ),
            Ok(mut f) => {
                // Read the contents of the file.
                let mut raw = Vec::new();

                match f.read_to_end(&mut raw) {
                    Err(e) => return Err( Box::new( e ) ),
                    _ => (),
                }

                ELFContent::parse(raw)
            },
        }
    }
}

impl core::convert::TryFrom<&PathBuf> for ELFContent {
    type Error = Box<dyn Error>;

    fn try_from(path: &PathBuf) -> Result<ELFContent, Box<dyn Error>> {
        // Open the file.
        match File::open(path) {
            Err(e) => return Err( Box::new(e) ),
            Ok(mut f) => {
                // Read the contents of the file.
                let mut raw = Vec::new();

                match f.read_to_end(&mut raw) {
                    Err(e) => return Err( Box::new( e ) ),
                    _ => (),
                }

                ELFContent::parse(raw)
            },
        }
    }
}

impl core::fmt::Display for ELFContent {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        // Build the output string.
        let mut string = String::new();

        // Write the name of the struct.
        string += &format!("ELF Content\n");

        // Write the ELF header information.
        string += &format!("{}", self.header);

        // Write all the sections.
        string += &self.sections.iter()
            .fold(String::new(), |f, s| f + &format!("{}", s));

        // Write all the symbols.
        string += &self.symbols.iter()
            .fold(String::new(), |f, s| f + &format!("{}", s));

        f.write_str(&string)
    }
}


#[cfg(all(test, feature="dev"))]
mod test {
    use super::*;

    #[test]
    fn displayELF() {
        use common::{
            SymbolType,
        };

        use traits::{
            Symbol,
        };

        // Current path.
        let here = std::env::current_dir().unwrap();
        here.join("test.elf");

        let elf = ELFContent::try_from(here.join("test.elf")).unwrap();

        //println!("{}", elf.header);

        //println!("{}", elf.sections.iter().fold(String::new(), |string, section| string + format!("{}", section)));

        //println!("{}", elf.symbols.iter().fold(String::new(), |string, symbol| string + format!("{}", symbol)));

        //println!("{}", elf.symbols.iter().filter(|s| s.stype() != SymbolType::Function).fold(String::new(), |string, symbol| string + &format!("{}", symbol)));

        // Get dwarf data.
        let owned = elf.dwarf().unwrap();

        let dwarf = {
            let endian = match elf.header.endian() {
                common::Endian::Little => gimli::RunTimeEndian::Little,
                common::Endian::Big => gimli::RunTimeEndian::Big,
            };

            owned.borrow(|section| gimli::EndianSlice::new( &section, endian ))
        };

        // Get the .debug_info section.
        let mut info = dwarf.debug_info.units();

        // Get the .debug_str section
        let mut debugstr = dwarf.debug_str;

        // Get the .debug_line section.
        let mut debugline = dwarf.debug_line;

        // Unit string buffer.
        let mut unitbuffer = Vec::new();

        let mut i = 0;

        while let Ok(Some(header)) = info.next() {
            // Get the unit.
            let unit = dwarf.unit(header).unwrap();

            if unit.low_pc == 0 {
                continue;
            }

            // Display the information.
            let header = {
                // Create the buffer string.
                let mut string = String::new();

                // Display the header information.
                match unit.header.offset() {
                    gimli::UnitSectionOffset::DebugInfoOffset(offset) => string += &format!("Compilation Unit ({}) @ offset 0x{:X}:\n", i, offset.0),
                    _ => string += &format!("Compilation Unit ({}) @ offset <unknown>:\n", i),
                }

                match unit.encoding().format {
                    gimli::Format::Dwarf32 => string += &format!("  Length:              0x{:X} (32-bit)\n", unit.header.unit_length()),
                    _ => string += &format!("  Length:              0x{:X} (64-bit)\n", unit.header.unit_length()),
                }

                string += &format!("  Version:             {}\n", unit.header.version());
                string += &format!("  Abbreviation offset: {:?}\n", unit.header.debug_abbrev_offset().0);
                string += &format!("  Pointer size:        {}\n", unit.header.address_size());

                string += &format!("  Base address:        {:#X}\n", unit.low_pc);

                string

                /*
                <c>   DW_AT_producer    : (indirect string, offset: 0x107): GNU C99 5.3.0 -m32 -mtune=generic -march=pentiumpro -g -std=gnu99 -ffreestanding
                <10>   DW_AT_language    : 12	(ANSI C99)
                <11>   DW_AT_name        : (indirect string, offset: 0x1b8): kernel/main.c
                <15>   DW_AT_comp_dir    : (indirect string, offset: 0x158): /usr/opsys
                <19>   DW_AT_low_pc      : 0x1002dd
                <1d>   DW_AT_high_pc     : 0x7f4
                <21>   DW_AT_stmt_list   : 0x0
                */
            };

            // Format the entries.
            let mut entries = String::new();

            // Get the cursor to the entries of the information.
            let mut cursor = unit.entries();

            // Enumeration tracker.
            let mut j = 0;

            while let Ok(Some(_)) = cursor.next_entry() {
                // Get the current entry.
                let entry = match cursor.current() {
                    Some(e) => e,
                    _ => continue,
                };

                // Create the buffer string.
                let mut string = String::new();

                match entry.tag() {
                    gimli::DW_TAG_compile_unit => {
                        // Add the header information.
                        string += &format!("<{:>3}> Abbrev Number: {} (DW_TAG_compile_unit)\n", j, entry.code());

                        // Enumerate the attributes.
                        let mut attrs = entry.attrs();
                        let mut k = 0;

                        // Check for the debug line offset.
                        let mut debuglineoff = gimli::DebugLineOffset(0);
                        let mut compdir = None;

                        while let Ok(Some(attr)) = attrs.next() {
                            string += &format!("     <{:>3}> {:<24} {}\n", k, attr.name(), attrvalue(attr.value(), &debugstr));

                            match attr.value() {
                                gimli::AttributeValue::DebugLineRef( r ) => debuglineoff = r,
                                _ => (),
                            }

                            if attr.name().static_string() == Some("DW_AT_comp_dir") {
                                if let gimli::AttributeValue::DebugStrRef(o) = attr.value() {
                                    match debugstr.get_str(o) {
                                        Ok(s) => compdir = Some(s),
                                        _ => (),
                                    }
                                }
                            }

                            k += 1;
                        }
            
                        // Attempt to get the associated files.
                        match debugline.program( debuglineoff, unit.header.address_size(), compdir, None ) {
                            Ok(incomplete) => {
                                for (f, file) in incomplete.header().file_names().iter().enumerate() {
                                    match file.path_name() {
                                        gimli::AttributeValue::DebugStrRef(o) => match debugstr.get_str(o) {
                                            Ok(s) => match String::from_utf8(s.slice().to_vec()) {
                                                Ok(fp) => string += &format!("     [{}] {:?}\n", f, fp),
                                                _ => string += &format!("     [{}] -Non UTF8-\n", f),
                                            },
                                            _ => string += &format!("     [{}] -Non UTF8-\n", f),
                                        },

                                        gimli::AttributeValue::String(s) => match String::from_utf8(s.slice().to_vec()) {
                                            Ok(fp) => string += &format!("     [{}] {:?}\n", f, fp),
                                            _ => string += &format!("     [{}] -Non UTF8-\n", f),
                                        },

                                        path => string += &format!("     [{}] {:?}\n", f, path),
                                    }
                                }
                            },
                            _ => (),
                        }
                    },

                    gimli::DW_TAG_inlined_subroutine => {
                        // Add the header information.
                        string += &format!("<{:>3}> Abbrev Number: {} (DW_TAG_inlined_subroutine)\n", j, entry.code());

                        // Enumerate the attributes.
                        let mut attrs = entry.attrs();
                        let mut k = 0;

                        while let Ok(Some(attr)) = attrs.next() {
                            string += &format!("     <{:>3}> {:<24} {}\n", k, attr.name(), attrvalue(attr.value(), &debugstr));

                            k += 1;
                        }
                    },

                    gimli::DW_TAG_subprogram => {
                        // Add the header information.
                        string += &format!("<{:>3}> Abbrev Number: {} (DW_TAG_subprogram)\n", j, entry.code());

                        // Enumerate the attributes.
                        let mut attrs = entry.attrs();
                        let mut k = 0;

                        while let Ok(Some(attr)) = attrs.next() {
                            string += &format!("     <{:>3}> {:<24} {}\n", k, attr.name(), attrvalue(attr.value(), &debugstr));

                            k += 1;
                        }
                    },

                    gimli::DW_TAG_variable | gimli::DW_TAG_formal_parameter | gimli::DW_TAG_constant => {
                        // Add the header information.
                        match entry.tag() {
                            gimli::DW_TAG_variable => string += &format!("<{:>3}> Abbrev Number: {} (DW_TAG_variable)\n", j, entry.code()),
                            gimli::DW_TAG_formal_parameter => string += &format!("<{:>3}> Abbrev Number: {} (DW_TAG_formal_parameter)\n", j, entry.code()),
                            gimli::DW_TAG_constant => string += &format!("<{:>3}> Abbrev Number: {} (DW_TAG_constant)\n", j, entry.code()),

                            _ => unreachable!(),
                        }

                        // Enumerate the attributes.
                        let mut attrs = entry.attrs();
                        let mut k = 0;

                        while let Ok(Some(attr)) = attrs.next() {
                            string += &format!("     <{:>3}> {:<24} {}\n", k, attr.name(), attrvalue(attr.value(), &debugstr));

                            k += 1;
                        }
                    },

                    gimli::DW_TAG_namespace => {
                        // Add the header information.
                        string += &format!("<{:>3}> Abbrev Number: {} (DW_TAG_namespace)\n", j, entry.code());

                        // Enumerate the attributes.
                        let mut attrs = entry.attrs();
                        let mut k = 0;

                        while let Ok(Some(attr)) = attrs.next() {
                            string += &format!("     <{:>3}> {:<24} {}\n", k, attr.name(), attrvalue(attr.value(), &debugstr));

                            k += 1;
                        }
                    },

                    tag => string += &format!("<{:>3}> {:?}\n", j, tag.static_string()),
                }

                entries += &format!("{}\n", string, );

                j += 1;
            }

            unitbuffer.push( format!("\n{}\n{}", header, entries) );
            //unitbuffer.push( format!("\n{}\n", header) );

            i += 1;
        }

        let l = unitbuffer.len() - 1;

        for i in 0..unitbuffer.len() {
            println!("{}", unitbuffer[i]);
        }

        assert!(false)
    }

    fn attrvalue<R: gimli::Reader>(attr: gimli::AttributeValue<R>, debugstr: &gimli::read::DebugStr<R>) -> String {
        match attr {
            gimli::AttributeValue::Addr(x) => format!("<Addr> {:#X}", x),

            gimli::AttributeValue::Data1(x) => format!("<Byte> {:#X}", x),
            gimli::AttributeValue::Data2(x) => format!("<Half> {:#X}", x),
            gimli::AttributeValue::Data4(x) => format!("<Word> {:#X}", x),
            gimli::AttributeValue::Data8(x) => format!("<Long> {:#X}", x),

            gimli::AttributeValue::Sdata(x) => format!("<SDat> {}", x),
            gimli::AttributeValue::Udata(x) => format!("<Udat> {}", x),

            gimli::AttributeValue::Flag(x) => format!("<Flag> {}", x),

            gimli::AttributeValue::Language(l) => format!("<Lang> {}", l),

            gimli::AttributeValue::String(s) => format!("<Str > {:?}", s),

            gimli::AttributeValue::DebugStrRef(o) => match debugstr.get_str(o) {
                Ok(s) => match String::from_utf8(s.to_slice().unwrap().to_vec()) {
                    Ok(s) => format!("<DStr> {}", s),
                    _ => String::from("<DStr> Unable to read UTF-8"),
                },
                _ => String::from("<DStr> Unable to read"),
            },

            v => format!("<????> {:?}", v),
        }
    }
}
