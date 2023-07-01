use openssl::symm::{decrypt, Cipher};

use crate::{HEADER_SIZE, KEY_SIZE};

//gfw decrypt data remove addional IV before cipher data
// data format: [IV] + [cipher data]
//   [IV] size 16 bytes for aes_256_cfb128
//   [cipher] is variable
pub fn gfw_decrypt_data(cipher: Cipher, key: &[u8], data: &[u8]) -> Vec<u8> {
    debug_assert!(data.len() > 16);
    debug_assert_eq!(key.len(), 32);

    let iv = &data[..16];
    let plaintext = decrypt(cipher, key, Some(&iv), &data[16..]).unwrap();

    plaintext
}

// gfw header format: [xxxxx,xxxxxxxx,,]
// header size 16 bytes
pub fn gfw_get_header(cipher: Cipher, key: &[u8], data: &[u8]) -> Vec<u8> {
    let header = gfw_decrypt_data(cipher, key, data);
    header
}

// gfw block size from header
// gfw header format: [xxxxx,xxxxxxxx,,]
// header size 16 bytes
// [noise:05d,cipher:08d,,]
pub fn gfw_block_size(header: &[u8]) -> (usize, usize) {
    //println!("header = {:?}", header);

    debug_assert_eq!(header.len(), 16);
    debug_assert_eq!([44], &header[5..6]);
    debug_assert_eq!([44, 44], &header[14..16]);

    let noise_size = std::str::from_utf8(&header[0..5])
        .unwrap()
        .parse::<usize>()
        .unwrap_or_default();
    let cipher_size = std::str::from_utf8(&header[6..14])
        .unwrap()
        .parse::<usize>()
        .unwrap_or_default();

    (noise_size, cipher_size)
}

// gfw decrypt data remove header and noise
// gfw cipher data format:
// [header data][cipher data][noise data]
//   [header data] = [IV] + [noise_size,cipher_data_size,,]
//   [cipher data] = [IV] + [data]
//   [noise data] = [random bytes]
pub fn gfw_decrypt_all(cipher: Cipher, key: &[u8], data: &[u8]) -> Vec<u8> {
    debug_assert!(data.len() > HEADER_SIZE);
    debug_assert_eq!(key.len(), KEY_SIZE);

    let header_text = gfw_get_header(cipher, key, &data[..HEADER_SIZE]);
    let (noise_size, cipher_size) = gfw_block_size(&header_text[..]);

    debug_assert_eq!(data.len(), HEADER_SIZE + noise_size + cipher_size);

    let cipher_text = &data[HEADER_SIZE..(HEADER_SIZE + cipher_size)];
    let plain_text = gfw_decrypt_data(cipher, key, cipher_text);

    plain_text
}
