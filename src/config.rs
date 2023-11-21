/**
Module for dealing with Doubletime-specific configuration files.
*/

use edit;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use std::path::{PathBuf};
use homedir::get_my_home;
use log::{trace, info, warn, error};

use crate::cli;
use crate::file_utils::{make_directory, read_file_to_string, write_string_to_file};


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    max_hours_per_day: f32,
    
    /// Other metadata
    metadata: HashMap<String, String>
}


/// Returns a default version of the Config so that users have a starting point
fn get_default_config() -> Config {
    let mut example_metadata = HashMap::new();
    example_metadata.insert("my name is".to_string(), "Matt".to_string());
    let config = Config {
        max_hours_per_day: 8.0,
        metadata: example_metadata
    };
    trace!("Default config retrieved: {:?}", config);
    return config;
}


/**
Gets Doubletime's home directory
*/
fn get_doubletime_home_directory() -> PathBuf {
    let home = get_my_home().unwrap().unwrap();
    let mut file_path = PathBuf::from(home);
    file_path.push("Doubletime");
    return file_path;
}


/**
Gets the path of a directory. The path will be {HOME}/Doubletime/{name}.
*/
fn get_doubletime_directory_path(name: String) -> PathBuf {
    let mut staging_path = get_doubletime_home_directory();
    staging_path.push(name);
    return staging_path;
}


/// Gets the config path as a PathBuf object
fn get_config_path() -> PathBuf {
    trace!("get_config_path()");
    let mut file_path = get_doubletime_home_directory();
    file_path.push("config.yaml");
    trace!("get_config_path: Got config path at {}", file_path.to_string_lossy());
    return file_path;
}


/// Writes the default configuration file
fn write_default_config() -> Result<(), Box<dyn std::error::Error>> {
    trace!("write_default_config()");

    let config = get_default_config();
    let config_path = get_config_path();
    trace!("Attempting to write default config {:?} to {:?}", config, config_path);
    let config_as_str = serde_yaml::to_string(&config)?;
    write_string_to_file(&config_as_str, config_path)?;
    trace!("Wrapping up write_default_config()");
    return Ok(());
}

/**
Setup the Doubletime home directory and all required subdirectories
*/
pub fn initialize() -> Result<(), Box<dyn std::error::Error>> {
    trace!("initialize()");

    // Make the home directory
    let home = get_doubletime_home_directory();
    make_directory(home)?;

    // Make the staging directory
    let staging_directory = get_doubletime_directory_path("Staging".to_string());
    make_directory(staging_directory)?;

    // Make the staging directory
    let project_directory = get_doubletime_directory_path("Projects".to_string());
    make_directory(project_directory)?;

    // Make the default config if it does not exist
    let config_path = get_config_path();
    if !config_path.exists() {
        write_default_config()?;
        info!("Default config has been created!");
        info!("Edit this config with `dt config edit`");
    }
    return Ok(());
}

/// Edits the config file in the user's default editor.
/// See here https://docs.rs/edit/latest/edit/fn.get_editor.html for details
/// on how the default editor is chosen
fn edit_config() {
    trace!("edit_config()");

    // Get the default editor. This is mainly for debugging.
    let editor = edit::get_editor();
    trace!("Editing config with default editor: {:?}", editor.unwrap());

    // Get the config path
    let config_path = get_config_path();

    let config_before_edit = read_file_to_string(config_path.clone());

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

    // Now try to re-load the config from disk and check that it's valid. If it
    // is not, we write the config_before_edit back to disk, rejecting the
    // user's changes
    let config_after_edit = read_file_to_string(config_path.clone());
    let parsed: Result<Config, _> = serde_yaml::from_str(&config_after_edit.unwrap());
    match parsed {
        Ok(_) => {},
        Err(_) => {
            error!("Config edit failed");
            error!("It's possible your edited config is not valid Config OR yaml file");
            info!("Never fear! Rolling back config to before previous edits...");
            let _ = write_string_to_file(&config_before_edit.unwrap(), config_path);
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


pub fn config(args: &cli::cli_config::ConfigCommand) {
    let current_command = &args.command;
    match current_command {
        Some(cli::cli_config::ConfigSubcommand::Edit(_)) => {edit_config();},
        Some(cli::cli_config::ConfigSubcommand::Show(_)) => {show_config();},
        None => {
            trace!("No config option provided!");
        }
    }
}



// pub fn write_config(destination_path: PathBuf, config: Config) -> Result<(), Box<dyn std::error::Error>>{
//     trace!("Attempting to write config file {:?} to disk at {:?}", config, destination_path.to_string_lossy());

//     let config_as_str = serde_yaml::to_string(&config)?;
//     let mut f = File::create(destination_path)?;
//     f.write_all(config_as_str.as_bytes())?;
//     return Ok(());
// }
