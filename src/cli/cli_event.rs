/// This module contains the parsing logic for the add command


use clap::{Args, Subcommand};



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
pub struct EventCommand {

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
