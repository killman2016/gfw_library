
Debian Cloudflare WARP packages [link](https://one.one.one.one/)
```bash
# Add cloudflare gpg key
curl https://pkg.cloudflareclient.com/pubkey.gpg | sudo gpg --yes --dearmor --output /usr/share/keyrings/cloudflare-warp-archive-keyring.gpg


# Add this repo to your apt repositories
echo "deb [arch=amd64 signed-by=/usr/share/keyrings/cloudflare-warp-archive-keyring.gpg] https://pkg.cloudflareclient.com/ $(lsb_release -cs) main" | sudo tee /etc/apt/sources.list.d/cloudflare-client.list


# Install
sudo apt-get update && sudo apt-get install cloudflare-warp

```

## Using WARP

The command line interface is the primary way to use WARP.

### [​​](https://developers.cloudflare.com/warp-client/get-started/linux/#initial-connection) Initial Connection have issue on VPS server. only used on client side....

To connect for the very first time you must call `register` first:

1. Register the client `warp-cli register`.
2. Connect `warp-cli connect`.
3. Run `curl https://www.cloudflare.com/cdn-cgi/trace/` and verify that `warp=on`.

### [​​](https://developers.cloudflare.com/warp-client/get-started/linux/#switching-modes) Switching modes

You can use `warp-cli set-mode --help` to get a list of the modes to switch between. For example:

- **DNS only mode via DoH:** `warp-cli set-mode doh`.
- **WARP with DoH:** `warp-cli set-mode warp+doh`.

### [​​](https://developers.cloudflare.com/warp-client/get-started/linux/#using-1111-for-families) Using 1.1.1.1 for Families

The Linux client supports all 1.1.1.1 for Families modes, in either WARP on DNS-only mode:

- **Families mode off:** `warp-cli set-families-mode off`
- **Malware protection:** `warp-cli set-families-mode malware`
- **Malware and adult content:** `warp-cli set-families-mode full`

### [​​](https://developers.cloudflare.com/warp-client/get-started/linux/#additional-commands) Additional commands

A complete list of all supported commands can be found by running:

  

```bash
~$ warp-cli --help

~$ warp-cli set-mode --help

# Set the mode
# 
# Usage: warp-cli set-mode <MODE>
# 
# Arguments:
#   <MODE>  [possible values: warp, doh, warp+doh, dot, warp+dot, proxy, tunnel_only]
# 
# Options:
#   -h, --help  Print help
```

Protecting yourself against [malware with 1.1.1.1 for Families](https://blog.cloudflare.com/introducing-1-1-1-1-for-families/) is just as easy, and it can be used with either WARP enabled or in straight DNS mode:

```
~$ warp-cli set-families-mode --help
warp-cli-set-families-mode 
 
USAGE:
    warp-cli set-families-mode [mode]
 
FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
 
ARGS:
    <mode>     [possible values: off, malware, full]
~$ warp-cli set-families-mode malware
Success
```