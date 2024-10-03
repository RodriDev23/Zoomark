use owo_colors::OwoColorize;
use std::io;
use tabled::{
    settings::{
        object::Columns, peaker::PriorityMax, Alignment, Format, Height, Modify, Settings, Style,
        Width,
    },
    Table, Tabled,
};

use crate::{
    display::terminal::get_terminal_size, helpers, json_operations::struct_json::StructJson,
};

#[derive(Tabled)]
struct TableOutput {
    command_name: String,
    description: String,
}

pub fn show_all_data() -> Result<Vec<String>, io::Error> {
    let full_path_file = helpers::fn_helpers::full_path_w_file();
    let all_data = std::fs::read_to_string(full_path_file)?;
    let vector_full_data = Vec::new();
    let data_formated: Vec<StructJson> =
        serde_json::from_str(&all_data).expect("error parsing the json");

    let mut table_to_print: Vec<TableOutput> = Vec::new();

    for val in data_formated {
        let table_entry = TableOutput {
            command_name: val.key_name_command.trim().to_string(),
            description: val.key_description_command.trim().to_string(),
        };
        table_to_print.push(table_entry);
    }

    let (width, height) = get_terminal_size();

    let table_settings = Settings::default()
        .with(Width::wrap(width).priority(PriorityMax))
        .with(Height::limit(height).priority(PriorityMax));

    let mut table = Table::new(table_to_print);
    table
        .with(Style::extended())
        .with(table_settings)
        .with(Style::modern_rounded())
        .modify(
            Columns::single(0),
            Format::content(|s| s.blue().to_string()),
        )
        .with(Modify::new(Columns::single(0)).with(Alignment::center()))
        .with(Modify::new(Columns::single(1)).with(Alignment::center()));

    println!("{table}");

    Ok(vector_full_data)
}
