use clap::{Args, Subcommand};


#[derive(Debug, Args)]
pub struct ProjectEditCommand {}

#[derive(Debug, Args)]
pub struct ProjectShowCommand {}



/// Project subcommands
#[derive(Debug, Subcommand)]
pub enum ProjectSubcommand {

    /// Edit the core Project file
    Edit(ProjectEditCommand),

    /// Show the core Project file
    Show(ProjectShowCommand)
}


#[derive(Debug, Args)]
pub struct ProjectCommand {

    #[clap(subcommand)]
    pub command: Option<ProjectSubcommand>

}
