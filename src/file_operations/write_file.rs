use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::json_operations::struct_json::StructJson;

pub fn write_file(file_created: &mut File, user_command: String, description_command: String) {
    let mut old_content_json = String::new();
    file_created
        .read_to_string(&mut old_content_json)
        .unwrap_or(1);

    let mut json_content: Vec<StructJson> = if old_content_json.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(old_content_json.as_str()).unwrap_or_else(|_| Vec::new())
    };
    let person = StructJson {
        key_name_command: user_command,
        key_description_command: description_command,
    };
    json_content.push(person);
    let json_string = serde_json::to_string(&json_content).expect("error parsing the data to json");
    file_created.set_len(0).expect("error truncating the file");
    file_created
        .write_all(json_string.as_bytes())
        .expect("error writing the file");
}

pub fn write_existing_file(
    existing_file: &PathBuf,
    user_command: String,
    description_command: String,
) {
    let contents = fs::read_to_string(existing_file).expect("error here in contents var");

    let mut vec_command: Vec<StructJson> = if contents.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
    };
    let person = StructJson {
        key_name_command: user_command,
        key_description_command: description_command,
    };
    vec_command.push(person);
    let json_string = serde_json::to_string(&vec_command).expect("error parsing the data to json");
    fs::write(existing_file, json_string).expect("error writing the file");
}
