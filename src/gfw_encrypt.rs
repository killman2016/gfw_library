use openssl::{
    rand::rand_bytes,
    symm::{encrypt, Cipher},
};

use crate::{IV_SIZE, KEY_SIZE, NOISE_SIZE};

// gfw encrypt data add addional IV before cipher data
// data format: [IV] + [cipher data]
//   [IV] size 16 bytes for aes_256_cfb128
//   [cipher] is variable
pub fn gfw_encrypt_data(cipher: Cipher, key: &[u8], data: &[u8]) -> Vec<u8> {
    debug_assert_eq!(key.len(), KEY_SIZE);

    let mut iv = vec![0u8; IV_SIZE];
    rand_bytes(&mut iv).unwrap();

    let ciphertext = encrypt(cipher, key, Some(&iv), data).unwrap();

    // let mut cipher_buffer = BytesMut::with_capacity(IV_SIZE+ciphertext.len());
    // cipher_buffer.put_slice(&iv);
    // cipher_buffer.put_slice(&ciphertext);
    // cipher_buffer

    [iv.to_vec(), ciphertext].concat()
}

pub fn gfw_get_noise(cipher_size: usize) -> (Vec<u8>, usize) {
    let _noise_size = cipher_size % NOISE_SIZE;

    let mut noise_data = vec![0u8; NOISE_SIZE];
    rand_bytes(&mut noise_data).unwrap();
    (noise_data, NOISE_SIZE)
}

// gfw encrypt data with addition header and noise
// gfw cipher data format:
// [header data][cipher data][noise data]
//   [header data] = [IV] + [xxxxx,xxxxxxxx,,]
//   [cipher data] = [IV] + [data]
//   [noise data] = [random bytes]

pub fn gfw_encrypt_all(cipher: Cipher, key: &[u8], data: &[u8]) -> Vec<u8> {
    debug_assert_eq!(key.len(), KEY_SIZE);

    // let data_size = data.len();

    // if data_size > 16 {
    //     println!(
    //         "\nplaintext <{}>: {:?} ... {:?}",
    //         data_size,
    //         &data[..8],
    //         &data[(data_size - 8)..]
    //     );
    // } else {
    //     println!("\nplaintext <{}>: {:?}", &data.len(), &data[..]);
    // }

    let cipher_data = gfw_encrypt_data(cipher, key, data);
    let cipher_size = cipher_data.len();
    let (noise_data, noise_size) = gfw_get_noise(cipher_size);
    let check_size = (noise_size + cipher_size) % 999;
    let header_text = format!("{:05}{:03}{:08}", noise_size, check_size, cipher_size);
    let header = header_text.as_bytes();

    let mut iv = vec![0u8; IV_SIZE];
    rand_bytes(&mut iv).unwrap();

    let cipher_header = gfw_encrypt_data(cipher, key, &header);

    // println!("encrypt size: {}", cipher_size - IV_SIZE);

    [noise_data, cipher_header, cipher_data].concat()
}
