use std::fs;

use super::kingdom::Kingdom;

pub const SAVES_PATH : &str = "./saves";

/// Reads a save file from the saves-folder and parses its JSON-content
/// # Arguments
/// * `file_name` is the name of the file, which should be read.
pub fn load_save_file(file_name: &str) -> Kingdom{

    let path = format!("{}/{file_name}",SAVES_PATH);

    let data = fs::read_to_string(path)
        .expect(format!("Unable to read file '{SAVES_PATH}/{file_name}'").as_str());

    let kingdom = serde_json::from_str(&data)
        .expect(format!("JSON in file '{file_name}' was not well-formatted").as_str());

    kingdom
}