use openssl::symm::Cipher;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{sleep, Duration};

use futures::future;
use tokio::io::{self, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::gfw_config::GFWConfig as Config;
use crate::gfw_decrypt::{gfw_block_size, gfw_decrypt_data, gfw_decrypt_header};
use crate::gfw_encrypt::gfw_encrypt_all;
use crate::{DATA_BUFFER_MAX, HEADER_BUFFER_SIZE, HEADER_SIZE, IV_SIZE, NOISE_SIZE};

// gfw press proxy with encrypt/decrypt ...
pub async fn gfw_press_proxy(is_local_proxy: bool) {
    let config_path = match is_local_proxy {
        true => String::from("./local_gfw.json"),
        false => String::from("./server_gfw.json"),
    };
    let config_str = std::fs::read_to_string(&config_path).unwrap();
    // Load the config structure from the string.
    let config = serde_json::from_str::<Config>(&config_str).unwrap();

    assert_eq!(is_local_proxy, config.is_local_proxy());

    let server = config.get_server();
    let forward_server = config.get_forward_server();
    let cipher = Cipher::aes_256_cfb128();
    let secret_key = config.get_secrect_key();

    // local or remote server?
    match config.is_local_proxy() {
        true => {
            println!("\nlocal proxy server running... at {}", &server);
        }
        false => {
            println!("\nremote (VPS) proxy server running... at {}", &server);
        }
    }

    // http mode or socks5 mode
    match config.is_http_mode() {
        true => {
            println!(
                "<http> proxy server ... forward data to <squid> server: {}",
                &forward_server
            );
        }
        false => {
            println!(
                "<socks5> proxy server ... forward data to <shadowsocks> server: {}",
                &forward_server
            );
        }
    }

    let server_addr = server.parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(server_addr).await.unwrap();

    loop {
        let (local_stream, _) = match listener.accept().await {
            Ok(stream) => stream,
            Err(accept_error) => {
                println!("accpeting socket failed with error {}", accept_error);
                sleep(Duration::from_millis(100)).await;
                continue;
            }
        };
        let forward_addr = forward_server.parse::<SocketAddr>().unwrap();
        tokio::spawn(async move {
            handle_connection(
                cipher,
                &secret_key,
                local_stream,
                forward_addr,
                is_local_proxy,
            )
            .await;
        });
    }
}

async fn handle_connection(
    cipher: Cipher,
    key: &[u8],
    local_stream: TcpStream,
    forward_addr: SocketAddr,
    local_or_remote: bool,
) {
    let forward_stream = TcpStream::connect(forward_addr).await.unwrap();
    gfw_relay(local_stream, forward_stream, local_or_remote, cipher, key).await;
}

pub async fn gfw_relay<L, R>(l: L, r: R, local_or_remote: bool, cipher: Cipher, key: &[u8])
where
    L: AsyncRead + AsyncWrite + Unpin,
    R: AsyncRead + AsyncWrite + Unpin,
{
    let (mut lr, mut lw) = tokio::io::split(l);
    let (mut rr, mut rw) = tokio::io::split(r);

    if local_or_remote {
        // local client
        let client_to_server = transfer_encrypt(&mut lr, &mut rw, cipher, key);
        let server_to_client = transfer_decrypt(&mut rr, &mut lw, cipher, key);
        tokio::select! {
            _ = client_to_server => {}, //{println!("close client to vps server.");} ,
            _ = server_to_client => {}, //{println!("close vps server to client.");} ,
        };
    } else {
        // vps server
        let client_to_server = transfer_decrypt(&mut lr, &mut rw, cipher, key);
        let server_to_client = transfer_encrypt(&mut rr, &mut lw, cipher, key);
        tokio::select! {
            _ = client_to_server => {}, //{ println!("close vps_server to squid_server."); } ,
            _ = server_to_client => {}, //{ println!("close squid_server to vps server."); } ,
        };
    }

    // println!("closing connection");
}

pub async fn transfer_encrypt<'a, R, W>(
    reader: &'a mut R,
    writer: &'a mut W,
    cipher: Cipher,
    key: &[u8],
) -> io::Result<u64>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let mut buf = vec![0u8; DATA_BUFFER_MAX].into_boxed_slice();

    loop {
        // read incoming data from reader
        let data_size = match reader.read(&mut buf).await {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => break,
        };

        // encrypt data from incoming stream
        let cipher_data = gfw_encrypt_all(cipher, &key, &buf[..data_size]);
        let _ = match writer.write_all(&cipher_data).await {
            Ok(_) => (),
            Err(_) => break,
        };
    }
    Ok(0)
}

pub async fn transfer_decrypt<'a, R, W>(
    reader: &'a mut R,
    writer: &'a mut W,
    cipher: Cipher,
    key: &[u8],
) -> io::Result<u64>
where
    R: AsyncRead + Unpin + ?Sized,
    W: AsyncWrite + Unpin + ?Sized,
{
    // header buffer size is 32 bytes
    let mut header_buffer = vec![0u8; HEADER_BUFFER_SIZE];

    loop {
        // get header data from reader
        let header_buffer_size = match reader.read_exact(&mut header_buffer).await {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => break,
        };
        assert_eq!(header_buffer_size, HEADER_BUFFER_SIZE);

        let header_text = gfw_decrypt_header(
            cipher,
            &key,
            &header_buffer[NOISE_SIZE..NOISE_SIZE + HEADER_SIZE],
        );

        let (noise_size, cipher_size) = gfw_block_size(&header_text);
        assert_eq!(noise_size, NOISE_SIZE);

        if cipher_size >= IV_SIZE {
            // read cipher data buffer
            let mut cipher_buffer = vec![0u8; cipher_size];
            let cipher_data_size = match reader.read_exact(&mut cipher_buffer).await {
                Ok(0) => break,
                Ok(n) => n,
                Err(_) => break,
            };
            assert_eq!(cipher_data_size, cipher_size);

            let data = gfw_decrypt_data(cipher, &key, &cipher_buffer);
            let _ = match writer.write_all(&data).await {
                Ok(_) => (),
                Err(_) => break,
            };
        }
    }
    Ok(0)
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
