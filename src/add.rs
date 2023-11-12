use std::fs::File;
use std::path::PathBuf;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use homedir::get_my_home;

use crate::cli::{AddCommand};
use crate::datetime::{parse_command_line_date, get_current_timestamp};
use crate::file_utils::make_directory;

use log;


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Event {
    timestamp: i64,
    message: String,
    project: String,
    now_str: String,
    uuid: String
}


impl Event {

    /// Logs information about the Event to the console
    fn log_info(&self) {
        let uid_slice = &self.uuid[..8];
        log::info!("Staging Event {}", uid_slice);
        log::debug!("| timestamp   {}", self.timestamp);
        log::debug!("| full commit {}", self.uuid);
        log::info!("| date        {}", self.now_str);
        log::info!("| project     {}", self.project);
        let message_slice = &self.message[..30];
        log::debug!("| full msg    {}", self.message);
        log::info!("| message     {} ...", message_slice);
    }

    fn stage(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::trace!("Event.stage - called");

        let timestamp = self.timestamp.to_string();

        // This file_path is the HOME directory on the user's computer
        let root = get_my_home().unwrap().unwrap();
        let mut file_path = PathBuf::from(root);
        let _ = make_directory(file_path.clone());

        file_path.push("staging");
        let _ = make_directory(file_path.clone());

        // Create a PathBuf and build the file path
        // let mut file_path = directory.clone();

        // Check whether or not the directory itself exists
        if !file_path.exists() {
            log::error!("File path '{}' does not exist", file_path.to_string_lossy());
            panic!();
        }

        // Create the full file path
        let fname = format!("{}.json", timestamp);
        file_path.push(fname);

        // Now assert that this file does _not_ exist. This should never
        // happen since the file name is actually the timestamp, so unless
        // someone is purposefully doing something really stupid, this should
        // never happen
        if file_path.exists() {
            log::error!("Json file {} exists! Something has gone HORRIBLY wrong!", file_path.to_string_lossy());
            panic!();
        }

        log::debug!("Event.stage is saving to {}", file_path.to_string_lossy());

        let file_path = File::create(file_path)?;
        serde_json::to_writer(file_path, self)?;
        return Ok(());
    }

    fn _commit(&self) {
        panic!("not implemented!");
    }

}

pub fn _add(args: &AddCommand) {

    // Parse the command line date
    let now: NaiveDate = parse_command_line_date(&args.date);
    log::debug!("Parsed date from command line to {:?}", now);

    // Get the current stimestamp
    let timestamp: i64 = get_current_timestamp();

    // Generate a uid
    let uuid = Uuid::new_v4().to_simple().to_string();
    log::trace!("UID generated: {}", uuid);

    let payload: Event = Event {
        timestamp: timestamp,
        message: (&args.message).to_string(),
        project: (&args.project).to_string(),
        now_str: now.format("%d-%b-%y").to_string(),
        uuid: uuid
    };

    payload.log_info();
    

    match payload.stage() {
        Ok(()) => {
            log::info!("Staging successful");
        },
        Err(e) => {
            log::error!("Error during Event.stage - {:?}", e);
            panic!();
        }
    }

}
