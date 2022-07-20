use sha_crypt::{sha512_check, sha512_simple, Sha512Params};

pub fn encode_password(password: &str) -> String {
    let params = Sha512Params::new(10_000).expect("RandomError!");
    sha512_simple(password, &params).expect("Should not fail")
}

pub fn verify_password(hashed_password: &str, password: &str) -> bool {
    sha512_check(password, hashed_password).is_ok()
}

#[test]
fn test_password() {
    let pwd = "root";
    let encoded = encode_password(pwd);
    println!("{}", encoded);
    assert_eq!(true, verify_password(encoded.as_str(), pwd));
}
