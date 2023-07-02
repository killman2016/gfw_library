use openssl::sha::sha256;
use openssl::symm::Cipher;

const TESTPASSED: &str = "e%wQ02#L7srkfg9$";
const NOISE_MAX: usize = 999;
const IV_SIZE: usize = 16;
const HEADER_SIZE: usize = 32;
const KEY_SIZE: usize = 32;
const BUFFER_MAX: usize = 1024 * 4;
pub mod gfw_decrypt;
pub mod gfw_encrypt;
pub mod gfw_proxy;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::gfw_decrypt::{gfw_decrypt_all, gfw_decrypt_data};
    use crate::gfw_encrypt::{gfw_encrypt_all, gfw_encrypt_data};

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn cipher_works() {
        let origin_text = b"Some Crypto Text Example!";

        let cipher = gfw_get_cipher();
        let key = gfw_get_key();

        let cipher_text = gfw_encrypt_data(cipher, &key, origin_text);
        let plain_text = gfw_decrypt_data(cipher, &key, &cipher_text);
        assert_eq!(origin_text, &plain_text[..]);

        let cipher_data = gfw_encrypt_all(cipher, &key, origin_text);
        let plain_data = gfw_decrypt_all(cipher, &key, &cipher_data);

        println!("text  = {:?}", origin_text);
        println!("plain = {:?}", plain_data);

        assert_eq!(origin_text, &plain_data[..]);
    }
}

// get gfw cipher kind
pub fn gfw_get_cipher() -> Cipher {
    Cipher::aes_256_cfb128()
}

// get test key
pub fn gfw_get_key() -> [u8; 32] {
    let key = sha256(TESTPASSED.as_bytes());
    debug_assert_eq!(key.len(), 32);
    key
}

//get secrect key
pub fn gfw_secrect_key(passwd: &str) -> [u8; 32] {
    let key = sha256(passwd.as_bytes());
    debug_assert_eq!(key.len(), 32);
    key
}
