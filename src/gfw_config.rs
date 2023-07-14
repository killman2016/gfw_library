use openssl::sha::sha256;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GFWConfig {
    is_local_proxy: bool,
    is_socks5_mode: bool,
    http_server: String,
    http_forward_server: String,
    socks5_server: String,
    socks5_forward_server: String,
    password: String,
}

impl GFWConfig {
    // http_mode = true; // http proxy
    // http_mode = false; // socks5 proxy
    pub fn is_socks5_mode(&self) -> bool {
        self.is_socks5_mode
    }

    // local_or_remote = true; // local proxy server
    // local_or_remote = false // proxy server on VPS
    pub fn is_local_proxy(&self) -> bool {
        self.is_local_proxy
    }

    // proxy server listening...
    pub fn get_server(&self) -> &str {
        if self.is_socks5_mode {
            self.socks5_server.as_str()            
        } else {
            self.http_server.as_str()
        }
    }

    // forward data to anohter server...
    pub fn get_forward_server(&self) -> &str {
        if self.is_socks5_mode {
            self.socks5_forward_server.as_str()
        } else {
            self.http_forward_server.as_str()           
        }
    }

    //get secrect key
    pub fn get_secrect_key(&self) -> [u8; 32] {
        //password key minimal lenght is 16
        assert!(self.password.len() >= 16);
        let key = sha256(self.password.as_bytes());
        assert_eq!(key.len(), 32);
        key
    }
}
