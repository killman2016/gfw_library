use openssl::{
    rand::rand_bytes,
    symm::{encrypt, Cipher},
};

use crate::{IV_SIZE, KEY_SIZE, NOISE_MAX};

//gfw encrypt data add addional IV before cipher data
// data format: [IV] + [cipher data]
//   [IV] size 16 bytes for aes_256_cfb128
//   [cipher] is variable
pub fn gfw_encrypt_data(cipher: Cipher, key: &[u8], data: &[u8]) -> Vec<u8> {
    debug_assert_eq!(key.len(), KEY_SIZE);

    let mut iv = [0u8; IV_SIZE];
    rand_bytes(&mut iv).unwrap();

    let ciphertext = encrypt(cipher, key, Some(&iv), data).unwrap();

    [iv.to_vec(), ciphertext].concat()
}

pub fn gfw_get_noise(size: usize) -> Vec<u8> {
    let noise_size = NOISE_MAX - (size % NOISE_MAX);
    let mut noise_data = vec![0u8; noise_size];
    rand_bytes(&mut noise_data).unwrap();
    noise_data
}

// gfw encrypt data with addition header and noise
// gfw cipher data format:
// [header data][cipher data][noise data]
//   [header data] = [IV] + [xxxxx,xxxxxxxx,,]
//   [cipher data] = [IV] + [data]
//   [noise data] = [random bytes]
pub fn gfw_encrypt_all(cipher: Cipher, key: &[u8], data: &[u8]) -> Vec<u8> {
    debug_assert_eq!(key.len(), KEY_SIZE);

    let cipher_data = gfw_encrypt_data(cipher, key, data);
    let noise_data = gfw_get_noise(cipher_data.len());

    let noise_size = noise_data.len();
    let cipher_size = cipher_data.len();
    let header_text = format!("{:05},{:08},,", noise_size, cipher_size);
    let header = header_text.as_bytes();

    let mut iv = [0u8; IV_SIZE];
    rand_bytes(&mut iv).unwrap();

    let cipher_header = gfw_encrypt_data(cipher, key, &header);

    debug_assert_eq!([44], &header[5..6]);
    debug_assert_eq!([44, 44], &header[14..16]);

    [cipher_header, cipher_data, noise_data].concat()
}
