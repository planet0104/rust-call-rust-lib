use std::os::raw::c_int;

#[link(name = "md5", kind="static")]
extern "C" {
  fn md5(input: *const u8, input_len: c_int, output: *mut u8);
}

fn main() {
  let input = "abcdefghijklmnopqrstuvwxyz";
  let mut output = vec![0u8; 16];
  unsafe{ md5(input.as_ptr(), input.len() as c_int, output.as_mut_ptr()) };
  println!("{}", hex::encode(&output));
}
