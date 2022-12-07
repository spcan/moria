//! Common traits for ELF objects.



mod file;
mod rename;
mod section;
mod symbol;



pub use file::FileHeader;
pub use rename::Rename;
pub use section::SectionHeader;
pub use symbol::Symbol;
