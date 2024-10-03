use crate::{
    file_operations::{
        create_file::{create_file, create_folder},
        write_file::{write_existing_file, write_file},
    },
    helpers,
};

pub fn check_folder_and_file(user_save_command: String, description_command_user: String) {
    let full_path = helpers::fn_helpers::full_path();
    let full_path_with_file = helpers::fn_helpers::full_path_w_file();

    if !full_path.exists() {
        create_folder();
    }
    if full_path_with_file.exists() {
        write_existing_file(
            &full_path_with_file,
            user_save_command.clone(),
            description_command_user.clone(),
        );
    } else {
        let mut file_new = create_file();
        write_file(
            &mut file_new,
            user_save_command.clone(),
            description_command_user.clone(),
        );
    }
    if full_path_with_file.is_file()
        && full_path_with_file
            .metadata()
            .expect("error getting metadata of file")
            .len()
            == 0
    {
        write_existing_file(
            &full_path_with_file,
            user_save_command.clone(),
            description_command_user.clone(),
        );
    }
}
