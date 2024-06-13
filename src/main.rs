extern crate xml;
extern crate regex;
extern crate clap;

use clap::{Arg, Command};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use xml::writer::{EmitterConfig, XmlEvent};

// RcBitmap structure
#[derive(Debug)]
struct RcBitmap {
    id: String,
    file: String,
}

// RcBitmapList structure
struct RcBitmapList {
    bitmaps: Vec<RcBitmap>,
}

impl RcBitmapList {
    fn new() -> Self {
        RcBitmapList { bitmaps: vec![] }
    }

    fn add(&mut self, bitmap: RcBitmap) {
        self.bitmaps.push(bitmap);
    }

    fn output(&self) {
        println!("void qtMfcInitBitmapResources() {{");
        println!("    qtMfcBitmapResources.clear();");
        for bitmap in &self.bitmaps {
            println!("    // {} BITMAP {}", bitmap.id, bitmap.file);
            println!(
                "    qtMfcBitmapResources.insert({}, new CBitmap({}));",
                bitmap.id, bitmap.file
            );
        }
        println!("}}");
    }
}

// RcIcon structure
#[derive(Debug)]
struct RcIcon {
    id: String,
    file: String,
}

// RcIconList structure
struct RcIconList {
    icons: Vec<RcIcon>,
}

impl RcIconList {
    fn new() -> Self {
        RcIconList { icons: vec![] }
    }

    fn add(&mut self, icon: RcIcon) {
        self.icons.push(icon);
    }

    fn output(&self) {
        println!("void qtInitIconResources() {{");
        println!("    qtIconNames.clear();");
        println!("    qtIconResources.clear();");
        println!("    // Icon with lowest ID value placed first to ensure application icon remains consistent on all systems.");
        for icon in &self.icons {
            println!("    // {} ICON {}", icon.id, icon.file);
            println!("    qtIconNames.insert({}, {});", icon.id, icon.file);
        }
        println!("}}");
    }
}

// RcStringTableItem structure
#[derive(Debug)]
struct RcStringTableItem {
    id: String,
    text: String,
}

// RcStringTable structure
struct RcStringTable {
    table: Vec<RcStringTableItem>,
}

impl RcStringTable {
    fn new(table: Vec<RcStringTableItem>) -> Self {
        RcStringTable { table }
    }

    fn output(&self) {
        for item in &self.table {
            println!(
                "    qtMfcStringResources.insert({}, \"{}\");",
                item.id, item.text
            );
        }
    }
}

// RcStringTableList structure
struct RcStringTableList {
    string_tables: Vec<RcStringTable>,
}

impl RcStringTableList {
    fn new() -> Self {
        RcStringTableList { string_tables: vec![] }
    }

    fn add(&mut self, string_table: RcStringTable) {
        self.string_tables.push(string_table);
    }

    fn output(&self) {
        println!("void qtMfcInitStringResources() {{");
        println!("    qtMfcStringResources.clear();");
        for string_table in &self.string_tables {
            println!("    // STRINGTABLE");
            println!("    // BEGIN");
            string_table.output();
            println!("    // END");
        }
        println!(
            "    // AFX resources
            qtMfcStringResources.insert(AFX_IDS_ALLFILTER, \"All files|\");
            qtMfcStringResources.insert(AFX_IDS_OPENFILE, \"Open\");
            qtMfcStringResources.insert(AFX_IDS_SAVEFILE, \"Save As\");
            qtMfcStringResources.insert(AFX_IDS_SAVEFILECOPY, \"Save As\");
            qtMfcStringResources.insert(AFX_IDS_UNTITLED, \"Untitled\");
            qtMfcStringResources.insert(AFX_IDP_ASK_TO_SAVE, \"Save changes to %s?\");
            qtMfcStringResources.insert(AFX_IDP_FAILED_TO_CREATE_DOC, \"Failed to create empty document.\");
        ");
        println!("}}");
    }
}

// RcToolbar structure
#[derive(Debug)]
struct RcToolbar {
    id: String,
    numbers: Vec<i32>,
    table: Vec<String>, // Assuming a simplified type for demonstration
}

// RcToolbarList structure
struct RcToolbarList {
    toolbars: Vec<RcToolbar>,
}

impl RcToolbarList {
    fn new() -> Self {
        RcToolbarList { toolbars: vec![] }
    }

    fn add(&mut self, toolbar: RcToolbar) {
        self.toolbars.push(toolbar);
    }

    fn output(&self) {
        for toolbar in &self.toolbars {
            println!("void qtMfcInitToolBarResource_{}(UINT dlgID, CToolBar* parent) {{", toolbar.id);
            println!("    // Toolbar logic here...");
        }
        println!("void qtMfcInitToolBarResource(UINT dlgID, CToolBar* parent) {{");
        println!("    switch (dlgID) {{");
        for toolbar in &self.toolbars {
            println!("        case {}:", toolbar.id);
            println!("            qtMfcInitToolBarResource_{}(dlgID, parent);", toolbar.id);
            println!("            break;");
        }
        println!("    }}");
        println!("}}");
    }
}

// RcAcceleratorTable structure
#[derive(Debug)]
struct RcAcceleratorTable {
    id: String,
    table: Vec<String>, // Assuming a simplified type for demonstration
}

// RcAcceleratorTableList structure
struct RcAcceleratorTableList {
    accelerator_tables: Vec<RcAcceleratorTable>,
}

impl RcAcceleratorTableList {
    fn new() -> Self {
        RcAcceleratorTableList {
            accelerator_tables: vec![],
        }
    }

    fn add(&mut self, accelerator_table: RcAcceleratorTable) {
        self.accelerator_tables.push(accelerator_table);
    }

    fn output(&self) {
        for accelerator_table in &self.accelerator_tables {
            println!("// {} ACCELERATORS", accelerator_table.id);
            println!("ACCEL ACCEL_{}[] = {{", accelerator_table.id);
            println!("    // Accelerator table logic here...");
            println!("    {{ 0, 0, 0 }},");
            println!("}};");
        }
        println!("ACCEL* qtMfcAcceleratorResource(UINT id) {{");
        println!("    switch (id) {{");
        for accelerator_table in &self.accelerator_tables {
            println!("        case {}:", accelerator_table.id);
            println!("            return ACCEL_{};", accelerator_table.id);
            println!("            break;");
        }
        println!("    }}");
        println!("    return NULL;");
        println!("}}");
    }
}

// RcMenu structure
#[derive(Debug)]
struct RcMenu {
    id: String,
    table: Vec<String>, // Assuming a simplified type for demonstration
}

// RcMenuList structure
struct RcMenuList {
    menus: Vec<RcMenu>,
}

impl RcMenuList {
    fn new() -> Self {
        RcMenuList { menus: vec![] }
    }

    fn add(&mut self, menu: RcMenu) {
        self.menus.push(menu);
    }

    fn output(&self) {
        for menu in &self.menus {
            println!("void qtMfcInitMenuResource_{}(CMenu* parent) {{", menu.id);
            println!("    // Menu logic here...");
        }
        println!("void qtMfcInitMenuResource(UINT menuID, CMenu* parent) {{");
        println!("    switch (menuID) {{");
        for menu in &self.menus {
            println!("        case {}:", menu.id);
            println!("            qtMfcInitMenuResource_{}(parent);", menu.id);
            println!("            break;");
        }
        println!("    }}");
        println!("    // Fixup shortcuts");
        println!("    int menu = 0;");
        println!("    CMenu* subMenu = parent->GetSubMenu(menu);");
        println!("    while (subMenu) {{");
        println!("        foreach (QAction* action, subMenu->toQMenu()->actions()) {{");
        println!("            if (action->text().contains(\"\\t\")) {{");
        println!("                action->setShortcut(QKeySequence(action->text().split(\"\\t\").at(1)));");
        println!("            }}");
        println!("        }}");
        println!("        menu++;");
        println!("        subMenu = parent->GetSubMenu(menu);");
        println!("    }}");
        println!("}}");
    }
}

// RcDialogEx structure
#[derive(Debug)]
struct RcDialogEx {
    id: String,
    items: Vec<String>, // Assuming a simplified type for demonstration
    options: Vec<String>, // Assuming a simplified type for demonstration
    numbers: Vec<i32>,
    caption: String,
    style: String,
    font: String,
}

// RcDialogExList structure
struct RcDialogExList {
    dialogs: Vec<RcDialogEx>,
}

impl RcDialogExList {
    fn new() -> Self {
        RcDialogExList { dialogs: vec![] }
    }

    fn add(&mut self, dialog: RcDialogEx) {
        self.dialogs.push(dialog);
    }

    fn output(&self) {
        for dialog in &self.dialogs {
            println!("void qtMfcInitDialogResource_{}(CDialog* parent) {{", dialog.id);
            println!("    // Dialog logic here...");
        }
        println!("void qtMfcInitDialogResource(UINT dlgID, CDialog* parent) {{");
        println!("    switch (dlgID) {{");
        for dialog in &self.dialogs {
            println!("        case {}:", dialog.id);
            println!("            qtMfcInitDialogResource_{}(parent);", dialog.id);
            println!("            break;");
        }
        println!("        case 0:");
        println!("            // Allow blank dialogs.");
        println!("            break;");
        println!("        default:");
        println!("            qFatal(\"dialog resource not implemented...\");");
        println!("    }}");
        println!("}}");
    }
}

// Read lines from a file and return an iterator
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Parse .rc file and populate lists
fn parse_rc_file(file_path: &str) -> (
    RcBitmapList, RcIconList, RcStringTableList, RcToolbarList, RcAcceleratorTableList, RcMenuList, RcDialogExList
) {
    let bitmap_list = RcBitmapList::new();
    let mut icon_list = RcIconList::new();
    let string_table_list = RcStringTableList::new();
    let toolbar_list = RcToolbarList::new();
    let accelerator_table_list = RcAcceleratorTableList::new();
    let menu_list = RcMenuList::new();
    let dialog_ex_list = RcDialogExList::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(line) = line {
                let line = line.trim();
                // Use regex to parse the line and populate the corresponding list
                let icon_re = Regex::new(r#"(\w+)\s+ICON\s+"(.+)""#).unwrap();
                if let Some(caps) = icon_re.captures(line) {
                    icon_list.add(RcIcon {
                        id: caps[1].to_string(),
                        file: caps[2].to_string(),
                    });
                }
                // Similarly, parse other types (bitmap, string table, toolbar, etc.)
            }
        }
    }

    (bitmap_list, icon_list, string_table_list, toolbar_list, accelerator_table_list, menu_list, dialog_ex_list)
}

// Create .qrc file
fn create_qrc_file(resources: &HashMap<String, String>, output_path: &str) {
    let file = File::create(output_path).unwrap();
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(file);

    writer.write(XmlEvent::start_element("RCC")).unwrap();
    writer
        .write(XmlEvent::start_element("qresource").attr("prefix", "/"))
        .unwrap();

    for (_name, path) in resources {
        writer.write(XmlEvent::start_element("file")).unwrap();
        writer.write(XmlEvent::characters(path)).unwrap();
        writer.write(XmlEvent::end_element()).unwrap(); // file
    }

    writer.write(XmlEvent::end_element()).unwrap(); // qresource
    writer.write(XmlEvent::end_element()).unwrap(); // RCC
}

// Create .ui file
fn create_ui_file(dialog_name: &str, controls: &Vec<String>, output_path: &str) {
    let file = File::create(output_path).unwrap();
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(file);

    writer.write(XmlEvent::start_element("ui").attr("version", "4.0")).unwrap();
    writer.write(XmlEvent::start_element("class")).unwrap();
    writer.write(XmlEvent::characters(dialog_name)).unwrap();
    writer.write(XmlEvent::end_element()).unwrap(); // class

    writer.write(XmlEvent::start_element("widget").attr("class", "QWidget").attr("name", dialog_name)).unwrap();
    writer.write(XmlEvent::start_element("property").attr("name", "geometry")).unwrap();
    writer.write(XmlEvent::start_element("rect")).unwrap();
    writer.write(XmlEvent::start_element("x")).unwrap();
    writer.write(XmlEvent::characters("0")).unwrap();
    writer.write(XmlEvent::end_element()).unwrap(); // x
    writer.write(XmlEvent::start_element("y")).unwrap();
    writer.write(XmlEvent::characters("0")).unwrap();
    writer.write(XmlEvent::end_element()).unwrap(); // y
    writer.write(XmlEvent::start_element("width")).unwrap();
    writer.write(XmlEvent::characters("400")).unwrap();
    writer.write(XmlEvent::end_element()).unwrap(); // width
    writer.write(XmlEvent::start_element("height")).unwrap();
    writer.write(XmlEvent::characters("300")).unwrap();
    writer.write(XmlEvent::end_element()).unwrap(); // height
    writer.write(XmlEvent::end_element()).unwrap(); // rect
    writer.write(XmlEvent::end_element()).unwrap(); // property

    let control_re = Regex::new(r#"CONTROL\s+"(.+)",\s*(\d+),\s*BUTTON,\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+)"#).unwrap();
    for control in controls {
        if let Some(caps) = control_re.captures(control) {
            let text = &caps[1];
            let control_id = &caps[2];
            let x = &caps[3];
            let y = &caps[4];
            let width = &caps[5];
            let height = &caps[6];

            writer.write(XmlEvent::start_element("widget").attr("class", "QPushButton").attr("name", &format!("button_{}", control_id))).unwrap();
            writer.write(XmlEvent::start_element("property").attr("name", "geometry")).unwrap();
            writer.write(XmlEvent::start_element("rect")).unwrap();
            writer.write(XmlEvent::start_element("x")).unwrap();
            writer.write(XmlEvent::characters(x)).unwrap();
            writer.write(XmlEvent::end_element()).unwrap(); // x
            writer.write(XmlEvent::start_element("y")).unwrap();
            writer.write(XmlEvent::characters(y)).unwrap();
            writer.write(XmlEvent::end_element()).unwrap(); // y
            writer.write(XmlEvent::start_element("width")).unwrap();
            writer.write(XmlEvent::characters(width)).unwrap();
            writer.write(XmlEvent::end_element()).unwrap(); // width
            writer.write(XmlEvent::start_element("height")).unwrap();
            writer.write(XmlEvent::characters(height)).unwrap();
            writer.write(XmlEvent::end_element()).unwrap(); // height
            writer.write(XmlEvent::end_element()).unwrap(); // rect
            writer.write(XmlEvent::end_element()).unwrap(); // property

            writer.write(XmlEvent::start_element("property").attr("name", "text")).unwrap();
            writer.write(XmlEvent::characters(text)).unwrap();
            writer.write(XmlEvent::end_element()).unwrap(); // property

            writer.write(XmlEvent::end_element()).unwrap(); // widget
        }
    }

    writer.write(XmlEvent::end_element()).unwrap(); // widget
    writer.write(XmlEvent::end_element()).unwrap(); // ui
}

fn main() {
    let matches = Command::new("rc2qt")
        .version("0.01-alpha")
        .author("Guillaume Gielly <guillaume.gielly@gmail.com>")
        .about("Converts .rc files to Qt project files")
        .arg(Arg::new("rcfile")
            .help("Sets the input .rc file")
            .required(true)
            .index(1))
        .arg(Arg::new("output")
            .help("Sets the output directory")
            .required(true)
            .index(2))
        .get_matches();

    let rc_file_path = matches.get_one::<String>("rcfile").unwrap();
    let output_dir = matches.get_one::<String>("output").unwrap();

    let (bitmap_list, icon_list, string_table_list, toolbar_list, accelerator_table_list, menu_list, dialog_ex_list) = parse_rc_file(rc_file_path);

    // Output each list
    bitmap_list.output();
    icon_list.output();
    string_table_list.output();
    toolbar_list.output();
    accelerator_table_list.output();
    menu_list.output();
    dialog_ex_list.output();

    // Example for creating a .qrc file:
    let resources: HashMap<String, String> = HashMap::new(); // Populate this with actual resource data
    let qrc_output_path = format!("{}/resources.qrc", output_dir);
    create_qrc_file(&resources, &qrc_output_path);

    // Example for creating a .ui file:
    let dialog_name = "ExampleDialog";
    let controls = vec!["CONTROL \"Button\", IDC_BUTTON1, BUTTON, WS_TABSTOP, 10, 10, 50, 14".to_string()];
    let output_path = format!("{}/{}.ui", output_dir, dialog_name);
    create_ui_file(dialog_name, &controls, &output_path);
}
