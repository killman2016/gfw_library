# This is a rust version library for [gfw.press](https://gfw.press) 

gfw press proxy server:

pls reference to following to project:

1. [tcp-relay-rust](https://crates.io/crates/tcp-relay-rust)

2. [proxy relay](https://github.com/icodesign/proxy-relay/tree/tokio0.2)

## gfw.press.rust http proxy relay Workflow: 

[browser:8080] <-> [gfw_client:13128] <- internet -> [gfw_server:13128]  <-> [squid:3128] <-> [destination]

```bash
# test use curl:
curl -v -x http://127.0.0.1:8080 https://www.google.com/
```

## gfw.press.rust socks5 proxy realy workflow:

[broser:1080] <-> [sslocal:1080] <-> [gf_client:13128] <- internet -> [gfw_server:13128] <-> [ssserver:8838] <-> [destination]

```bash
# test use curl:
curl -v -x socks5h://localhost:1080 https://www.google.com/
```

# GFW.Press Client Code Examle:

```rust
#[tokio::main]
async fn main() {
    let server = String::from("127.0.0.1:13128");
    let forward_server = String::from("ip_address:13128");

    println!("start local server: {}", &server);
    println!("connect to remote server: {}", &forward_server);

    // act as a local proxy server
    let up = true;
    gfw_library::gfw_proxy::gfw_press_proxy(server, forward_server, up).await;
}
```

# GFW.Press Server Code Example:

```rust
#[tokio::main]
async fn main() {
    let server = String::from("ip_address:13128"); // IPv4 only 
    // let server : &str = "[::]:13128"; // Both IPv4 & IPv6 (Linux dual stack only)
    // let forward_server = String::from("127.0.0.1:3128"); // forward to VPS squid http proxy server
    let forward_server = String::from("127.0.0.1:8388"); // forward to VPS shadowsocks socks5 server

    println!("start remote (VPS) proxy server: {}", &server);
    println!("connect to forward server: {}", &forward_server);

    // act as proxy on VPS server
    let up_or_down = false; 
    gfw_library::gfw_proxy::gfw_press_proxy(server, forward_server, up_or_down).await;
}
```