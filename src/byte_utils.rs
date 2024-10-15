pub fn as_u32_le(bytes: &[u8]) -> u32 {
    ((bytes[0] as u32) <<  0) |
    ((bytes[1] as u32) <<  8) |
    ((bytes[2] as u32) << 16) |
    ((bytes[3] as u32) << 24)
}

pub fn as_u32_be(bytes: &[u8]) -> u32 {
    ((bytes[0] as u32) << 24) |
    ((bytes[1] as u32) << 16) |
    ((bytes[2] as u32) <<  8) |
    ((bytes[3] as u32) <<  0)
}

pub fn as_u16_le(bytes: &[u8]) -> u16 {
    ((bytes[0] as u16) << 0) |
    ((bytes[1] as u16) << 8)
}

pub fn as_u16_be(bytes: &[u8]) -> u16 {
    ((bytes[0] as u16) << 8) |
    ((bytes[1] as u16) << 0)
}

pub fn as_u32_array(bytes: &[u8]) -> &[u32] {
    unsafe {
        let (_, data, _): (&[u8], &[u32], &[u8]) = bytes.align_to::<u32>();

        data
    }
}

pub fn as_string(bytes: &[u8], offset: usize, string_length: usize) -> String {
    String::from_utf8(bytes[offset..offset + string_length - 1].to_vec())
        .unwrap()
}