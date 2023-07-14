### Rust Cargo Compile and Installation

看到有友分享RUST语言书籍，于是想到了基于rust的Shadowsocks2022，下面就来分享一下rust语言之下的shadowsocks编译，搭建教程。

首先安装一下gcc工具链：

`apt install build-essential`

### 安装rust

`curl https://sh.rustup.rs -sSf | sh`

运行脚本时，选择 1 默认安装

安装好 rust 后：  
`source $HOME/.cargo/env`

检查  
`cargo --version`  
`rustc -V`

### 编译 Shadowsocks-rust

```bash
git clone https://github.com/shadowsocks/shadowsocks-rust.git
cd shadowsocks-rust
export SODIUM_USE_PKG_CONFIG=1
```

检查 cargo  
`cargo check`

编译  
`cargo build --release`

安装到默认路径

`make install TARGET=release`

### 配置文件

Shadowsocks-rust 版和 libev 版配置文件是一样的，因为新版的AEAD 2022 Ciphers加密，需要base64生成的密码，所以需要先生成密码：

`openssl rand -base64 32`

上面生成的密码支持 `2022-blake3-aes-256-gcm` 和 `2022-blake3-chacha20-poly1305` 两种加密协议，所以下面配置文件的加密协议可选择上面两种。

新建 shadowsocks 文件夹及 config.json 配置文件：

`mkdir -p /etc/shadowsocks`  
`vi /etc/shadowsocks/config.json`

配置文件如下：

```json
{
    "server":"0.0.0.0",
    "server_port":1024,
    "password":"填写上面生成的密码",
    "timeout":600,
    "method":"2022-blake3-chacha20-poly1305"
}
```

### 使用 systemd 守护进程

`vi /etc/systemd/system/shadowsocks.service`

写入内容如下：

```ini
[Unit]
Description=Shadowsocks Server
After=network.target

[Service]
ExecStart=/usr/local/bin/ssserver -c /etc/shadowsocks/config.json

Restart=on-abort

[Install]
WantedBy=multi-user.target
```

下面重载一下systemd服务，启动shadowsocks就好了。

`systemctl daemon-reload` //Systemctl重载

`systemctl start shadowsocks` //启动

`systemctl enable shadowsocks` //添加开机自启动

`systemctl status shadowsocks` //查看状态

---
# Installation Only

上次说过 [Shadowsocks-rust 编译搭建](https://1024.day/d/1521)，这次说说简单搭建方法。

因为官方已经编译好了rust二进制文件，只要下载解压，写好配置文件就可以用了，所以这次简单很多。

官方GitHub：[https://github.com/shadowsocks/shadowsocks-rust](https://github.com/shadowsocks/shadowsocks-rust)

### 下载解压

下载解压到运行目录，一个命令搞定：

`wget https://github.com/shadowsocks/shadowsocks-rust/releases/download/v1.15.3/shadowsocks-v1.15.3.x86_64-unknown-linux-gnu.tar.xz -O - | tar -xJ -C /usr/local/bin/`

`chmod +x /usr/local/bin/*`

### 配置文件

```bash
mkdir -p /etc/shadowsocks
vi /etc/shadowsocks/config.json
```

配置文件如下：

```json
{
    "server":"0.0.0.0",
    "server_port":1024,
    "password":"填写密码",
    "timeout":600,
    "method":"chacha20-ietf-poly1305"
}
```

### 使用 systemd 守护进程

`vi /etc/systemd/system/shadowsocks.service`

写入内容如下：

```ini
[Unit]
Description=Shadowsocks Server
After=network.target

[Service]
ExecStart=/usr/local/bin/ssserver -c /etc/shadowsocks/config.json

Restart=on-abort

[Install]
WantedBy=multi-user.target
```

下面重载一下systemd服务，启动shadowsocks就好了。

`systemctl daemon-reload` #Systemctl重载

`systemctl start shadowsocks` #启动

`systemctl enable shadowsocks` #添加开机自启动

`systemctl status shadowsocks` #查看状态