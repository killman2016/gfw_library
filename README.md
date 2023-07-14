## This is a rust version library for [gfw.press](https://gfw.press) 
gfw press proxy library with different encrypt/decrypt format

Reference: thanks to following two projects:

1. [tcp-relay-rust](https://crates.io/crates/tcp-relay-rust)

2. [proxy relay](https://github.com/icodesign/proxy-relay/tree/tokio0.2)

## gfw.press.rust http proxy relay Workflow: 

[browser] <-> [gfw_client:13128] <-- internet --> [gfw_server:13128]  <-> [squid:3128] <-> [destination]

```bash
# test http proxy use curl:
curl -v -x http://127.0.0.1:13128 -L https://www.google.com/
```

## gfw.press.rust socks5 proxy relay workflow:

[broser] <-> [sslocal:8838] <-> [gf_client:18838] <-- internet --> [gfw_server:18838] <-> [ssserver:8838] <-> [destination]

```bash
# test socks5 proxy use curl:
curl -v -x socks5h://localhost:8838 -L https://www.google.com/
```

# GFW.Press Client Code Example:

```rust
#[tokio::main]
async fn main() {
    // act as a local proxy server
    let local_proxy_server = true;
    gfw_library::gfw_proxy::gfw_press_proxy(local_proxy_server).await;
}
```
client config json for sslocal:

```json
{
    "local_address": "127.0.0.1",
    "local_port":8838,
    "protocol":"socks",
    "server":"127.0.0.1",
    "server_port":18838,
    "password":"password",
    "timeout":7200,
    "method":"aes-256-cfb",
    "fast_open": false
}
```

client config json for gfw_client

```json
{
	"is_local_proxy":true,
    "is_socks5_mode": true,
	"http_server":"127.0.0.1:13128",
	"http_forward_server":"ip_address:13128",
	"socks5_server":"127.0.0.1:18838",
	"socks5_forward_server":"ip_address:18838",
	"password":"password"
}
```
# GFW.Press Server Code Example:

```rust
#[tokio::main]
async fn main() {

    // act as proxy on VPS server
    let local_proxy_server = false; 
    gfw_library::gfw_proxy::gfw_press_proxy(local_proxy_server).await;
}
```

server config json for gfw_server
```json
{
	"is_local_proxy":false,
    "is_socks5_mode": true,
	"http_server":"ip_address:13128",
	"http_forward_server":"127.0.0.1:3128",
	"socks5_server":"ip_address:18838",
	"socks5_forward_server":"127.0.0.1:8838",
	"password":"password"
}
```

server config json for ssserver
```json
{
    "local_address": "127.0.0.1",
    "local_port":8838,
    "protocl":"socks",
    "server":"127.0.0.1",
    "server_port":8838,
    "password":"password",
    "timeout":7200,
    "method":"aes-256-gcm",
    "fast_open": false
}
```