# [debian 安装 Shadowsocks-Rust](http://www.gonewto.com/?post/d4rnsx)

by 神秘人 at 2022-11-30

此处以debian linux-amd64 为例

1. 服务器端

1.1. 利用程序随机产生端口及密度

```
 echo $((1024 + $RANDOM))
```

利用上面随机产生数字，作为程序服务器的端口:

```
21429（随机产生， 不一定是这个数字）
```

1.2 得用下面程序随机程序高强度密码，长度为32，加密方式为：2022-blake3-chacha20-poly1305

```
# using openssl
openssl rand -base64 32
# or using ssservice
ssservice genkey -m 2022-blake3-chacha20-poly1305
```

利用上面随机产生数字，作为密码

```
# （随机产生， 不一定是这个数字）
Qi0n04pcO38SFROxnIspyE0WRwwMjVEf
# or using ssservice
3gCHhhHx20WmMpCTQVdtGOjTuIQ4Um+qKfJPGnXcSXg=
```

安装tar -xf解压工具

```
 sudo apt-get install xz-utils
```

2.开始安装 Shadowsocks-Rust Binary

[Shadowsocks-Rust下载](https://github.com/shadowsocks/shadowsocks-rust/releases)

将Shadowsocks-Rust下到载服务器（选择服务器版本一般是x86_64-unknown-linux-gnu.tar.xz）

```
  wget   https://github.com/shadowsocks/shadowsocks-rust/releases/download/v1.15.0-alpha.9/shadowsocks-v1.15.0-alpha.9.x86_64-unknown-linux-gnu.tar.xz
```

解压程序

```
 tar -xf  shadowsocks-v1.15.0-alpha.9.x86_64-unknown-linux-gnu.tar.xz
```

复制 Shadowsocks 服务器代码 到 /usr/local/bin:

```
 cp ssserver /usr/local/bin
```

配置 Shadowsocks 服务器设置

```
apt-get install vim 
```

创建文件 /etc/shadowsocks-rust.json

```
vim /etc/shadowsocks-rust.json
```

将下面模版复制到上面文件内容:

```
{
   "server": "0.0.0.0",
   "server_port": 21429,
   "password": "Qi0n04pcO38SFROxnIspyE0WRwwMjVEf",
   "timeout": 300,
   "method": "2022-blake3-chacha20-poly1305",
   "mode": "tcp_only",
   "fast_open": false
}
```

上面模版要改换的地方

```
 server port ：21429，将21429换成上面随机端口产生的数字

"password": "Qi0n04pcO38SFROxnIspyE0WRwwMjVEf"，将“”内的数段换成上面随字产生的密码。
```

3，创建 SystemD Service File（目的是保证程序自动运行）

创建 /usr/lib/systemd/system/shadowsocks-rust.service

```
vim /usr/lib/systemd/system/shadowsocks-rust.service
```

复制下面模版到上面文件内容:

```
[Unit]
Description=shadowsocks-rust service
After=network.target

[Service]
ExecStart=/usr/local/bin/ssserver -c /etc/shadowsocks-rust.json
ExecStop=/usr/bin/killall ssserver
Restart=always
RestartSec=10
StandardOutput=syslog
StandardError=syslog
SyslogIdentifier=ssserver
User=nobody
Group=nogroup

[Install]
WantedBy=multi-user.target
```

开机自动运行 Shadowsocks-Rust Server

```
systemctl enable shadowsocks-rust
```

启动 Shadowsocks-Rust Server

```
 systemctl start shadowsocks-rust
```

检查 Shadowsocks-Rust 是否激活并运行:

```
systemctl status shadowsocks-rust
```

---

设置vps服务器防火墙

安装ufw防火墙

```
apt update && apt install -y ufw
```

设置开放端口（8388改成你的ss端口，其他你需要开启的端口也都打开）

```
ufw allow ssh

ufw allow 8388
```

设置ufw开机启动

```
ufw enable
```

查看ufw防护墙状态

```
ufw status
```