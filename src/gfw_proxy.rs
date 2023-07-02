use std::error::Error;

use tokio::net::{TcpListener, TcpStream};

use futures::future;
use tokio::io::{self, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::gfw_decrypt::{gfw_block_size, gfw_decrypt_data, gfw_decrypt_header};
use crate::gfw_encrypt::gfw_encrypt_all;
use crate::{gfw_get_cipher, gfw_get_key, BUFFER_MAX, HEADER_SIZE, IV_SIZE};

// gfw press proxy with encrypt/decrypt ...
pub async fn gfw_press_proxy(
    server: String,
    forward_server: String,
    up: bool,
    use_tokio: bool,
) -> io::Result<()> {
    // proxy server listerning ...

    let listener = TcpListener::bind(server).await.unwrap();

    //dbg!(&listener);

    loop {
        let (local_stream, _) = listener.accept().await.unwrap();
        //  {
        //     Ok(socket_stream) => socket_stream,
        //     Err(accept_error) => {
        //         println!("accpeting socket failed with error {}", accept_error);
        //         continue;
        //     }
        // };

        let proxy_server = forward_server.clone();

        tokio::spawn(async move {
            handle_connection(local_stream, proxy_server.as_str(), up, use_tokio).await;
        });
    }
}

async fn handle_connection(local_stream: TcpStream, proxy_server: &str, up: bool, use_tokio: bool) {
    println!(
        "listerning ... {}:{}",
        local_stream.local_addr().unwrap().ip(),
        local_stream.local_addr().unwrap().port()
    );
    println!("connect to proxy server: {}", proxy_server);

    let remote_stream = TcpStream::connect(proxy_server).await.unwrap();
    gfw_relay(local_stream, remote_stream, up, use_tokio).await;
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

pub async fn transfer_encrypt<'a, R, W>(reader: &'a mut R, writer: &'a mut W) -> io::Result<u64>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let cipher = gfw_get_cipher();
    let key = gfw_get_key();

    let mut buf = vec![0u8; BUFFER_MAX];

    loop {
        // read incoming data from reader
        // not working on this: Box::new(vec![]);

        //let mut buffer: Vec<u8>= vec![];
        //while let Some(s) = reader.read_buf(buf).next().await {

        let n = reader.read(&mut buf).await.unwrap();
        if n > 0 {
            // encrypt
            let cipher_data = gfw_encrypt_all(cipher, &key, &buf[..n]);
            // decrypt to get bock size of noise and cipher data size
            let header_text = gfw_decrypt_header(cipher, &key, &cipher_data[..HEADER_SIZE]);
            println!("118 {:?}", &header_text);
            let (noise_size, cipher_size) = gfw_block_size(&header_text[..]);
            // send header with noise data together
            let cipher_pos = HEADER_SIZE + noise_size;
            // orginal data length equal to cipher data lenght
            assert_eq!(n, cipher_size - IV_SIZE);
            // write header + noise data to write
            println!("write incoming data header to forward server ...");
            writer.write_all(&cipher_data[..cipher_pos]).await.unwrap();
            // writer.flush().await.unwrap();
            // write cipher data to writer
            // send cipher date
            println!("write incoming cipher data to forward server ...");
            writer.write_all(&cipher_data[cipher_pos..]).await.unwrap();
            //writer.flush().await.unwrap();
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

    // header buffer size is 32 bytes

    loop {
        let mut header_buffer = vec![0u8; HEADER_SIZE];
        // get header data from reader
        // println!("read cipher header ...");
        let header_size = reader.read_exact(&mut header_buffer).await.unwrap();

        if header_size == HEADER_SIZE {
            //decrypt header and get block size of noise and cipher data
            let header_text = gfw_decrypt_header(cipher, &key, &header_buffer[..header_size]);
            println!("155 {:?}", &header_text[..]);
            let (noise_size, cipher_size) = gfw_block_size(&header_text[..]);

            // read noise buffer
            let mut noise_buffer = vec![0u8; noise_size];
            let _ = reader.read_exact(&mut noise_buffer).await.unwrap();

            // read data buffer
            let mut data_buffer = vec![0u8; cipher_size];
            let data_size = reader.read_exact(&mut data_buffer).await.unwrap();

            if data_size == cipher_size {
                assert_eq!(data_size, cipher_size);

                println!(
                    "\n noise size:{:<8},\ncipher size:{:<8} \nread size: {:<8}",
                    noise_size, cipher_size, data_size
                );

                // let cipher_data = vec![0u8];
                // let x =  &data_buffer[noise_size..];
                let data = gfw_decrypt_data(cipher, &key, &data_buffer[..]);
                ////
                writer.write_all(&data).await.unwrap();
                ////
                //// writer.flush().await.unwrap();
            }
        }
    }
}

//
// standard proxy server without any encrypt/decrypt ...
//

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
