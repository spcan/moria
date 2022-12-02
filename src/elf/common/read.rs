//! Internal functions to read other endian data.



use byteorder::{
    BigEndian, LittleEndian,
    ByteOrder,
};



#[allow(dead_code)]
/// Private read function.
pub(crate) fn little16(bytes: &[u8]) -> u16 {
    LittleEndian::read_u16(bytes)
}

#[allow(dead_code)]
/// Private read function.
pub(crate) fn big16(bytes: &[u8]) -> u16 {
    BigEndian::read_u16(bytes)
}

#[allow(dead_code)]
/// Private read function.
pub(crate) fn little32(bytes: &[u8]) -> u32 {
    LittleEndian::read_u32(bytes)
}

#[allow(dead_code)]
/// Private read function.
pub(crate) fn big32(bytes: &[u8]) -> u32 {
    BigEndian::read_u32(bytes)
}

#[allow(dead_code)]
/// Private read function.
pub(crate) fn little64(bytes: &[u8]) -> u64 {
    LittleEndian::read_u64(bytes)
}

#[allow(dead_code)]
/// Private read function.
pub(crate) fn big64(bytes: &[u8]) -> u64 {
    BigEndian::read_u64(bytes)
}
