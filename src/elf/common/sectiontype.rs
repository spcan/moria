//! Section Type as defined by the ELF format.



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SectionType {
    /// Null section type, used also for unknown.
    Null,

    /// Program data section.
    ProgramData,

    /// Symbol table section.
    SymbolTable,

    /// String table section.
    StringTable,

    /// Relocation entries with addends.
    RelocationA,

    /// Symbol hash table.
    Hash,

    /// Dynamic linking information.
    Dynamic,

    /// Notes.
    Notes,

    /// BSS | Program space with no data.
    Bss,

    /// Relocation entries with no addends.
    Relocation,

    /// Reserved.
    SharedLib,

    /// Dynamic linker symbol table.
    DynamicSymbol,

    /// Contructor array.
    Init,

    /// Desctructors array.
    Fini,

    /// Pre contructors array.
    PreInit,

    /// Section group.
    Group,

    /// Extended section indices.
    SymbolTableX,

    /// Number of defined types.
    Num,

    /// OS Specific.
    OperatingSystem(u32),
}

impl core::convert::From<u32> for SectionType {
    fn from(t: u32) -> SectionType {
        use SectionType::*;

        match t {
            0x00 => Null,
            0x01 => ProgramData,
            0x02 => SymbolTable,
            0x03 => StringTable,
            0x04 => RelocationA,
            0x05 => Hash,
            0x06 => Dynamic,
            0x07 => Notes,
            0x08 => Bss,
            0x09 => Relocation,
            0x0A => SharedLib,
            0x0B => DynamicSymbol,
            0x0E => Init,
            0x0F => Fini,
            0x10 => PreInit,
            0x11 => Group,
            0x12 => SymbolTableX,
            0x13 => Num,
            x => match x {
                0x60000000..=0xFFFFFFFF => OperatingSystem(x),
                _ => Null,
            },
        }
    }
}

impl core::fmt::Display for SectionType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use SectionType::*;

        let arg = match *self {
            Null          => String::from("Null/Unused/Unknown"),
            ProgramData   => String::from("Program data"),
            SymbolTable   => String::from("Symbol table"),
            StringTable   => String::from("String table"),
            RelocationA   => String::from("Relocation entries with addend"),
            Hash          => String::from("Symbol hash table"),
            Dynamic       => String::from("Dynamic linking information"),
            Notes         => String::from("Notes"),
            Bss           => String::from("BSS"),
            Relocation    => String::from("Relocation entries with no addends"),
            SharedLib     => String::from("RESERVED"),
            DynamicSymbol => String::from("Dynamic linker symbol table"),
            Init          => String::from("Array of contructors"),
            Fini          => String::from("Array of destructors"),
            PreInit       => String::from("Array of pre-contructors"),
            Group         => String::from("Section group"),
            SymbolTableX  => String::from("Extended section indices"),
            Num           => String::from("Number of defined types"),
            OperatingSystem(x) => format!("OS Specific ({:#X})", x),
        };

        write!(f, "{}", arg)
    }
}
