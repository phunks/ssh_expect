use magic_crypt::{new_magic_crypt, MagicCryptTrait};

const KEY: &str = "magic base64";

pub fn encrypt(text: &str) -> String {
    let mc = new_magic_crypt!(KEY, 256);
    mc.encrypt_str_to_base64(text)
}

pub fn decrypt(base64: &str) -> String {
    let mc = new_magic_crypt!(KEY, 256);
    mc.decrypt_base64_to_string(&base64).unwrap()
}


#[test]
fn test_encrypt_decrypt() {
    let mc = new_magic_crypt!("magickey", 256);
    let base64 = mc.encrypt_str_to_base64("Zaq12wsx");
    // assert_eq!("DS/2U8royDnJDiNY2ps3f6ZoTbpZo8ZtUGYLGEjwLDQ=", base64);
    // assert_eq!("http://magiclen.org", mc.decrypt_base64_to_string(&base64).unwrap());

    println!("{:?}, {:?}",base64, mc.decrypt_base64_to_string(&base64).unwrap());
}