#![cfg_attr(feature = "with-bench", feature(test))]
extern crate rand;
extern crate rustc_serialize as serialize;

#[cfg(all(test, feature = "with-bench"))]
extern crate test;

pub mod hmac;
pub mod md5;
pub mod buffer;
pub mod symmetriccipher;
pub mod digest;
mod cryptoutil;
pub mod mac;
mod step_by;
pub mod sha2;
mod simd;

pub fn it_works() {
println!("yes\n");
}
