use std::{os::raw::c_int, slice, ptr::slice_from_raw_parts_mut};

/// md5
/// input 输入的字节数组
/// input_len 输入的字节数组长度
/// output 存放结果的字节数组，长度固定16字节
#[no_mangle]
pub extern fn md5(input:*const u8, input_len:c_int, output:*mut u8){
    let bytes = unsafe{ slice::from_raw_parts(input, input_len as usize) };
    let digest = md5::compute(bytes).to_vec();
    let output = unsafe{ &mut *slice_from_raw_parts_mut(output, digest.len()) };
    output.copy_from_slice(&digest);
}