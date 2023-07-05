pub mod aes;
pub mod cr4;

pub fn get_length(bytes: &[u8]) -> Option<i32> {
    if bytes.len() < 4 {
        None
    } else {
        let mut num: i32 = 0;
        num |= (bytes[0] & 0xff) as i32;
        num |= ((bytes[1] & 0xff) as i32) << 8;
        num |= ((bytes[2] & 0xff) as i32) << 16;
        num |= ((bytes[3] & 0xff) as i32) << 24;
        Some(num)
    }
}
