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
pub struct ConfigCommand {

    #[clap(subcommand)]
    pub command: Option<ConfigSubcommand>

}
