/// This module contains the parsing logic for the add command


use clap::{Args, Subcommand};



#[derive(Debug, Args)]
pub struct EventAddCommand {

    /// The date corresponding to the event to be added
    #[arg(short, long)]
    pub date: Option<String>,

    /// 'Commit message' for the event
    #[arg(short, long)]
    pub message: Option<String>,

    // Project name for the event
    #[arg(short, long)]
    pub project: Option<String>

}

#[derive(Debug, Args)]
pub struct EventUnstageCommand {

    /// Commit hash to unstage
    #[arg(short, long)]
    pub id: String

}



/// Config subcommands
#[derive(Debug, Subcommand)]
pub enum EventSubcommand {

    /**
    Add a new event to the staging area
    */
    Add(EventAddCommand),

    /**
    Unstage an event
    */
    Unstage(EventUnstageCommand)
}


#[derive(Debug, Args)]
pub struct EventCommand {

    #[clap(subcommand)]
    pub command: EventSubcommand,

}
