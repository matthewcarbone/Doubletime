/**
Basic file utilities that are agnostic to the Doubletime code.
*/

use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::path::PathBuf;

use std::fs::create_dir_all;

use log::trace;


pub fn make_directory(directory: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let dstring = directory.to_string_lossy();

    trace!("Attempting to make directory {} including its parents", dstring);

    // If the directory exists, we're done, log it and return Ok(())
    // If the directory does not exist, log it and continue
    if directory.exists() {
        trace!("directory {} already exists, exiting with Ok", dstring);
        return Ok(());
    }
    // Create the directory and all of its parents, propogating errors
    // accordingly
    create_dir_all(directory.clone())?;

    trace!("directory {} created successfully", dstring);
    return Ok(());
}


pub fn write_string_to_file(content: &str, file_path: PathBuf) -> Result<(), std::io::Error> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    return Ok(());
}


pub fn read_file_to_string(file_path: PathBuf) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    return Ok(content);
}
