use crate::gfw_config::{Config, ConfigType};

use crate::gfw_config::{ ServiceConfig, RuntimeMode};

use clap::{Arg, ArgAction, ArgMatches, Command, ValueHint};
use std::path::PathBuf;
use std::process::ExitCode;
use tokio::{self, runtime::Builder};

// Define command line options
pub fn define_command_line_options(mut app: Command) -> Command {
    app = app.arg(
        Arg::new("CONFIG")
            .short('c')
            .long("config")
            .num_args(1)
            .action(ArgAction::Set)
            .value_parser(clap::value_parser!(PathBuf))
            .value_hint(ValueHint::FilePath)
            .help("gfw proxy shadowsocks config file"),
    );
    app
}

// Program entrace `main`
pub fn main(matches: &ArgMatches) -> ExitCode {
    // config and runtime
    let (config, runtime) = {
        let config_path_opt = matches.get_one::<PathBuf>("CONFIG").cloned().or_else(|| {
            if !matches.contains_id("SERVER_CONFIG") {
                match crate::gfw_config::get_default_config_path("local.json") {
                    None => None,
                    Some(p) => {
                        println!("loading default config {p:?}");
                        Some(p)
                    }
                }
            } else {
                None
            }
        });
        let mut service_config = match config_path_opt {
            Some(ref config_path) => match ServiceConfig::load_from_file(config_path) {
                Ok(c) => c,
                Err(err) => {
                    eprintln!("loading config {config_path:?},{err}");
                    return crate::EXIT_CODE_LOAD_CONFIG_FAILURE.into();
                }
            },
            None => ServiceConfig::default(),
        };
        service_config.set_options(matches);

        let mut builder = match service_config.runtime.mode {
            RuntimeMode::SingleThread => Builder::new_current_thread(),
            #[cfg(feature = "multi-Thread")]
            RuntimeMode::MultiThread => {
                let mut builder = Builder::new_multi_thread();
                if let Some(worker_threads) = service_config.runtime.worker_count {
                    builder.worker_threads(worker_threads);
                }
                builder
            }
        };

        println!("{:?}",service_config);

        let mut config = match config_path_opt {
            Some(cpath) => match Config::load_from_file(&cpath,ConfigType::Local) {
                Ok(cfg) => cfg,
                Err(err) => {
                    eprintln!("loading config {cpath:?}, {err}");
                    return crate::EXIT_CODE_LOAD_CONFIG_FAILURE.into();
                }
            },
            None => Config::new(ConfigType::Local),
        };

        let runtime = builder.enable_all().build().expect("create tokio Runtime");

        (config, runtime)
    };

    return crate::EXIT_CODE_LOAD_CONFIG_FAILURE.into()
}
