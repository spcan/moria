//! Section Header flags.





pub struct SectionFlags(pub(crate) u64);

impl SectionFlags {
    /// Returns `true` if this section is writable.
    pub fn write(&self) -> bool {
        (self.0 & 0x1) != 0
    }

    /// Returns `true` if this section occupies memory during execution.
    pub fn alloc(&self) -> bool {
        (self.0 & 0x2) != 0
    }

    /// Returns `true` if this section contains code.
    pub fn exec(&self) -> bool {
        (self.0 & 0x4) != 0
    }

    /// Returns `true` if this section might be merged.
    pub fn merge(&self) -> bool {
        (self.0 & 0x10) != 0
    }

    /// Returns `true` if this section contains NULL terminated strings.
    pub fn strings(&self) -> bool {
        (self.0 & 0x20) != 0
    }

    /// Returns `true` if sh_info contains SHT index..
    pub fn infolink(&self) -> bool {
        (self.0 & 0x40) != 0
    }

    /// Returns `true` if this section must preserve order.
    pub fn linkorder(&self) -> bool {
        (self.0 & 0x80) != 0
    }

    /// Returns `true` if this section is OS non conforming.
    pub fn nonconforming(&self) -> bool {
        (self.0 & 0x100) != 0
    }

    /// Returns `true` if this section is member of a group.
    pub fn group(&self) -> bool {
        (self.0 & 0x200) != 0
    }

    /// Returns `true` if this section contains Thread Local Data.
    pub fn tls(&self) -> bool {
        (self.0 & 0x400) != 0
    }
}

impl core::fmt::Display for SectionFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        // Contant strings.
        const STRINGS: [&'static str; 10] = [
            "Write", "Alloc", "Exec", "Merge", "Strings",
            "InfoLink", "LinkOrder", "OS-Non conforming",
            "Group", "TLS",
        ];

        let flags = [
            self.write(), self.alloc(), self.exec(), self.merge(), self.strings(),
            self.infolink(), self.linkorder(), self.nonconforming(),
            self.group(), self.tls(),
        ];

        let string = flags.iter()
            .zip(STRINGS.iter())
            .filter(|(f, _)| **f)
            .map(|(_, s)| *s)
            .collect::<Vec<_>>();

        if string.is_empty() {
            f.write_str("----")
        } else{
            f.write_str(&string.join(" + "))
        }

    }
}