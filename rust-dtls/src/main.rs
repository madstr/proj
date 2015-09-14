extern crate crypto;
pub mod test;
fn main() {
    crypto::it_works();
	
	test::hmac_test_main();
	
}
