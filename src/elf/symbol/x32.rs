//! 32-bit implementation of an ELF symbol.



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

            Endian, SymbolBind, SymbolType,
        },

        traits::{
            Symbol
        },
    },
};

use super::ELFSymbol;


impl ELFSymbol<u32> {
    /// Size of a 32-bit symbol.
    const SIZE: usize = 16;

    /// Internal read function.
    fn read<B: ByteOrder>(data: &[u8]) -> u32 {
        B::read_u32( data )
    }
}

impl Symbol for ELFSymbol<u32> {
    fn all(table: &[u8], endian: Endian) -> Vec<Box<dyn Symbol>> where Self: Sized + 'static {
        // List of all sections.
        let mut out: Vec<Box<dyn Symbol>> = Vec::new();

        for chunk in table.chunks(Self::SIZE) {
            out.push( Box::new( Self::parse(chunk, endian) ) )
        }

        out
    }

    fn parse(chunk: &[u8], endian: Endian) -> Self {
        // Get the read function.
        let (read16, read32, read): (fn(&[u8]) -> u16, fn(&[u8]) -> u32, fn(&[u8]) -> u32) = match endian {
            Endian::Little => (little16, little32, Self::read::<LittleEndian>),
            Endian::Big    => (big16   , big32   , Self::read::<BigEndian>   ),
        };

        // Get the name index.
        let strndx = read32( &chunk[0..4] ) as usize;

        // Start dynamic section.
        let mut i = 4;

        // Get value of the symbol.
        let value = read( &chunk[i..i+4] );
        i += 4;

        // Get size of the symbol.
        let size = read( &chunk[i..i+4] );
        i += 4;

        // Get the type and binding.
        let (stype, binding) = {
            // Get the byte.
            let byte = chunk[i];
            i += 1;

            // Get the symbol type.
            let stype = SymbolType::from( byte & 0xF );

            // Get the binding.
            let binding = SymbolBind::from( byte >> 4 );

            (stype, binding)
        };

        // Get the visibility.
        //let visibility = SymbolVisibility::from( chunk[5] );
        let visibility = chunk[i];
        i += 1;

        // Get the relativity.
        let relativity = read16( &chunk[i..i+2] );

        ELFSymbol {
            strndx,
            name: String::new(),
            stype,
            binding,
            visibility,
            relativity,
            value,
            size,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn address(&self) -> usize {
        self.value as usize
    }

    fn size(&self) -> usize {
        self.size as usize
    }

    fn stype(&self) -> SymbolType {
        self.stype
    }
}

impl core::fmt::Display for ELFSymbol<u32> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        // Indicate object type.
        let mut string = String::from("ELF Symbol\n");

        // Add symbol identification.
        string += "|- Symbol ID\n";
        string += &format!("|  |- Name offset  : {}\n", self.strndx);
        string += &format!("|  |- Name         : {}\n", self.name);

        // Add symbol information.
        string += "|- Symbol Information\n";
        string += &format!("|  |- Type      : {}\n", self.stype);
        string += &format!("|  |- Binding   : {}\n", self.binding);
        string += &format!("|  |- Visibility: {}\n", self.visibility);

        if self.relativity != 0 {
            string += &format!("|- Related section index: {} \n", self.relativity)
        }

        // Add address information.
        string += "|- Symbol value\n";

        match self.stype {
            SymbolType::Function => {
                string += &format!("   |- Address: 0x{:08X}\n", self.value);
                string += &format!("   |- Size   : {:.2} MiB | {:.2} kiB | {} B \n", self.size as f64 / (1024.0 * 1024.0), self.size as f64 / 1024.0, self.size);
            },
            _ => {
                string += &format!("   |- Value: 0x{:08X}\n", self.value);
                string += &format!("   |- Size : {:.2} MiB | {:.2} kiB | {} B \n", self.size as f64 / (1024.0 * 1024.0), self.size as f64 / 1024.0, self.size);
            },
        }

        f.write_str(&string)

    }
}