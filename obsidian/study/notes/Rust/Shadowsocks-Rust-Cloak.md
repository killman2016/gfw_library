# 如何部署Shadowsocks-rust和Cloak

## 前言

无庸讳言，Shadowsocks是一款优异的穿墙利器。首先，作为开源项目，用户无需为其软件付费，只需要承担租赁虚拟主机的硬件成本。其次，该软件小巧灵活，布署简单，非常适合个人用户独立架设使用。也因此不用涉及虚拟主机之外的任何其他第三方的网络服务提供商，在数据安全方面更有保证。

现在网上常见的shadowsocks在Ubuntu上的部署教程大多基于shadowsocks-libev。问题在于，shadowsocks-libev已停止开发，仅仅处于维护状态，相关开发已移向shadowsocks-rust。正如在其github的[README文件](https://github.com/shadowsocks/shadowsocks-libev)中说的：

> Bug-fix-only libev port of shadowsocks. Future development moved to shadowsocks-rust

抵抗网络封锁是一场需要不断进行技术升级的猫鼠游戏。在近日的使用中，我发现shadowsocks-libev已越来越频繁地出现服务中断、需要重启的状况。是时候迁移到Shadowsocks的下一代port——Shadowsocks-rust了！

但是网上适合用户难度的、反映shadowsocks-rust完整部署步骤的教程还付之阙如。经过一些尝试，我成功地部署了Shadowsocks-rust。现将全部部署过程分享出来，以方便其他和我有相似需要的用户使用。在这个过程中，我参考了多个老版Shadowsocks的教程，Shadowsocks-rust的官方文件，以及各类相关技术教程和技术问答帖。这里一并致谢！

另外，为解决由于active probing导致的IP被封问题，本教程采用了基于Cloak的解决方案。特向其作者[Andy Wang](https://github.com/cbeuw)表示感谢！

## 设置临时代理

一开始我们需要处理这样一个让人挠头的问题：为了实现翻墙，我们首先需要安装Shadowsocks和Cloak，然而为了能成功安装这些软件，我们首先又需要能够翻墙。这就陷入了一个死循环。好在天无绝人之路。

只要你在墙外有ubuntu的云端服务器，而且可以使用ssh进行远程访问，就可以方便地[通过ssh建立socks5代理](https://ma.ttias.be/socks-proxy-linux-ssh-bypass-content-filters/)。

比如你设定socks5代理的本地端口为12345，则相关命令为：

```
ssh -D 12345 -q -C -N <usename@RemoteSeverIP>
```

其中`-D 12345`在本地端口12345打开socks5端口。`-c`压缩数据以节省带宽。`-q`开启安静模式。`-N`不执行远程命令，仅用于forwarding ports。

则可以对各命令冠以`https_proxy=socks5://127.0.0.1:12345 https_proxy=socks5://127.0.0.1:12345` 以满成所有相关程序的下载。

注意上面的端口值是任意选定的，可以根据你的需要修改为其他数值。

## 安装Shadowsocks

### 预安装cargo

根据[官方推荐](https://github.com/shadowsocks/shadowsocks-rust)，我们使用cargo来安装shadowsocks-rust。因此需要提前安装[rustup](https://snapcraft.io/install/rustup/ubuntu)：

```
sudo snap install rustup --classic
```

然后，为正常使用cargo，需要安装和设置rust的[stable toolchain](https://stackoverflow.com/questions/44303915/no-default-toolchain-configured-after-installing-rustup):

```
rustup install stable
rustup default stable
```

### 安装Shadowsocks-rust

假定在服务器端和本地端都安装Ubuntu18.04以上系统。则服务器端和本地端的安装方法是完全一致的。也就是说二者安装的是同样一份软件，只是在使用过程中调用不同的命令。

```
cargo install shadowsocks-rust
```

安装完成后，用于服务器端的ssserver命令和用于本地端的sslocal命令就都位于~/.cargo/bin/文件夹下了。

## 安装Cloak

为了解决虚拟主机因GFW的active probing而反复被封的问题，我们需要给Shadowsocks的数据流加个“外罩”，伪装成访问没有被墙的“正常”网站的样子。因而我们需进一步安装[Cloak插件](https://github.com/cbeuw/Cloak)。我们需要在服务器端和本地端都安装Cloak，安装方法完全一样：

### 预安装go

```
snap install go --classic
```

### 安装cloak

```
git clone https://github.com/cbeuw/Cloak
cd Cloak
go get ./...
make
```

安装完成后可以在/Cloak/build/文件夹内找到两个可执行程序：ck-client和ck-server，分别在本地端和服务器端调用。

如前所述，在本地端安装时，往往需要提前使用代理。在上述命令中，可在git clone和go get前冠以https_proxy=socks5://127.0.0.1:12345。其中127.0.0.1:12345为代理的本地地址和端口，需要根据你本地系统的设置而改动。

上述代码的另一个问题是，现在github即使对克隆代码也要求先登录，因此第一行代码调整为：

```
git clone https://username:password@github.com/cbeuw/Cloak
```

## 编辑配置文件

### Shadowsocks服务器端的配置文件

```
{
    "server": "my_server_ip",
    "server_port": 8388,
    "password": "mypassword",
    "method": "chacha20-ietf-poly1305",
    // 以下两行给出服务器端Cloak插件及其配置文件的路径
    "plugin": "<MyPathTo>/Cloak/build/ck-server",
    "plugin_opts": "<MyPathTo>/ckserver.json"
    // 仅用于 `sslocal`
    // 当运行 `ssserver` 时删除以下两行
    // "local_address": "127.0.0.1",
    // "local_port": 1080
}
```

依据[gfw.report](https://gfw.report/blog/ss_tutorial/en/)的建议，在上述配置中，我们使用较强的chacha20-ietf-poly1305加密方法，以抵抗GFW的主动探测。同时，我们也需要设置更复杂的密钥以防被GFW猜中。可以用以下命令生成复杂密码：

```
openssl rand -base64 16
```

“server_port”可设置为不同于8388的其他数，这个数需介于1024和65535之间。

上述配置文件中的plugin和plugin_op分别指向服务器端的cloak插件和cloak插件的配置文件。

最后，把上述配置文件保存为json文件。可把文件保存在任意位置。可任意指定文件名，比如config.json。

### Cloak服务器端的配置文件

ckserver.json为Cloak在服务器端的配置文件。其内容如下：

```
{
    "ProxyBook": {
        "shadowsocks": [
            "tcp",
            "127.0.0.1:8388"
        ]
    },
    "BindAddr": [
        ":443",
        ":80"
    ],
    "BypassUID": [
        "User ID here"
    ],
    "RedirAddr": "www.bing.com",
    "PrivateKey": "Private key here",
    "DatabasePath": "userinfo.db"
}
```

上述设置中的PrivateKey来自ck-server命令产生的一对公钥和私钥：

```
ck-server -key
```

其中私钥写入Cloak的服务器端的配置文件，公钥写入Cloak的本地端的配置文件。

BypassUID为本地端（客户端）的用户ID，由如下命令产生：

```
ck-server -uid
```

可由上述命令生成多个用户ID。只有写入上述服务器端的配置文件的用户ID才会取得访问授权。配置文件中可以列入多个用户ID。

上述配置文件中的RedirAddr是Cloak假扮的被访问外网网站。这个网站应为国内可访问的大站。如bing.com。

### Shadowsocks本地端的配置文件

SS本地端的配置文件与服务器端类似。但应将plugin调整为本地端命令，并增加local_address和local_port两项。

```
{
    "server": "my_server_ip",
    "server_port": 8388,
    "password": "mypassword",
    "method": "chacha20-ietf-poly1305",
    // 以下两行给出本地端Cloak插件及其配置文件的路径
    "plugin": "<MyPathTo>/Cloak/build/ck-client",
    "plugin_opts": "<MyPathTo>/ckclient.json",
    // 仅用于 sslocal
    // 当运行 ssserver 时删除以下两行
    "local_address": "127.0.0.1",
    "local_port": 1080
}
```

### Cloak本地端（客户端）的配置文件

ckclient.json是Cloak的linux系统本地端配置文件。

```
{
    "Transport": "direct",
    "ProxyMethod": "shadowsocks",
    "EncryptionMethod": "plain",
    "UID": "User ID here",
    "PublicKey": "Public key here",
    "ServerName": "www.bing.com",
    "AlternativeNames": ["cloudflare.com", "github.com"],
    "NumConn": 4,
    "BrowserSig": "chrome",
    "StreamTimeout": 300
}
```

其中UID和PublicKey分别是服务器端命令生成的用户名和公钥。AlternativeNames指定其他伪装网址。BrowserSig指定伪装的流览器类型。

## 服务器端防火墙设置

我们使用ufw来为Shadowsocks服务器开设端口。首先在Ubuntu上安装ufw：

```
sudo apt update && sudo apt install -y ufw
```

我们需要为ssh和Shadowsocks-rust开设端口：

```
sudo ufw allow ssh
sudo ufw allow 8388
```

注意要使用你前面设置的端口值来替换8388。

然后启用ufw：

```
sudo ufw enable
```

## 运行程序

现在就可以在命令行运行shadowsocks-rust了。

**服务器端**

```
~/.cargo/bin/ssserver -c /<mypathto>/config.json
```

**本地端**

```
~/.cargo/bin/sslocal -c /<mypathto>/config.json
```

在这个过程中Cloak作为插件程序也会被自动调用。

## 在本地端的程序中使用代理

现在你就可以在本地端的具备socks5代理选项的应用里（比如Firefox）测试Shadowsocks-rust使否成功了。本地端软件的设置可参考[这里](https://www.linuxbabe.com/ubuntu/shadowsocks-libev-proxy-server-ubuntu)。值得一提的是，可以使用tsocks为命令行软件填加通过socks5访问网络的功能。首先安装tsocks:

```
sudo apt install tsocks
```

需要修改tsocks的配置文件，该文件位于/etc/tsocks.conf。将其中server的ip修改为我们前面设置的本地端地址。即将文件中的：

```
server = 192.168.0.1
```

更改为：

```
server = 127.0.0.1
```

现在，只要在相关命令前冠以tsocks就可以使用网络代理了。经我实践检验，tsocks并不对每个命令行程序都有用。所幸，有些命令行程序有自己的socks5代理选项。比如Python的package管理程序pip可以这样使用socks5:

```
python3 -m pip install --user <LocalFile.tar.gz> --proxy socks5://127.0.0.1:1080
pip3 install <MyPackage> --proxy socks5://127.0.0.1:1080
```

### Android系统中的本地端程序

在Android端不仅要安装Shadowsocks客户端，还要安装Cloak插件。Shadowsocks客户端可在google play下载安装，Cloak插件的apk文件可在[这里](https://github.com/cbeuw/Cloak-android/releases)下载安装。

### Windows系统中的本地端程序

首先下载[Shadowsocks-windows](https://github.com/shadowsocks/shadowsocks-windows/releases),相关设置可参考[这里](https://centixkadon.github.io/b/app/shadowsocks/client/)。然后下载Cloak的[Windows客户端](https://github.com/cbeuw/Cloak/releases)。相关的设置方法参考[这里](https://github.com/cbeuw/Cloak/wiki/Underlying-proxy-configuration-guides#client-1)：即在图形界面的客户端的设置中，“插件程序”一栏填入下载的.exe程序的路径，“插件选项”一栏填入前面设置过的ckclient.json文件的路径。

## 启用服务

上述在命令行里调用ssserver和sslocal命令的方法虽然简单直接，但存在一些弊端：

- 因为程序不在后台执行，因此会占据命令行资源。
    
- 更重要的，每此重启服务器端和本地端都需要重新启动程序，费时费力费神费心。
    

为此，依据[这里](https://github.com/shadowsocks/shadowsocks-rust/issues/103)的方法，可以填加shadowsocks服务。首先，无论是在服务器端还是本地端，需在/usr/local/systemd/system/或/lib/systemd/system/中加入配置文件。服务器端的配置文件ssserver.service：

```
[Unit]
Description=ssserver service
After=network.target

[Service]
ExecStart=~/.cargo/bin/ssserver -c /<mypathto>/config.json
ExecStop=/usr/bin/killall ssserver
Restart=always
RestartSec=10                       # Restart service after 10 seconds if service crashes
StandardOutput=syslog               # Output to syslog
StandardError=syslog                # Output to syslog
SyslogIdentifier=ssserver

[Install]
WantedBy=multi-user.target
```

本地端的配置文件sslocal.service：

```
[Unit]
Description=sslocal service
After=network.target

[Service]
ExecStart=~/.cargo/bin/sslocal -c /<mypathto>/config.json
ExecStop=/usr/bin/killall sslocal
Restart=always
RestartSec=10                       # Restart service after 10 seconds if service crashes
StandardOutput=syslog               # Output to syslog
StandardError=syslog                # Output to syslog
SyslogIdentifier=sslocal

[Install]
WantedBy=multi-user.target
```

然后，分别在服务器端和本地端启用服务：

```
sudo systemctl start ssserver.service 
```

```
sudo systemctl start sslocal.service
```

为了使机器重启后上述服务依然运行，分别在服务器端和本地端分别运行如下命令：

```
sudo systemctl enable ssserver.service
```

```
sudo systemctl enable sslocal.service
```

然后，可以分别以如下命令查看服务的状态：

```
sudo systemctl status ssserver.service
```

```
sudo systemctl status sslocal.service
```

如果状态显示为active，说明服务启用成功了。

## 设置备用端口

~~依据[gfw.report](https://gfw.report/blog/ss_tutorial/en/)的报告，不讲武德的GFW开始进行封端口的操作了。如果仅有一个可用端口，每次被封后就需在服务器端进行更改，重启服务器，并对所有本地端口进行相应更改。[gfw.report](https://gfw.report/blog/ss_tutorial/en/)提出的办法是把TCP和UDP在其他端口的流量重新定向到Shadowsocks的服务器端口上。这样每次被封后只需在本地端换一个端口就行了。在服务器端运行命令如下：~~

```
sudo iptables -t nat -A PREROUTING -p tcp --dport 1024:65535 -j REDIRECT --to-port 8388 
```

```
sudo iptables -t nat -A PREROUTING -p udp --dport 1024:65535 -j REDIRECT --to-port 8388
```

~~可通过如下命令查看相关设置：~~

```
sudo iptables -t nat -L PREROUTING -nv --line-number
```

~~为在重启后仍保持上述iptables设置，我们可按照[这里](https://linuxconfig.org/how-to-make-iptables-rules-persistent-after-reboot-on-linux)的方法安装使用iptables-persistent包。首先安装：~~

```
sudo apt install iptables-persistent
```

~~然后运行~~

```
sudo iptables-save > /etc/iptables/rules.v4
sudo ip6tables-save > /etc/iptables/rules.v6
```

~~如此可将设置永久分别保存在rules.v4和rules.v6文件中。~~

## 后记

经过一天的使用，我发现Shadowsocks-rust与Shadowsocks-libev相比，更加稳定，很少掉线。在使用中如果有新的情况值得注意，我会更新在这里。

====================================

2023.4.7

0点许，IP被封。重新布署。这是Shadowsocks的一个老问题了。 但一个IP能存活多久很不一定。在被封前，手机一直发出Shadowsocks客户端反复重启，耗电过多的提醒。

====================================

2023.4.12

IP又被封。IP频繁被封可能是由于Shadowsocks-rust本身没有防范active probing的功能。而据[gfw.report](https://gfw.report/blog/ss_tutorial/en/)，Shadowsocks-libev的较新版本有此功能。这也解释了我之前布署的Shadowsocks-libev一直没有被封IP的原因。经过一番搜索，我决定通过给Shadowsocks-rust加[外罩](https://github.com/cbeuw/Cloak)的方式来解决这一问题。我会更新整个文档以反映这个变化。

====================================

2023.5.17

已平稳运行月余。

今天通过ssh设置临时代理的方式，实现了在无初始代理的本地端机器上的成功部署。但这会带来active probing的风险。究竟后续如何，我们拭目以待。