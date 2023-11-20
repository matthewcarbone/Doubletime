/**
Core command line interface (CLI) parser logic

The doubletime CLI supports commands for staging, committing and querying
project information saved to its internal database. The user can control
everything required from the command line.

Commands are ordered in the same sense as "module.class.method", such that the
first command always corresponds to the base object you're modifying or adding,
with subsequent commands detailing what you want to do. For example

```
dt event stage --project=my_project --time=6.5
```

to stage a new event under my_project for 6.5 hours. Or,

```
dt event unstage -c 12ab382l
```

to unstage event with hash 12ab382l.
*/



pub mod cli_event;
pub mod cli_config;

use crate::cli::cli_event::EventCommand;
use crate::cli::cli_config::ConfigCommand;
use clap::{Parser, Subcommand};


/// Core doubletime commands
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Help message for Add.
    Event(EventCommand),
    Config(ConfigCommand)
}


#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
/// Search for a pattern in a file and display the lines that contain it.
pub struct Arguments {

    #[clap(long, default_value_t = false)]
    /**
    If turned on, runs doubletime in debugging mode, outputting all debug 
    logs to the console
    */
    pub debug: bool,

    #[clap(long, default_value_t = false)]
    /**
    If turned on, runs doubletime in trace mode, outputting all trace 
    logs to the console. Note that this can be quite verbose, as the trace
    logs are primarily used for development. If both --trace and --debug are
    set, --trace takes priority.
    */
    pub trace: bool,

    #[clap(long, default_value_t = false)]
    /**
    Dangerous command, will override safeties (such as adding an event that
    lasts longer than the default 8 hours).
    */
    pub force: bool,

    #[clap(subcommand)]
    pub command: Command,

    // /// The pattern to look for
    // #[clap(short, long)]
    // pub pattern: String,

    // #[clap(short, long)]
    // /// The path to the file to read
    // file: std::path::PathBuf,

}


pub fn parse_args() -> Arguments {
    let args = Arguments::parse();
    return args;
}
