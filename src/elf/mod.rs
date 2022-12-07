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

        println!("{}", elf.symbols.iter().filter(|s| s.stype() != SymbolType::Function).fold(String::new(), |string, symbol| string + &format!("{}", symbol)));

        assert!(false)
    }
}
