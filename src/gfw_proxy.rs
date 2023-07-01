use std::error::Error;

use tokio::net::{TcpListener, TcpStream};

use futures::future;
use tokio::io::{self, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::gfw_decrypt::{gfw_block_size, gfw_decrypt_data, gfw_decrypt_header};
use crate::gfw_encrypt::gfw_encrypt_all;
use crate::{gfw_get_cipher, gfw_get_key, HEADER_SIZE, IV_SIZE};

// gfw press proxy with encrypt/decrypt ...
pub async fn gfw_press_proxy(
    server: &str,
    forward_server: &str,
    up_or_down: bool,
    use_tokio: bool,
) -> Result<(), Box<dyn Error>> {
    // proxy server listerning ...

    let listener = TcpListener::bind(server).await.unwrap();

    //dbg!(&listener);

    loop {
        let (local_stream, _) = match listener.accept().await {
            Ok(socket_stream) => socket_stream,
            Err(accept_error) => {
                println!("accpeting socket failed with error {}", accept_error);
                continue;
            }
        };

        let remote_stream = match TcpStream::connect(forward_server).await {
            Ok(remote_stream) => remote_stream,
            Err(connect_error) => {
                println!(
                    "connecting to remote socket failed with error {}",
                    connect_error
                );
                continue;
            }
        };

        //println!("{}:{}",listener.local_addr().unwrap().ip(),listener.local_addr().unwrap().port());
        // failed with async move on following function why?
        // tokio::spawn( async move { gfw_relay(local_stream, remote_stream, up_or_down) });
        tokio::spawn(gfw_relay(
            local_stream,
            remote_stream,
            up_or_down,
            use_tokio,
        ));
    }
}

pub async fn gfw_relay<L, R>(l: L, r: R, up: bool, use_tokio: bool)
where
    L: AsyncRead + AsyncWrite + Unpin,
    R: AsyncRead + AsyncWrite + Unpin,
{
    let (mut lr, mut lw) = tokio::io::split(l);
    let (mut rr, mut rw) = tokio::io::split(r);

    if up {
        // local client
        let client_to_server = transfer_encrypt(&mut lr, &mut rw);
        let server_to_client = transfer_decrypt(&mut rr, &mut lw);

        if use_tokio {
            tokio::select! {
                _ = client_to_server => {println!("close client to vps server.");} ,
                _ = server_to_client => {println!("close vps server to client.");} ,
            };
        } else {
            future::try_join(client_to_server, server_to_client)
                .await
                .unwrap();
        }
        //println!("client return value: {x} / {y}");
        //Ok((x, y))
    } else {
        // vps server
        let client_to_server = transfer_decrypt(&mut lr, &mut rw);
        let server_to_client = transfer_encrypt(&mut rr, &mut lw);
        if use_tokio {
            tokio::select! {
                _ = client_to_server => { println!("close vps_server to squid_server."); } ,
                _ = server_to_client => { println!("close squid_server to vps server."); } ,
            };
        } else {
            future::try_join(client_to_server, server_to_client)
                .await
                .unwrap();
        }
    }

    println!("closing connection");
}

// pub async fn gfw_relay_split<'a, LR, LW, RR, RW>(
//     mut lr: &'a mut LR,
//     mut lw: &'a mut LW,
//     mut rr: &'a mut RR,
//     mut rw: &'a mut RW,
//     up: bool,
//     use_tokio: bool,
// )
// // -> io::Result<(u64, u64)>
// where
//     LR: AsyncRead + Unpin,
//     LW: AsyncWrite + Unpin,
//     RR: AsyncRead + Unpin,
//     RW: AsyncWrite + Unpin,
// {
//     if up {
//         // local client
//         let client_to_server = transfer_encrypt(&mut lr, &mut rw);
//         let server_to_client = transfer_decrypt(&mut rr, &mut lw);

//         if use_tokio {
//             tokio::select! {
//                 _ = client_to_server => {println!("close client to vps server.");} ,
//                 _ = server_to_client => {println!("close vps server to client.");} ,
//             };
//         } else {
//             future::try_join(client_to_server, server_to_client)
//                 .await
//                 .unwrap();
//         }
//     } else {
//         // vps server
//         let client_to_server = transfer_decrypt(&mut lr, &mut rw);
//         let server_to_client = transfer_encrypt(&mut rr, &mut lw);

//         if use_tokio {
//             tokio::select! {
//                 _ = client_to_server => { println!("close vps_server to squid_server."); } ,
//                 _ = server_to_client => { println!("close squid_server to vps server."); } ,
//             };
//         } else {
//             future::try_join(client_to_server, server_to_client)
//                 .await
//                 .unwrap();
//         }
//     }
// }

pub async fn transfer_encrypt<'a, R, W>(reader: &'a mut R, writer: &'a mut W) -> io::Result<u64>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let cipher = gfw_get_cipher();
    let key = gfw_get_key();

    // read incoming data from reader
    let mut buf = vec![0u8; 1024 * 4];
    loop {
        match reader.read(&mut buf).await {
            // Return value of `Ok(0)` signifies that the remote has
            // closed
            Ok(0) => return Ok(0),
            Ok(n) => {
                // Copy the data back to socket
                let cipher_data = gfw_encrypt_all(cipher, &key, &buf[..n]);
                writer.write_all(&cipher_data).await.unwrap();
                writer.shutdown().await.unwrap();
                return Ok(cipher_data.len().try_into().unwrap());
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
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let cipher = gfw_get_cipher();
    let key = gfw_get_key();

    loop {
        // get header data from reader
        let mut header_buffer = vec![0u8; HEADER_SIZE];
        let n = reader.read_exact(&mut header_buffer).await.unwrap();

        if n == HEADER_SIZE {
            let header_text = gfw_decrypt_header(cipher, &key, &header_buffer[..]);
            let (noise_size, cipher_size) = gfw_block_size(&header_text[..]);
            let data_size = noise_size + cipher_size;

            loop {
                let mut data_buffer = Box::new(vec![0u8; data_size]);
                reader.read_exact(&mut data_buffer).await.unwrap();
                let data = gfw_decrypt_data(cipher, &key, &data_buffer[..cipher_size]);
                writer.write_all(&data).await.unwrap();
                writer.shutdown().await.unwrap();
                return Ok(data.len().try_into().unwrap());
            }
        }
    }

    //Ok(data.len().try_into().unwrap())
    // loop {
    // // get header data from reader
    // let mut header_buffer = vec![0u8; HEADER_SIZE];
    // reader.read_exact(&mut header_buffer).await.unwrap();
    //     match reader.read_exact(&mut header_buffer).await {
    //         // Return value of `Ok(0)` signifies that the remote has
    //         // closed
    //         Ok(0) => return Ok(0),
    //         Ok(n) => {
    //             assert_eq!(n, HEADER_SIZE);
    //             // decrypt gfw header
    //             let header_text = gfw_decrypt_header(cipher, &key, &header_buffer[..n]);
    //             // get gfw block size ( noise_size, cipher_data_size )
    //             let (noise_size, cipher_size) = gfw_block_size(&header_text[..]);
    //             let data_size = noise_size + cipher_size;

    //             // get cipher data from reader
    //             let mut data_buffer = Box::new(vec![0u8; data_size]);
    //             loop {
    //                 match reader.read_exact(&mut data_buffer).await {
    //                     Ok(0) => return Ok(0),
    //                     Ok(n) => {
    //                         assert_eq!(n, data_size);
    //                         let data = gfw_decrypt_data(cipher, &key, &data_buffer[..cipher_size]);
    //                         if writer.write_all(&data).await.is_err() {
    //                             // Unexpected socket error. There isn't much we can
    //                             // do here so just stop processing.
    //                             return Ok(n.try_into().unwrap());
    //                         };
    //                     }
    //                     Err(_) => {
    //                         // Unexpected socket error. There isn't much we can do
    //                         // here so just stop processing.
    //                         return Ok(0);
    //                     }
    //                 }
    //             }
    //         }
    //         Err(_) => {
    //             // Unexpected socket error. There isn't much we can do
    //             // here so just stop processing.
    //             return Ok(0);
    //         }
    //     }
    // }
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

// standard proxy server without any encrypt/decrypt ...
pub async fn gfw_std_proxy(server: &str, forward_server: &str) -> Result<(), Box<dyn Error>> {
    // proxy server listerning ...

    let listener = TcpListener::bind(server).await.unwrap();

    loop {
        let (local_stream, _) = match listener.accept().await {
            Ok(socket_stream) => socket_stream,
            Err(accept_error) => {
                println!("accpeting socket failed with error {}", accept_error);
                continue;
            }
        };

        let remote_stream = match TcpStream::connect(forward_server).await {
            Ok(remote_stream) => remote_stream,
            Err(connect_error) => {
                println!(
                    "connecting to remote socket failed with error {}",
                    connect_error
                );
                continue;
            }
        };
        // println!(
        //     "{}:{}",
        //     listener.local_addr().unwrap().ip(),
        //     listener.local_addr().unwrap().port()
        // );

        tokio::spawn(relay(local_stream, remote_stream));
    }
}

pub async fn relay<L, R>(l: L, r: R)
//-> io::Result<(u64, u64)>
where
    L: AsyncRead + AsyncWrite + Unpin,
    R: AsyncRead + AsyncWrite + Unpin,
{
    let (mut lr, mut lw) = tokio::io::split(l);
    let (mut rr, mut rw) = tokio::io::split(r);
    relay_split(&mut lr, &mut lw, &mut rr, &mut rw).await;
}

pub async fn relay_split<'a, LR, LW, RR, RW>(
    mut lr: &'a mut LR,
    mut lw: &'a mut LW,
    mut rr: &'a mut RR,
    mut rw: &'a mut RW,
)
//-> io::Result<(u64, u64)>
where
    LR: AsyncRead + Unpin + ?Sized,
    LW: AsyncWrite + Unpin + ?Sized,
    RR: AsyncRead + Unpin + ?Sized,
    RW: AsyncWrite + Unpin + ?Sized,
{
    let client_to_server = transfer(&mut lr, &mut rw);
    let server_to_client = transfer(&mut rr, &mut lw);
    future::try_join(client_to_server, server_to_client)
        .await
        .unwrap();
    println!("closing connection");
}

pub async fn transfer<'a, R, W>(reader: &'a mut R, writer: &'a mut W) -> io::Result<u64>
where
    R: AsyncRead + Unpin + ?Sized,
    W: AsyncWrite + Unpin + ?Sized,
{
    let len = tokio::io::copy(reader, writer).await?;
    writer.shutdown().await?;
    Ok(len)
}
