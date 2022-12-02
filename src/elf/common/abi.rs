//! Operating System codes of the ELF format.



#![allow(dead_code)]



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperatingSystem {
    SystemV(u8),
    HPUX(u8),
    NetBSD(u8),
    Linux(u8),
    GNUHurd(u8),
    Solaris(u8),
    AIX(u8),
    IRIX(u8),
    FreeBSD(u8),
    Tru64(u8),
    NovellModesto(u8),
    OpenBSD(u8),
    OpenVMS(u8),
    NonStopKernel(u8),
    AROS(u8),
    FenixOS(u8),
    CloudABI(u8),
    OpenVOS(u8),

    None,
}

impl core::convert::From<(u8, u8)> for OperatingSystem {
    fn from((os, v): (u8, u8)) -> OperatingSystem {
        use OperatingSystem::*;

        match os {
            0x00 => match v{
                0 => OperatingSystem::None,
                _ => SystemV(v),
            },
            0x01 => HPUX(v),
            0x02 => NetBSD(v),
            0x03 => Linux(v),
            0x04 => GNUHurd(v),
            0x06 => Solaris(v),
            0x07 => AIX(v),
            0x08 => IRIX(v),
            0x09 => FreeBSD(v),
            0x0A => Tru64(v),
            0x0B => NovellModesto(v),
            0x0C => OpenBSD(v),
            0x0D => OpenVMS(v),
            0x0E => NonStopKernel(v),
            0x0F => AROS(v),
            0x10 => FenixOS(v),
            0x11 => CloudABI(v),
            0x12 => OpenVOS(v),

            _ => None,
        }
    }
}


impl core::fmt::Display for OperatingSystem {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use OperatingSystem::*;

        let arg = match *self {
            SystemV(v)       => format!("System V - rev {}", v),
            HPUX(v)          => format!("HP-UX - rev {}", v),
            NetBSD(v)        => format!("NetBSD - rev {}", v),
            Linux(v)         => format!("Linux - rev {}", v),
            GNUHurd(v)       => format!("GNU Hurd - rev {}", v),
            Solaris(v)       => format!("Solaris - rev {}", v),
            AIX(v)           => format!("AIX - rev {}", v),
            IRIX(v)          => format!("IRIX - rev {}", v),
            FreeBSD(v)       => format!("FreeBSD - rev {}", v),
            Tru64(v)         => format!("Tru64 - rev {}", v),
            NovellModesto(v) => format!("Novell Modesto - rev {}", v),
            OpenBSD(v)       => format!("OpenBSD - rev {}", v),
            OpenVMS(v)       => format!("OpenVMS - rev {}", v),
            NonStopKernel(v) => format!("NonStop Kernel - rev {}", v),
            AROS(v)          => format!("AROS - rev {}", v),
            FenixOS(v)       => format!("Fenix OS - rev {}", v),
            CloudABI(v)      => format!("Cload ABI - rev {}", v),
            OpenVOS(v)       => format!("Stratus Technologies OpenVOS - rev {}", v),

            None => String::from("No OS ABI defined"),
        };

        write!(f, "{}", arg)
    }
}