extern crate crypto;
use crypto::{ hmac};
pub mod sha_test;

pub fn hmac_test_main(){
	sha_test::sha_tests_main();
hmac::test::test_hmac_md5();
hmac::test::test_hmac_md5_incremental();
}
