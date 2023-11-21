use edit;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use std::path::{PathBuf};
use homedir::get_my_home;
use log::{trace, warn, error};

use crate::cli;
use crate::file_utils::{make_directory, read_file_to_string, write_string_to_file};
use crate::datetime::{validate_datetime_format};


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Project {

    name: String,

    /// Total amount of time allocated to the project, in hours
    total_time: f32,

    /// Indicates whether or not the project is active or not
    /// Setting this flag to false will prevent you from accidentally charging
    /// it.
    active: bool,

    /// Whether or not the project is "real"
    /// A real project has features on it enabled such as being able to add
    /// time to it. Not being real means it's basically a placeholder.
    real: bool,

    start_date: String,
    end_date: String,

    /// Other metadata
    metadata: HashMap<String, String>
}


impl Project {

    /**
    Validates the integrity of the project. Will also throw warnings where
    necessary (for example, when active is false).
    */
    fn validate(&self) {
        if !validate_datetime_format(&self.start_date) {
            error!("start_date {} invalid format!", self.start_date);
            panic!();
        }
        if !validate_datetime_format(&self.end_date) {
            error!("end_date {} invalid format!", self.end_date);
            panic!();
        }
    }

}


/// Returns a default version of the Config so that users have a starting point
pub fn get_default_project() -> Project {
    let mut example_metadata = HashMap::new();
    example_metadata.insert("P/A".to_string(), "56789/12345".to_string());
    let default_project = Project {
        name: "TEST_PROJECT".to_string(),
        total_time: 100.0,
        active: true,
        real: false,
        start_date: "01-Oct-23".to_string(),
        end_date: "01-Oct-24".to_string(),
        metadata: example_metadata
    };
    default_project.validate();
    trace!("Default project retrieved: {:?}", default_project);
    return default_project;
}


pub fn execute(args: &cli::cli_project::ProjectCommand) {
    let current_command = &args.command;
    match current_command {
        Some(cli::cli_project::ProjectSubcommand::Edit(_)) => {},
        Some(cli::cli_project::ProjectSubcommand::Show(_)) => {},
        None => {
            let _ = get_default_project();
            warn!("No project option provided!");
        }
    }
}
