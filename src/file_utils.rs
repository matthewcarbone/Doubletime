use std::path::PathBuf;

use std::fs::create_dir_all;

use log::trace;


/// Safely makes a directory. If the directory already exists, safely
/// exits without doing anything. 

// let p = PathBuf::from("yo/lo/swag");
// match file_utils::make_directory(p) {
//     Ok(()) => {
//         log::info!("Made directory!");
//     },
//     Err(e) => {
//         log::error!("Error making directory - {:?}", e);
//         panic!();
//     }
// }

pub fn make_directory(directory: PathBuf) -> Result<(), Box<dyn std::error::Error>>{
    let dstring = directory.to_string_lossy();

    trace!("Attempting to make directory '{}' including its parents", dstring);

    if directory.exists() {
        trace!("directory {} already exists, exiting Ok", dstring);
        return Ok(());
    } else {
        trace!("directory {} does not exist, continuing", dstring);
    }

    create_dir_all(directory.clone())?;

    trace!("directory {} created successfully", dstring);
    return Ok(());
}

