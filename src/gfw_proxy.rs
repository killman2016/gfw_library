//use bytes::{Buf, BufMut, BytesMut};

use std::error::Error;

use tokio::net::{TcpListener, TcpStream};

use futures::future;
use tokio::io::{self, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::gfw_decrypt::{gfw_block_size, gfw_decrypt_data, gfw_decrypt_header};
use crate::gfw_encrypt::gfw_encrypt_all;
use crate::{gfw_get_cipher, gfw_get_key, BUFFER_MAX, HEADER_BUFFER_SIZE, HEADER_SIZE, NOISE_SIZE};

// gfw press proxy with encrypt/decrypt ...
pub async fn gfw_press_proxy(server: String, forward_server: String, up: bool) -> io::Result<()> {
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
            handle_connection(local_stream, proxy_server.as_str(), up).await;
        });
    }
}

async fn handle_connection(local_stream: TcpStream, proxy_server: &str, up: bool) {
    println!(
        "listerning ... {}:{}",
        local_stream.local_addr().unwrap().ip(),
        local_stream.local_addr().unwrap().port()
    );
    println!("connect to proxy server: {}", proxy_server);

    let remote_stream = TcpStream::connect(proxy_server).await.unwrap();

    gfw_relay(local_stream, remote_stream, up).await;
}

pub async fn gfw_relay<L, R>(l: L, r: R, up: bool)
where
    L: AsyncRead + AsyncWrite + Unpin,
    R: AsyncRead + AsyncWrite + Unpin,
{
    let (mut lr, mut lw) = tokio::io::split(l);
    let (mut rr, mut rw) = tokio::io::split(r);
    //println!("gfw_realy begin ...1");
    if up {
        // local client
        //println!("gfw_realy client begin ...");

        let client_to_server = transfer_encrypt(&mut lr, &mut rw);
        let server_to_client = transfer_decrypt(&mut rr, &mut lw);

        // if use_tokio {
        tokio::select! {
            _ = client_to_server => {println!("close client to vps server.");} ,
            _ = server_to_client => {println!("close vps server to client.");} ,
        };
        // } else {
        // future::try_join(client_to_server, server_to_client)
        //     .await
        //     .unwrap();
        //}
    } else {
        // vps server
        //println!("gfw_realy server begin ...");
        let client_to_server = transfer_decrypt(&mut lr, &mut rw);
        let server_to_client = transfer_encrypt(&mut rr, &mut lw);
        // future::try_join
        // if use_tokio {
        tokio::select! {
            _ = client_to_server => { println!("close vps_server to squid_server."); } ,
            _ = server_to_client => { println!("close squid_server to vps server."); } ,
        };
        // } else {
        // future::try_join(client_to_server, server_to_client)
        //     .await
        //     .unwrap();
        // }
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

    loop {
        //println!("transfer encrypt ...");
        // read incoming data from reader
        // not working on this: Box::new(vec![]);

        //let mut buffer: Vec<u8>= vec![];
        //while let Some(s) = reader.read_buf(buf).next().await {

        let mut buf = vec![0u8; BUFFER_MAX].into_boxed_slice();

        let data_size = match reader.read(&mut buf).await {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => break,
        }; //.unwrap();

        // encrypt data from incoming stream
        let cipher_data = gfw_encrypt_all(cipher, &key, &buf[..data_size]);
        writer.write_all(&cipher_data).await.unwrap();
    }
    Ok(0)
}

pub async fn transfer_decrypt<'a, R, W>(reader: &'a mut R, writer: &'a mut W) -> io::Result<u64>
where
    R: AsyncRead + Unpin + ?Sized,
    W: AsyncWrite + Unpin + ?Sized,
{
    let cipher = gfw_get_cipher();
    let key = gfw_get_key();

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

        if cipher_size > 0 {
            // read cipher data buffer
            let mut cipher_buffer = vec![0u8; cipher_size];
            println!(
                "reading cipher data... {}",
                String::from_utf8_lossy(&header_text)
            );
            let cipher_data_size = reader.read_exact(&mut cipher_buffer).await.unwrap();
            assert_eq!(cipher_data_size, cipher_size);

            println!(
                "\n noise size:{:>8} \ncipher size:{:>8} \n  read size:{:>8}",
                noise_size, cipher_size, cipher_data_size
            );

            let data = gfw_decrypt_data(cipher, &key, &cipher_buffer);
            println!("ready to write_all");
            writer.write_all(&data).await.unwrap();
            println!("done write_all");

            // noise size:      992
            // cipher size:     648
            //   read size:     648
            // decrypt data size (632) bytes, [23, 3, 3, 2, 115, 230, 238, 30] ... [102, 146, 240, 205, 141, 21, 109, 255]
            // ready to write_all
            // thread 'tokio-runtime-worker' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', /home/a14248/project/rust/gfw.press.rust/gfw_library/src/gfw_proxy.rs:173:43
            // reading cipher data... waitting for data ...
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
