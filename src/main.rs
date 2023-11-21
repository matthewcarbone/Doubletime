// #![warn(missing_docs)]

use std::env;

pub mod cli;
pub mod config;
pub mod datetime;
pub mod event;
pub mod file_utils;
pub mod project;

extern crate pretty_env_logger;
use log;

use crate::config::initialize;


fn throw_loggling_level_warning(args: &cli::Arguments) {
    if args.trace {
        log::warn!("Logging level set to TRACE");
    } else if args.debug {
        log::warn!("Logging level set to DEBUG");
    }
}

/// Sets the logging level for the entire package
fn set_logging_level(args: &cli::Arguments) {
    env::set_var("RUST_LOG_STYLE", "auto");
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

fn are_safeties_on(args: &cli::Arguments) -> bool{
    if !args.disable_safeties {
        return true;
    }
    return false;
}

fn main() {

    let args = cli::parse_args();  // Parse the command line arguments

    // Set the logging level for the package using a hack that sets an
    // environment variable in local scope, which is then read back by the
    // pretty logger. This happens right away so that all other calls
    // have their logging levels set correctly.
    set_logging_level(&args);

    // Initialize files if they don't exist
    // The initialization is safe in the sense that it will attempt to create
    // the doubletime "home" directory at <HOME>/Doubletime, in addition to
    //other auxiliary directories and files, but will safely do nothing if
    // the directory already exists.
    match initialize() {
        Ok(()) => {
            log::debug!("Initialization success");
        }
        Err(e) => {
            log::error!("Error {} with initialize()", e);
            panic!();
        }
    }

    // Access the current core (highest level) command
    let current_command = &args.command;
    let _safeties_on = are_safeties_on(&args); // Do something with this later

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
        },
        cli::Command::Project(sc_data) => {
            log::debug!("Project subcommand data: {:?}", sc_data);
            project::execute(&sc_data);
        }
    }
}
