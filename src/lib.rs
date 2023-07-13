use openssl::sha::sha256;
use openssl::symm::Cipher;

const TESTPASSED: &str = "e%aQ02#F9srkfg6$";
const HEADER_BUFFER_SIZE: usize = 1024; // NOISE_SIZE + HEADER_SIZE;
const HEADER_SIZE: usize = 32;
const NOISE_SIZE: usize = HEADER_BUFFER_SIZE - HEADER_SIZE;
const IV_SIZE: usize = 16;
const KEY_SIZE: usize = 32;
const BUFFER_MAX: usize = 1024 * 8 - IV_SIZE;

/// Exit code when server exits unexpectedly
pub const EXIT_CODE_SERVER_EXIT_UNEXPECTEDLY: sysexits::ExitCode = sysexits::ExitCode::Software;
/// Exit code when server aborted
pub const EXIT_CODE_SERVER_ABORTED: sysexits::ExitCode = sysexits::ExitCode::Software;
/// Exit code when loading configuration from file fails
pub const EXIT_CODE_LOAD_CONFIG_FAILURE: sysexits::ExitCode = sysexits::ExitCode::Config;
/// Exit code when loading ACL from file fails
pub const EXIT_CODE_LOAD_ACL_FAILURE: sysexits::ExitCode = sysexits::ExitCode::Config;
/// Exit code when insufficient params are passed via CLI
pub const EXIT_CODE_INSUFFICIENT_PARAMS: sysexits::ExitCode = sysexits::ExitCode::Usage;
/// Build timestamp in UTC
pub const BUILD_TIME: &str = build_time::build_time_utc!();

pub mod gfw_config;
pub mod gfw_decrypt;
pub mod gfw_encrypt;
pub mod gfw_proxy;
pub mod service;

#[cfg(test)]
mod tests {

    use super::*;
    use crate::gfw_decrypt::{gfw_decrypt_all, gfw_decrypt_data};
    use crate::gfw_encrypt::{gfw_encrypt_all, gfw_encrypt_data};

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
