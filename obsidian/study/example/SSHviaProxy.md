How to establish an SSH connection via a proxy
[link](https://www.simplified.guide/ssh/connect-via-socks-proxy)

1. check port information:

`ss -natp`

OpenSSH's client supports connections via SOCKS and HTTPS proxies.
To establish a proxied connection, you'll need the ProxyCommand option
and third-party tools like nc or netcat.

2. check if the SOCKS or HTTPS proxy is reachable from the SSH client's host

`nc -zv 127.0.0.1 8838`
Connection to 127.0.0.1 8838 port [tcp/*] succeeded!

3. Use ProxyCommand as an option for the SSH client

`ssh -o ProxyCommand='nc -X5 -x 127.0.0.1:8838 %h %p' remoteuser@remotehost`

4. Add ProxyCommand to SSH client configuration file to persist the option

`$ cat .ssh/config`

Host remotehost
    hostname 192.168.1.10
    user remoteuser
    ProxyCommand nc -X5 -x 127.0.0.1:8838 %h %p

5. Connect again using the SSH client with just the Host name as parameter.

`$ ssh remotehost`
