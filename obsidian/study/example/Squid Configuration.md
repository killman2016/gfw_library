```
# /etc/squid/squid.conf
# forward to another proxy server
# cache_peer inside.fw.address.domain parent 13128 0 default no-query

cashe_peer 127.0.0.1 parent 13128 0 default no-query
acl homenet src 192.168.1.0/28 # Home network address
http_access allow homenet

acl CONNECT method CONNECT # allow hppts connect to squid proxy
```

Squid Command Line

```bash
sudo squid -k parse # parse configuration
sudo squid -k reconfigurate # re-load configuration
```

IP address Subneting

| Subnet | Network | First Usable IP | Last Usable IP | Broadcast IP|CIDR Block example|
|---------|------------|------------|-------------|-----------|---|
| 0       | 192.168.1.0 | 192.168.1.1 | 192.168.1.14 |192.168.1.15 | 192.168.1.0/28|
|9 |192.168.18.144 |192.168.18.145 |192.168.18.158  |192.168.18.159| 192.168.18.144/28|

[Example](https://www.exampointers.com/sub.php)

|**CIDR block**|**IP range (network - broadcast)**|**Subnet Mask**|**IP Quantity**|
|---|---|---|------|
|192.168.1.0/32|192.168.1.0 - 192.168.1.0|255.255.255.255|1 = $2^0 = 2^{(32-32)}$|
|192.168.1.0/31|192.168.1.0 - 192.168.1.1|255.255.255.254|2 = $2^1 = 2^{(32-31)}$|
|192.168.1.0/30|192.168.1.0 - 192.168.1.3|255.255.255.252|4|
|192.168.1.0/29|192.168.1.0 - 192.168.1.7|255.255.255.248|8|
|192.168.1.0/28|192.168.1.0 - 192.168.1.15|255.255.255.240|16|
|192.168.1.0/27|192.168.1.0 - 192.168.1.31|255.255.255.224|32|
|192.168.1.0/26|192.168.1.0 - 192.168.1.63|255.255.255.192|64|
|192.168.1.0/25|192.168.1.0 - 192.168.1.127|255.255.255.128|128 = $2^7 = 2^{(32-25)}$|
|192.168.1.0/24|192.168.1.0 - 192.168.1.255|255.255.255.0|256 = $2^8 = 2^{(32-24)}$|

Question: Why squid does not work with HTTPS sites?

At the simplest level you've not defined any rules to allow HTTPS traffic to connect through squid. (At the more complex level you need to create trusted certificates so you can intercept and validate HTTPS traffic with `ssl-bump`. I'm not showing that here.)

```
acl CONNECT method CONNECT
acl SSL_ports port 443
acl Safe_ports port 443         # https
http_access deny CONNECT !SSL_ports
```

checking log file

`sudo tail -f /var/log/squid/access.log`