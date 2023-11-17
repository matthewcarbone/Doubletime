use edit;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use std::path::{PathBuf};
use homedir::get_my_home;
use log::{trace, warn, error};

use crate::cli;
use crate::file_utils::{make_directory, read_file_to_string, write_string_to_file};


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Project {

    /// Total amount of time allocated to the project, in hours
    total_time: f32,

    /// Indicates whether or not the project is active or not
    /// Setting this flag to false will prevent you from accidentally charging
    /// it.
    active: bool,

    start_date: String,
    end_date: String,

    /// Other metadata
    metadata: HashMap<String, String>
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    max_hours_per_day: f32,
    projects: Vec<Project>
}


/// Gets the config path as a PathBuf object
fn get_config_path() -> PathBuf {
    let home = get_my_home().unwrap().unwrap();
    let mut file_path = PathBuf::from(home);
    file_path.push("Doubletime");
    file_path.push("config.yaml");
    trace!("Got config path at {}", file_path.to_string_lossy());
    return file_path;
}


/// Returns a default version of the Config so that users have a starting point
pub fn get_default_config() -> Config {
    let mut example_metadata = HashMap::new();
    example_metadata.insert("P/A".to_string(), "56789/12345".to_string());
    let example_project = Project {
        total_time: 100.0,
        active: true,
        start_date: "01-October-23".to_string(),
        end_date: "01-October-24".to_string(),
        metadata: example_metadata
    };
    let config = Config {
        max_hours_per_day: 8.0,
        projects: vec![example_project]
    };
    trace!("Default config retrieved: {:?}", config);
    return config;
}


/// Writes the default configuration file
fn write_default_config() -> Result<(), Box<dyn std::error::Error>> {
    trace!("Calling write_default_config()");
    let config = get_default_config();
    let config_path = get_config_path();
    trace!("Attempting to write default config {:?} to {:?}", config, config_path);
    let config_as_str = serde_yaml::to_string(&config)?;
    write_string_to_file(&config_as_str, config_path)?;
    trace!("Wrapping up write_default_config()");
    return Ok(());
}


/// Setup the .doubletime home directory
fn initialize() -> Result<(), Box<dyn std::error::Error>> {
    trace!("Attempting initialization");
    let config_path = get_config_path();
    let parent = PathBuf::from(config_path.parent().unwrap());
    make_directory(parent)?;
    if !config_path.exists() {
        write_default_config()?;
    }
    return Ok(());
}





/// Edits the config file in the user's default editor.
/// See here https://docs.rs/edit/latest/edit/fn.get_editor.html for details
/// on how the default editor is chosen
fn edit_config() {

    // The initialization is safe in the sense that it will attempt to create
    // the doubletime "home" directory at <HOME>/.doubletime, but will safely
    // do nothing if the directory already exists.
    match initialize() {
        Ok(()) => {
            trace!("Call to initialize() successful")
        }
        Err(e) => {
            error!("Error {} with initialize() during edit_config", e);
            panic!();
        }
    }

    // Get the default editor. This is mainly for debugging.
    let editor = edit::get_editor();
    trace!("Editing config with default editor: {:?}", editor.unwrap());

    // Get the config path
    let config_path = get_config_path();

    // Open the editor in your default config
    match edit::edit_file(config_path.clone()) {
        Ok(()) => {
            trace!("Config edited successfully");
        }
        Err(e) => {
            error!("Error {} editing config at {}", e, config_path.to_string_lossy());
            panic!();
        }
    }
}


fn show_config() {
    let config_path = get_config_path();
    if !config_path.exists() {
        warn!("Config path at {:?} does not exist, cannot visualize it. Try running config edit.", config_path);
        return;
    }
    match read_file_to_string(config_path.clone()) {
        Ok(config) => {
            let parsed: Config = serde_yaml::from_str(&config).unwrap();
            println!("{:#?}", parsed);
            return;
        },
        Err(e) => {
            error!("Error {} reading config path at {:?} to memory", e, config_path);
            panic!();
        }
    }
}


pub fn config(args: &cli::ConfigCommand) {
    let current_command = &args.command;
    match current_command {
        cli::ConfigSubcommand::Edit(_) => {edit_config();},
        cli::ConfigSubcommand::Show(_) => {show_config();}
    }
}



// pub fn write_config(destination_path: PathBuf, config: Config) -> Result<(), Box<dyn std::error::Error>>{
//     trace!("Attempting to write config file {:?} to disk at {:?}", config, destination_path.to_string_lossy());

//     let config_as_str = serde_yaml::to_string(&config)?;
//     let mut f = File::create(destination_path)?;
//     f.write_all(config_as_str.as_bytes())?;
//     return Ok(());
// }
