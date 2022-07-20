use data_encoding::HEXUPPER;

pub fn encode_password(password: &str) -> String {
    let salt = passwords::hasher::gen_salt();
    let hashed = passwords::hasher::bcrypt(10, &salt, "password\0").unwrap();
    format!("{}${}", HEXUPPER.encode(&hashed), HEXUPPER.encode(&salt))
}

pub fn verify_password(password: &str, input: &str) -> bool {
    let d: Vec<_> = password.split('$').collect();
    if d.len() < 2 {
        return false;
    }
    let hashed = HEXUPPER.decode(d[0].as_bytes());
    if hashed.is_err() {
        return false;
    }
    let hashed = hashed.unwrap();
    let salt = HEXUPPER.decode(d[1].as_bytes());
    if salt.is_err() {
        return false;
    }
    let salt = salt.unwrap();
    unsafe { passwords::hasher::identify_bcrypt(10, &salt, input, &hashed) }
}

#[test]
fn test_password() {
    let pwd = "root";
    let encoded = encode_password(pwd);
    assert_eq!(true, verify_password(encoded.as_str(), pwd));
}
