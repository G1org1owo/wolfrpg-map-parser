use serde::Serialize;
use crate::byte_utils::{as_string, as_u32_le};

#[derive(Serialize)]
pub struct CommentCommand {
    comment: String,
}

impl CommentCommand {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset = 2;

        let comment_length: usize = as_u32_le(&bytes[offset..offset+4]) as usize;
        offset += 4;
        let comment: String = as_string(bytes, offset, comment_length);
        offset += comment_length;

        offset += 1; // command end signature

        (offset, Self {
            comment,
        })
    }
}