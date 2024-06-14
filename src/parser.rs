use regex::Regex;
use std::io::{self, BufRead};
use std::path::Path;
use crate::resource::*;
use log::{info, warn, error}; // Import logging macros

pub fn parse_rc_file(file_path: &str) -> (
    RcBitmapList, RcIconList, RcStringTableList, RcToolbarList, RcAcceleratorTableList, RcMenuList, RcDialogExList
) {
    let mut bitmap_list = RcBitmapList::new();
    let mut icon_list = RcIconList::new();
    let mut string_table_list = RcStringTableList::new();
    let mut toolbar_list = RcToolbarList::new();
    let mut accelerator_table_list = RcAcceleratorTableList::new();
    let mut menu_list = RcMenuList::new();
    let mut dialog_ex_list = RcDialogExList::new();

    if let Ok(mut lines) = read_lines(file_path) {
        while let Some(Ok(line)) = lines.next() {
            let line = line.trim();

            // Example for bitmaps:
            let bitmap_re = Regex::new(r#"(\w+)\s+BITMAP\s+"(.+)""#).unwrap();
            if let Some(caps) = bitmap_re.captures(line) {
                let id = caps[1].to_string();
                let file = caps[2].to_string();

                // Logging information
                info!("Parsing bitmap: ID = {}, File = {}", id, file);

                // Basic validation: check if file path ends with ".bmp"
                if !file.ends_with(".bmp") {
                    warn!("Bitmap file '{}' does not have a .bmp extension", file);
                }

                // Optional: Normalize file paths, e.g., replace backslashes with forward slashes
                let normalized_file = file.replace("\\", "/");

                // Extract additional attributes
                let mut width: Option<u32> = None;
                let mut height: Option<u32> = None;
                let mut color_depth: Option<u32> = None;
                let mut compression: Option<String> = None;
                let mut palette: Option<String> = None;
                let mut dpi: Option<u32> = None;
                let mut color_mode: Option<String> = None;
                let mut compression_level: Option<u32> = None;
                let mut author: Option<String> = None;

                // Look ahead for additional attributes
                while let Some(Ok(attr_line)) = lines.next() {
                    let attr_line = attr_line.trim();
                    if attr_line.starts_with("END") {
                        break;
                    }

                    if let Some(caps) = Regex::new(r#"WIDTH\s+(\d+)"#).unwrap().captures(attr_line) {
                        width = Some(caps[1].parse().unwrap());
                    } else if let Some(caps) = Regex::new(r#"HEIGHT\s+(\d+)"#).unwrap().captures(attr_line) {
                        height = Some(caps[1].parse().unwrap());
                    } else if let Some(caps) = Regex::new(r#"COLOR_DEPTH\s+(\d+)"#).unwrap().captures(attr_line) {
                        color_depth = Some(caps[1].parse().unwrap());
                    } else if let Some(caps) = Regex::new(r#"COMPRESSION\s+(.+)"#).unwrap().captures(attr_line) {
                        compression = Some(caps[1].to_string());
                    } else if let Some(caps) = Regex::new(r#"PALETTE\s+(.+)"#).unwrap().captures(attr_line) {
                        palette = Some(caps[1].to_string());
                    } else if let Some(caps) = Regex::new(r#"DPI\s+(\d+)"#).unwrap().captures(attr_line) {
                        dpi = Some(caps[1].parse().unwrap());
                    } else if let Some(caps) = Regex::new(r#"COLOR_MODE\s+(.+)"#).unwrap().captures(attr_line) {
                        color_mode = Some(caps[1].to_string());
                    } else if let Some(caps) = Regex::new(r#"COMPRESSION_LEVEL\s+(\d+)"#).unwrap().captures(attr_line) {
                        compression_level = Some(caps[1].parse().unwrap());
                    } else if let Some(caps) = Regex::new(r#"AUTHOR\s+(.+)"#).unwrap().captures(attr_line) {
                        author = Some(caps[1].to_string());
                    } else {
                        warn!("Unknown attribute for bitmap '{}': {}", id, attr_line);
                    }
                }

                // Add bitmap to the list
                bitmap_list.add(RcBitmap {
                    id,
                    file: normalized_file,
                    width,
                    height,
                    color_depth,
                    compression,
                    palette,
                    dpi,
                    color_mode,
                    compression_level,
                    author,
                });

                continue;
            }

            // Example for icons:
            let icon_re = Regex::new(r#"(\w+)\s+ICON\s+"(.+)""#).unwrap();
            if let Some(caps) = icon_re.captures(line) {
                icon_list.add(RcIcon {
                    id: caps[1].to_string(),
                    file: caps[2].to_string(),
                });
                continue;
            }

            // Example for string tables:
            let string_table_re = Regex::new(r#"\s*STRINGTABLE\s*BEGIN"#).unwrap();
            if string_table_re.is_match(line) {
                let mut string_items = Vec::new();
                while let Some(Ok(line)) = lines.next() {
                    let line = line.trim();
                    if line == "END" {
                        break;
                    }
                    let string_item_re = Regex::new(r#"(\w+)\s+"(.+)""#).unwrap();
                    if let Some(caps) = string_item_re.captures(line) {
                        string_items.push(RcStringTableItem {
                            id: caps[1].to_string(),
                            text: caps[2].to_string(),
                        });
                    }
                }
                string_table_list.add(RcStringTable::new(string_items));
                continue;
            }

            // Example for toolbars:
            let toolbar_re = Regex::new(r#"(\w+)\s+TOOLBAR\s+"(.+)""#).unwrap();
            if let Some(caps) = toolbar_re.captures(line) {
                toolbar_list.add(RcToolbar {
                    id: caps[1].to_string(),
                    numbers: Vec::new(),
                    table: vec![caps[2].to_string()],
                });
                continue;
            }

            // Example for accelerator tables:
            let accelerator_table_re = Regex::new(r#"\s*ACCELERATORS\s*BEGIN"#).unwrap();
            if accelerator_table_re.is_match(line) {
                let mut accelerator_items = Vec::new();
                while let Some(Ok(line)) = lines.next() {
                    let line = line.trim();
                    if line == "END" {
                        break;
                    }
                    accelerator_items.push(line.to_string()); // Simplified
                }
                accelerator_table_list.add(RcAcceleratorTable {
                    id: String::new(),
                    table: accelerator_items,
                });
                continue;
            }

            // Example for menus:
            let menu_re = Regex::new(r#"\s*MENU\s*BEGIN"#).unwrap();
            if menu_re.is_match(line) {
                let mut menu_items = Vec::new();
                while let Some(Ok(line)) = lines.next() {
                    let line = line.trim();
                    if line == "END" {
                        break;
                    }
                    menu_items.push(line.to_string()); // Simplified
                }
                menu_list.add(RcMenu {
                    id: String::new(),
                    table: menu_items,
                });
                continue;
            }

            // Example for dialogs:
            let dialog_re = Regex::new(r#"(\w+)\s+DIALOGEX"#).unwrap();
            if let Some(caps) = dialog_re.captures(line) {
                let dialog_id = caps[1].to_string();
                let mut dialog_items = Vec::new();
                while let Some(Ok(line)) = lines.next() {
                    let line = line.trim();
                    if line == "END" {
                        break;
                    }
                    dialog_items.push(line.to_string()); // Simplified
                }
                dialog_ex_list.add(RcDialogEx {
                    id: dialog_id,
                    items: dialog_items,
                    options: Vec::new(),
                    numbers: Vec::new(),
                    caption: String::new(),
                    style: String::new(),
                    font: String::new(),
                });
            }
        }
    }

    (
        bitmap_list, icon_list, string_table_list, toolbar_list, accelerator_table_list, menu_list, dialog_ex_list
    )
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
