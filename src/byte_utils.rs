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

pub fn as_u32_vec(bytes: &[u8]) -> Vec<u32> {
    let mut vec: Vec<u32> = vec!();
    for i in 0 .. (bytes.len() / 4) {
        vec.push(as_u32_le(&bytes[i*4..][..4]));
    }

    vec
}

pub fn as_string(bytes: &[u8], offset: usize, string_length: usize) -> String {
    String::from_utf8(bytes[offset..offset + string_length - 1].to_vec())
        .unwrap()
}