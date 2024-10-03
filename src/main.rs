use clap::{command as clapCommand, Arg, ArgAction};
use zoomark::{
    display::table::show_all_data, installation::installation_folder::check_folder_and_file,
};

fn main() {
    let arguments_commands = clapCommand!()
        .about("Little program that reminds you what your commands do")
        // Command name argument
        .arg(
            Arg::new("command_name")
                .short('c')
                .long("command")
                .help("This argument defines the command name you want to save"),
        )
        // Command description argument
        .arg(
            Arg::new("command_description")
                .short('d')
                .long("description")
                .help("This argument defines the description you want to save"),
        )
        // Show commands argument
        .arg(
            Arg::new("show_command")
                .short('s')
                .long("show-commands")
                .help("This argument shows you the commands you have saved")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let command_name = arguments_commands.get_one::<String>("command_name");
    let command_description = arguments_commands.get_one::<String>("command_description");

    if command_name.is_some() && command_description.is_none() {
        println!("Error: '--description <command_description>' must be used with '--command <command_name>'");
    } else if command_description.is_some() && command_name.is_none() {
        println!("Error: '--command <command_name>' must be used with '--description <command_description>'");
    }

    if let (Some(first_arg), Some(second_arg)) = (command_name, command_description) {
        check_folder_and_file(first_arg.to_owned(), second_arg.to_owned());
    }

    if arguments_commands.get_flag("show_command") {
        show_all_data().expect("error showing the data");
    }
}
