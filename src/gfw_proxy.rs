use futures::future;
use tokio::io::{self, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::gfw_decrypt::{gfw_block_size, gfw_decrypt_data, gfw_decrypt_header};
use crate::gfw_encrypt::gfw_encrypt_all;
use crate::{gfw_get_cipher, gfw_get_key};

pub async fn transfer<'a, R, W>(reader: &'a mut R, writer: &'a mut W) -> io::Result<u64>
where
    R: AsyncRead + Unpin + ?Sized,
    W: AsyncWrite + Unpin + ?Sized,
{
    let len = tokio::io::copy(reader, writer).await?;
    writer.shutdown().await?;
    Ok(len)
}

pub async fn relay<'a, L, R>(l: &'a mut L, r: &'a mut R) -> io::Result<(u64, u64)>
where
    L: AsyncRead + AsyncWrite + Unpin + ?Sized,
    R: AsyncRead + AsyncWrite + Unpin + ?Sized,
{
    let (mut lr, mut lw) = tokio::io::split(l);
    let (mut rr, mut rw) = tokio::io::split(r);
    return relay_split(&mut lr, &mut lw, &mut rr, &mut rw).await;
}

pub async fn relay_split<'a, LR, LW, RR, RW>(
    mut lr: &'a mut LR,
    mut lw: &'a mut LW,
    mut rr: &'a mut RR,
    mut rw: &'a mut RW,
) -> io::Result<(u64, u64)>
where
    LR: AsyncRead + Unpin + ?Sized,
    LW: AsyncWrite + Unpin + ?Sized,
    RR: AsyncRead + Unpin + ?Sized,
    RW: AsyncWrite + Unpin + ?Sized,
{
    let client_to_server = transfer(&mut lr, &mut rw);
    let server_to_client = transfer(&mut rr, &mut lw);
    return future::try_join(client_to_server, server_to_client).await;
}

pub async fn gfw_relay<'a, L, R>(l: &'a mut L, r: &'a mut R, up: bool) -> io::Result<(u64, u64)>
where
    L: AsyncRead + AsyncWrite + Unpin + ?Sized,
    R: AsyncRead + AsyncWrite + Unpin + ?Sized,
{
    let (mut lr, mut lw) = tokio::io::split(l);
    let (mut rr, mut rw) = tokio::io::split(r);
    return gfw_relay_split(&mut lr, &mut lw, &mut rr, &mut rw, up).await;
}

pub async fn gfw_relay_split<'a, LR, LW, RR, RW>(
    mut lr: &'a mut LR,
    mut lw: &'a mut LW,
    mut rr: &'a mut RR,
    mut rw: &'a mut RW,
    up: bool,
) -> io::Result<(u64, u64)>
where
    LR: AsyncRead + Unpin + ?Sized,
    LW: AsyncWrite + Unpin + ?Sized,
    RR: AsyncRead + Unpin + ?Sized,
    RW: AsyncWrite + Unpin + ?Sized,
{
    if up {
        let client_to_server = transfer_encrypt(&mut lr, &mut rw);
        let server_to_client = transfer_decrypt(&mut rr, &mut lw);
        return future::try_join(client_to_server, server_to_client).await;
    } else {
        let client_to_server = transfer_decrypt(&mut lr, &mut rw);
        let server_to_client = transfer_encrypt(&mut rr, &mut lw);
        return future::try_join(client_to_server, server_to_client).await;
    }
}

pub async fn transfer_encrypt<'a, R, W>(reader: &'a mut R, writer: &'a mut W) -> io::Result<u64>
where
    R: AsyncRead + Unpin + ?Sized,
    W: AsyncWrite + Unpin + ?Sized,
{
    let cipher = gfw_get_cipher();
    let key = gfw_get_key();

    // read incoming data from reader
    let mut buf = vec![];
    loop {
        match reader.read(&mut buf).await {
            // Return value of `Ok(0)` signifies that the remote has
            // closed
            Ok(0) => return Ok(0),
            Ok(n) => {
                // Copy the data back to socket
                let cipher_data = gfw_encrypt_all(cipher, &key, &buf[..n]);
                if writer.write_all(&cipher_data).await.is_err() {
                    // Unexpected socket error. There isn't much we can
                    // do here so just stop processing.
                    return Ok(n.try_into().unwrap());
                }
            }
            Err(_) => {
                // Unexpected socket error. There isn't much we can do
                // here so just stop processing.
                return Ok(0);
            }
        }
    }
}

pub async fn transfer_decrypt<'a, R, W>(reader: &'a mut R, writer: &'a mut W) -> io::Result<u64>
where
    R: AsyncRead + Unpin + ?Sized,
    W: AsyncWrite + Unpin + ?Sized,
{
    let cipher = gfw_get_cipher();
    let key = gfw_get_key();

    // get header data from reader
    let mut header_buffer = vec![0u8; 32];
    loop {
        match reader.read_exact(&mut header_buffer).await {
            // Return value of `Ok(0)` signifies that the remote has
            // closed
            Ok(0) => return Ok(0),
            Ok(n) => {
                // decrypt gfw header
                let header_text = gfw_decrypt_header(cipher, &key, &header_buffer[..n]);
                // get gfw block size ( noise_sieze, cipher_data_size )
                let (noise_size, cipher_size) = gfw_block_size(&header_text[..]);
                let data_size = noise_size + cipher_size;

                // get cipher data from reader
                let mut data_buffer = Box::new(vec![0u8; data_size]);
                loop {
                    match reader.read_exact(&mut data_buffer).await {
                        Ok(0) => return Ok(0),
                        Ok(n) => {
                            let data = gfw_decrypt_data(cipher, &key, &data_buffer[..cipher_size]);
                            if writer.write_all(&data).await.is_err() {
                                // Unexpected socket error. There isn't much we can
                                // do here so just stop processing.
                                return Ok(n.try_into().unwrap());
                            };
                        }
                        Err(_) => {
                            // Unexpected socket error. There isn't much we can do
                            // here so just stop processing.
                            return Ok(0);
                        }
                    }
                }
            }
            Err(_) => {
                // Unexpected socket error. There isn't much we can do
                // here so just stop processing.
                return Ok(0);
            }
        }
    }
}

// pub async fn transfer<'a, R, W>(reader: &'a mut R, writer: &'a mut W) -> io::Result<u64>
// where
//     R: AsyncRead + Unpin + ?Sized,
//     W: AsyncWrite + Unpin + ?Sized,
// {
//     let len = tokio::io::copy(reader, writer).await?;
//     writer.shutdown().await?;
//     Ok(len)
// }
