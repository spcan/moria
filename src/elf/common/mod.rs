//! Common structures, objects and abstractions of the ELF format.



pub(crate) mod read;

mod abi;
mod endian;
mod filetype;
mod flags;
mod isa;
mod sectionflags;
mod sectiontype;
mod symbolbind;
mod symboltype;




pub use abi::OperatingSystem;
pub use endian::Endian;
pub use filetype::FileType;
pub use flags::ArchFlags;
pub use isa::InstructionSet;
pub use sectionflags::SectionFlags;
pub use sectiontype::SectionType;
pub use symbolbind::SymbolBind;
pub use symboltype::SymbolType;
