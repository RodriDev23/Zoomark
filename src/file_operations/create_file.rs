use std::fs::{self, File};

use crate::helpers;

pub fn create_folder() {
    let directory_user = helpers::fn_helpers::full_path();
    match fs::create_dir_all(&directory_user) {
        Ok(_) => println!("folder create it!"),
        Err(err) => {
            println!("Erro creating the folder: {}", err);
        }
    }
}

pub fn create_file() -> File {
    let full_path_c = helpers::fn_helpers::full_path_w_file();
    File::create(&full_path_c).expect("error creating the file")
}
