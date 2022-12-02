//! Architecture Flags as defined in the ELF format.



pub struct ArchFlags(pub(super) usize);

impl core::convert::From<u32> for ArchFlags {
    #[inline(always)]
    fn from(x: u32) -> Self {
        Self(x as usize)
    }
}

impl core::fmt::Display for ArchFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "0x{:08X}", self.0)
    }
}
