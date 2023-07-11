use openssl::sha::sha256;
use serde_derive::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    http_mode: bool,
    gfw_http_server: String,
    http_forward_server: String,
    local_or_remote: bool,
    gfw_socks5_server: String,
    socks5_forward_server: String,
    password: String,
}

impl Config {
    pub fn get_http_mode(&self) -> bool {
        self.http_mode
    }

    pub fn get_proxy_type(&self) -> bool {
        self.local_or_remote
    }

    pub fn get_server(&self) -> &str {
        if self.http_mode {
            self.gfw_http_server.as_str()
        } else {
            self.gfw_socks5_server.as_str()
        }
    }

    pub fn get_forward_server(&self) -> &str {
        if self.http_mode {
            self.http_forward_server.as_str()
        } else {
            self.socks5_forward_server.as_str()
        }
    }

    //get secrect key
    pub fn gfw_secrect_key(&self) -> [u8; 32] {
        let key = sha256(self.password.as_bytes());
        assert_eq!(key.len(), 32);
        key
    }
}
