This is a tracking file for study rust ...

[proxy relay function from](https://github.com/icodesign/proxy-relay/)

# Examples: using tokio::io;;copy_buf

```rust
use tokio::io;

let mut reader: &[u8] = b"hello";
let mut writer: Vec<u8> = vec![];

io::copy_buf(&mut reader, &mut writer).await?;

assert_eq!(b"hello", &writer[..]);
```

# [Read from tokio::io](https://skyao.io/learning-tokio/docs/tutorial/io.html)
```rust
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    // Return value of `Ok(0)` signifies that the remote has
                    // closed
                    Ok(0) => return,
                    Ok(n) => {
                        // Copy the data back to socket
                        if socket.write_all(&buf[..n]).await.is_err() {
                            // Unexpected socket error. There isn't much we can
                            // do here so just stop processing.
                            return;
                        }
                    }
                    Err(_) => {
                        // Unexpected socket error. There isn't much we can do
                        // here so just stop processing.
                        return;
                    }
                }
            }
        });
    }
}

```