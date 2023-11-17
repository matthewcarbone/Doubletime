use clap::{Args, Parser, Subcommand};



#[derive(Debug, Args)]
pub struct AddCommand {

    /// The date corresponding to the event to be added
    #[arg(short, long, default_value_t = String::from("TODAY"))]
    pub date: String,

    /// 'Commit message' for the event
    #[arg(short, long, default_value_t = String::from("null"))]
    pub message: String,

    // Project name for the event
    #[arg(short, long, default_value_t = String::from("null"))]
    pub project: String

}



#[derive(Debug, Args)]
pub struct ConfigEditCommand {}

#[derive(Debug, Args)]
pub struct ConfigShowCommand {}



/// Config subcommands
#[derive(Debug, Subcommand)]
pub enum ConfigSubcommand {

    /// Edit the core configuration file
    Edit(ConfigEditCommand),

    /// Show the core configuration file
    Show(ConfigShowCommand)
}


#[derive(Debug, Args)]
pub struct ConfigCommand {

    #[clap(subcommand)]
    pub command: ConfigSubcommand

}

/// Core doubletime commands
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Help message for Add.
    Add(AddCommand),
    Config(ConfigCommand)
}


#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
/// Search for a pattern in a file and display the lines that contain it.
pub struct Arguments {

    #[clap(short, long, default_value_t = false)]
    pub debug: bool,

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
