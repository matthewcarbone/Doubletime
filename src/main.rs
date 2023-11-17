use std::env;
use homedir::get_my_home;

pub mod cli;
pub mod config;
pub mod add;
pub mod datetime;
pub mod file_utils;

extern crate pretty_env_logger;
use log;


/// Sets the logging level for the entire package
fn set_logging_level(args: &cli::Arguments) {
    if args.debug {
        env::set_var("RUST_LOG", "trace");
        log::trace!("Logging level set to TRACE");
    } else {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
}

fn main() {

    // println!("RUNNIG");

    

    // Parse the command line arguments
    let args = cli::parse_args();

    // Set the logging level for the package using a hack that sets an
    // environment variable in local scope, which is then read back by the
    // pretty logger
    set_logging_level(&args);

    // let conf = config::get_default_config();

    // panic!();

    // // Print all arguments at the TRACE log level
    // log::trace!("{:?}", args);

    // Access the current core command
    let current_command = &args.command;

    // Get current home directory
    let home = get_my_home().unwrap().unwrap();

    log::trace!("Home directory is '{:?}'", home);


    // println!("{:?}", current_command);

    // Match the subcommand in question
    // match current_command {
    //    cli::Command::Add { .. } => {
    //        println!("{:?}\n{:?}", args, args.command);
    //        println!("{:?}", &args.command);
    //    },

    match current_command {
        cli::Command::Add(sc_data) => {
            log::debug!("Add subcommand data: {:?}", sc_data);
            add::_add(&sc_data);
        },
        cli::Command::Config(sc_data) => {
            log::debug!("Config subcommand data: {:?}", sc_data);
            config::config(&sc_data);
            // add::_add(&sc_data);
        }
    }
}


// #[derive(Debug)]
// struct Person<'a> {
//     _name: &'a str,
//     _age: u8
// }

// fn main() {
//     let _name = "Peter";
//     let _age = 27;
//     let peter = Person { _name, _age };

//     // Pretty print
//     println!("{:#?}", peter);
// }

