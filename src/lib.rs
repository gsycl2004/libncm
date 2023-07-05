use std::{ffi::{c_int, c_char, c_ulonglong}, slice};



pub mod util;
pub mod convert;
pub mod model;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[no_mangle]
pub unsafe extern "C" fn ncm2mp3(src:*mut c_char,src_length:c_ulonglong,dst:*mut c_char,dst_length:c_ulonglong){
    
    let src = Vec::from_raw_parts(src as *mut u8, src_length as usize, src_length as usize);
    let dst = Vec::from_raw_parts(dst as *mut u8, dst_length as usize, dst_length as usize);
    let src = String::from_utf8(src).unwrap();
    let dst =String::from_utf8(dst).unwrap();
    convert::ncm2mp3(src.clone(), dst.clone());
}
