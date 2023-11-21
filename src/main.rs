// #![warn(missing_docs)]

use std::env;
use homedir::get_my_home;

pub mod cli;
pub mod config;
pub mod event;
pub mod datetime;
pub mod file_utils;

extern crate pretty_env_logger;
use log;


fn throw_loggling_level_warning(args: &cli::Arguments) {
    if args.trace {
        log::warn!("Logging level set to TRACE");
    } else if args.debug {
        log::warn!("Logging level set to DEBUG");
    }
}

/// Sets the logging level for the entire package
fn set_logging_level(args: &cli::Arguments) {
    if args.trace {
        env::set_var("RUST_LOG", "trace");
    } else if args.debug {
        env::set_var("RUST_LOG", "debug");
    } else {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
    throw_loggling_level_warning(args);
}

fn main() {

    let args = cli::parse_args();  // Parse the command line arguments

    // Set the logging level for the package using a hack that sets an
    // environment variable in local scope, which is then read back by the
    // pretty logger. This happens right away so that all other calls
    // have their logging levels set correctly.
    set_logging_level(&args);

    // Access the current core (highest level) command
    let current_command = &args.command;

    // Get current home directory
    let home = get_my_home().unwrap().unwrap();
    log::debug!("Home directory is '{:?}'", home);

    // Depending on the core command, we match against the available options
    // and run that specific logic
    match current_command {
        cli::Command::Event(sc_data) => {
            log::debug!("Event subcommand data: {:?}", sc_data);
            event::execute(&sc_data);
        },
        cli::Command::Config(sc_data) => {
            log::debug!("Config subcommand data: {:?}", sc_data);
            config::config(&sc_data);
            // add::_add(&sc_data);
        }
    }
}
