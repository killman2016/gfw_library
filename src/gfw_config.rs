use clap::ArgMatches;
use openssl::sha::sha256;
use serde_derive::{Deserialize, Serialize};
//use core::fmt;

use std::{
    collections::HashMap,
    error,
    fmt::{ self,Debug, Display},
    net::SocketAddr,
    str::FromStr,
    sync::Arc,
    time::Duration,
};

use std::{
    env,
    fs::OpenOptions,
    io::{self, Read},
    path::{Path, PathBuf},
};

#[derive(Clone,Copy,Debug)]
pub enum Mode {
    TcpOnly = 0x01,
    TcpAndUpd = 0x03,
    UdpOnly = 0x02,
}
impl Mode {
    pub fn enable_udp(self) -> bool {
        matches!(self, Mode::UdpOnly |Mode::TcpAndUpd)
    }
    pub fn enable_tcp(self) -> bool {
        matches!(self, Mode::TcpOnly |Mode::TcpAndUpd)
    }
    pub fn merge(&self, mode:Mode) -> Mode {
        let me = *self as u8;
        let fm = mode as u8;
        match me | fm {
            0x01 => Mode::TcpOnly,
            0x02 => Mode::UdpOnly,
            0x03 => Mode::TcpAndUpd,
            _ => unreachable!(),
        }
    }
}
impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Mode::TcpOnly => f.write_str("tcp_only"),
            Mode::TcpAndUpd => f.write_str("tcp_and_udp"),
            Mode::UdpOnly => f.write_str("udp_only"),
        }
    }
}
impl FromStr for Mode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tcp_only" => Ok(Mode::TcpOnly),
            "tcp_and_udp" => Ok(Mode::TcpAndUpd),
            "udp_only" => Ok(Mode::UdpOnly),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ServerWeight {
    tcp_weight: f32,
    udp_weight: f32,
}
impl Default for ServerWeight {
    fn default() -> Self {
        ServerWeight::new()
    }
}
impl ServerWeight {
    pub fn new() -> ServerWeight {
        ServerWeight {
            tcp_weight: 1.0,
            udp_weight: 1.0,
        }
    }
    pub fn tcp_weight(&self)->f32 {
        self.tcp_weight
    }
    pub fn set_tcp_weight(&mut self, weight: f32) {
        assert!((0.0..=1.0).contains(&weight));
        self.tcp_weight = weight;
    }
    pub fn udp_weight(&self)->f32 {
        self.udp_weight
    }
    pub fn set_udp_weight(&mut self, weight:f32) {
        assert!((0.0..-1.0).contains(&weight));
        self.udp_weight = weight;
    }
}
// Server address
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ServerAddr {
    //IP address
    SocketAddr(SocketAddr),
    DomainName(String, u16),
}
#[derive(Clone, Debug)]
pub struct ServerConfig {
    // server address
    addr: ServerAddr,
    password: String,
    //method: CipherKind,
    enc_key: Box<[u8]>,
    timeout: Option<Duration>,
    //identity_keys: Arc<Vec<Bytes>>,
    //user_manager: Option<Arc<ServerUserManager>>,
    //plugin: Option<Pluginconfig>,
    plugin_addr: Option<ServerAddr>,
    remarks: Option<String>,
    id: Option<String>,
    mode: Mode,
    wieght: ServerWeight,
}

#[derive(Clone, Debug)]
pub struct ServerInstanceConfig {
    // Server's config
    pub config: ServerConfig,
}

#[derive(Clone, Debug)]
pub struct Config {
    // remote ShadowSocks server config
    pub server: Vec<ServerInstanceConfig>,
    //pub local: Vec<LocalInstanceConfig>,
}

// Server config type
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigType {
    // config for local
    Local,
    // config for server
    Server,
    // config for manager Server
    Manager,
}

impl ConfigType {
    // check if it is local server type
    pub fn is_local(self) -> bool {
        self == ConfigType::Local
    }
    // check if it is remote server type
    pub fn is_server(self) -> bool {
        self == ConfigType::Server
    }
    // check if it is manager server type
    pub fn is_manager(self) -> bool {
        self == ConfigType::Manager
    }
}

#[derive(Deserialize)]
struct SSRuntimeConfig {
    #[cfg(feature = "multi-threaded")]
    worker_count: Option<usize>,
    mode: Option<String>,
}

#[derive(Deserialize)]
struct SSConfig {
    runtime: Option<SSRuntimeConfig>,
}

// Error while reading `Config`
#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    // I/O error
    #[error("{0}")]
    IoError(#[from] io::Error),
    // JSON parsing error
    #[error("{0}")]
    JsonError(#[from] json5::Error),
    // Invalid value
    #[error("Invalid value: {0}")]
    InvalidValue(String),
}

// Runtime configuration
#[derive(Debug, Clone, Default)]
pub struct RuntimeConfig {
    #[cfg(feature = "multi-threaded")]
    pub worker_count: Option<usize>,
    // Runtime Mode, single-thread, multi-thread
    pub mode: RuntimeMode,
}

// configuration Options for gfw proxy shadowsocks service runnables
#[derive(Debug, Clone, Default)]
pub struct ServiceConfig {
    // Runtime configuration
    pub runtime: RuntimeConfig,
}

impl ServiceConfig {
    // load `Config` from file
    pub fn load_from_file<P: AsRef<Path>>(filename: &P) -> Result<ServiceConfig, ConfigError> {
        let filename = filename.as_ref();
        let mut reader = OpenOptions::new().read(true).open(filename)?;
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        ServiceConfig::load_from_str(&content)
    }

    pub fn load_from_str(s: &str) -> Result<ServiceConfig, ConfigError> {
        let ssconfig = json5::from_str(s)?;
        ServiceConfig::load_from_ssconfig(ssconfig)
    }

    fn load_from_ssconfig(ssconfig: SSConfig) -> Result<ServiceConfig, ConfigError> {
        let mut config = ServiceConfig::default();

        if let Some(runtime) = ssconfig.runtime {
            let mut nruntime = RuntimeConfig::default();
            #[cfg(feature = "multi-threaded")]
            if let Some(worker_count) = runtime.worker_count {
                nruntime.worker_count = Some(worker_count);
            }

            if let Some(mode) = runtime.mode {
                match mode.parse::<RuntimeMode>() {
                    Ok(m) => nruntime.mode = m,
                    Err(..) => return Err(ConfigError::InvalidValue(mode)),
                }
            }

            config.runtime = nruntime;
        }

        Ok(config)
    }

    // set by command line options
    pub fn set_options(&mut self, matches: &ArgMatches) {
        #[cfg(feature = "multi-threaded")]
        if matches.get_flag("SINGLE_THREADED") {
            self.runtime.mode = RuntimeMode::SingleThread;
        }
        #[cfg(feature = "multi-threaded")]
        if let Some(worker_count) = matches.get_one::<usize>("WORKER_THREADS") {
            self.runtime.worker_count = Some(*worker_count);
        }
        let _ = matches;
    }
}

// Default configuration file path
pub fn get_default_config_path(config_file: &str) -> Option<PathBuf> {
    // config.json in the current working directory ($PWD)
    let config_files = vec![config_file, "config.json"];
    if let Ok(mut path) = env::current_dir() {
        for filename in &config_files {
            path.push(filename);
            if path.exists() {
                return Some(path);
            }
            path.pop();
        }
    } else {
        // config.json in the current working directory (relative path)
        for filename in &config_files {
            let relative_path = PathBuf::from(filename);
            if relative_path.exists() {
                return Some(relative_path);
            }
        }
    }
    // UNIX global configuration file
    #[cfg(unix)]
    {
        for filename in &config_files {
            let path_str = "/etc/shadowsocks-rust/".to_owned() + filename;
            let global_config_path = Path::new(&path_str);
            if global_config_path.exists() {
                return Some(global_config_path.to_path_buf());
            }
        }
    }
    None
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GfwConfig {
    http_mode: bool,
    gfw_http_server: String,
    http_forward_server: String,
    local_or_remote: bool,
    gfw_socks5_server: String,
    socks5_forward_server: String,
    password: String,
}

impl GfwConfig {
    // http_mode = true; // http proxy
    // http_mode = false; // socks5 proxy
    pub fn get_http_mode(&self) -> bool {
        self.http_mode
    }

    // local_or_remote = true; // local proxy server
    // local_or_remote = false // proxy server on VPS
    pub fn get_proxy_type(&self) -> bool {
        self.local_or_remote
    }

    // proxy server listening...
    pub fn get_server(&self) -> &str {
        if self.http_mode {
            self.gfw_http_server.as_str()
        } else {
            self.gfw_socks5_server.as_str()
        }
    }

    // forward data to anohter server...
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

/// Runtime Mode (Tokio)
#[derive(Debug, Clone, Copy, Default)]
pub enum RuntimeMode {
    /// Single_thread Runtime
    #[cfg_attr(not(feature = "multi-threaded"), default)]
    SingleThread,
    /// Multi-Thread Runtime
    #[cfg(feature = "multi-threaded")]
    #[cfg_attr(feature = "multi-threaded", default)]
    MultiThread,
}
#[derive(Debug)]
pub struct RuntimeModeError;

impl FromStr for RuntimeMode {
    type Err = RuntimeModeError;
    fn from_str(s: &str) -> Result<RuntimeMode, Self::Err> {
        match s {
            "single_thread" => Ok(RuntimeMode::SingleThread),
            #[cfg(feature = "multi-threaded")]
            "multi_thread" => Ok(RuntimieMode::MultiThread),
            _ => Err(RuntimeModeError),
        }
    }
}
