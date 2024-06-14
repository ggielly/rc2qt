mod generator;
mod parser;
mod resource;

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("rc2qt")
        .version("0.01")
        .author("Guillaume Gielly")
        .about("Converts .rc files to Qt project files")
        .arg(
            Arg::new("rcfile")
                .help("Sets the input .rc file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .help("Sets the output directory")
                .required(true)
                .index(2),
        )
        .get_matches();

    let rc_file_path = matches.get_one::<String>("rcfile").unwrap();
    let output_dir = matches.get_one::<String>("output").unwrap();

    let (
        bitmap_list,
        icon_list,
        string_table_list,
        toolbar_list,
        accelerator_table_list,
        menu_list,
        dialog_ex_list,
    ) = parser::parse_rc_file(rc_file_path);

    // Output each list
    bitmap_list.output();
    icon_list.output();
    string_table_list.output();
    toolbar_list.output();
    accelerator_table_list.output();
    menu_list.output();
    dialog_ex_list.output();

    // Example for creating a .qrc file:
    let resources: std::collections::HashMap<String, String> = std::collections::HashMap::new(); // Populate this with actual resource data
    let qrc_output_path = format!("{}/resources.qrc", output_dir);
    generator::create_qrc_file(&resources, &qrc_output_path);

    // Example for creating a .ui file:
    let dialog_name = "ExampleDialog";
    let controls =
        vec!["CONTROL \"Button\", IDC_BUTTON1, BUTTON, WS_TABSTOP, 10, 10, 50, 14".to_string()];
    let output_path = format!("{}/{}.ui", output_dir, dialog_name);
    generator::create_ui_file(dialog_name, &controls, &output_path);
}
