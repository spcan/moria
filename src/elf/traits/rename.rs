//! Common trait for objects which names are inside a string table.



pub trait Rename {
    /// Returns the offset of the start of the `CStr` inside the string table.
    fn strndx(&self) -> usize;

    /// Sets the name of the ELF object.
    fn setname(&mut self, name: String);
}
