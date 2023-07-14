// //! This is a gfw proxy shadow clone
// //! You have to provide all needed configuration attributes
// //! via command line parameters, or you could specify a configuration file.
// //! the format of configuration file is defined in mod `config`.

// use std::process::ExitCode;

// use clap::Command;

// fn main() -> ExitCode {
//     let mut app = Command::new("gfw_proxy_ss")
//         .version(gfw_library::VERSION)
//         .about("a gfw proxy shadowsocks clone");
//     app = gfw_library::local::define_command_line_options(app);
//     let matches = app.get_matches();
//     local::main(&matches)
// }
