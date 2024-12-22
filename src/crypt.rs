use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait};
use std::sync::LazyLock;

const KEY: &str = "magic base64";
static MC: LazyLock<MagicCrypt256> = LazyLock::new(|| new_magic_crypt!(KEY, 256));

pub fn encrypt(text: &str) -> String {
    MC.encrypt_str_to_base64(text)
}

pub fn decrypt(base64: &str) -> String {
    MC.decrypt_base64_to_string(base64).unwrap()
}

#[test]
fn test_encrypt_decrypt() {
    let passwd = "xt71FxyaqsnFcFYXgplS";
    let base64 = MC.encrypt_str_to_base64(passwd);

    println!(
        "{:?}, {:?}",
        base64,
        MC.decrypt_base64_to_string(&base64).unwrap()
    );
    assert_eq!(MC.decrypt_base64_to_string(&base64).unwrap(), passwd);
}
