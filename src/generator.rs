extern crate xml;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use xml::writer::{EmitterConfig, XmlEvent}; // Import the Regex module

pub fn create_qrc_file(resources: &HashMap<String, String>, output_path: &str) {
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


pub fn create_ui_file(dialog_name: &str, controls: &Vec<String>, output_path: &str) {
    let file = File::create(output_path).unwrap();
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(file);

    writer
        .write(XmlEvent::start_element("ui").attr("version", "4.0"))
        .unwrap();
    writer.write(XmlEvent::start_element("class")).unwrap();
    writer.write(XmlEvent::characters(dialog_name)).unwrap();
    writer.write(XmlEvent::end_element()).unwrap(); // class

    writer
        .write(
            XmlEvent::start_element("widget")
                .attr("class", "QWidget")
                .attr("name", dialog_name),
        )
        .unwrap();
    writer
        .write(XmlEvent::start_element("property").attr("name", "geometry"))
        .unwrap();
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

    let control_re =
        Regex::new(r#"CONTROL\s+"(.+)",\s*(\d+),\s*BUTTON,\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+)"#)
            .unwrap();
    for control in controls {
        if let Some(caps) = control_re.captures(control) {
            let text = &caps[1];
            let control_id = &caps[2];
            let x = &caps[3];
            let y = &caps[4];
            let width = &caps[5];
            let height = &caps[6];

            writer
                .write(
                    XmlEvent::start_element("widget")
                        .attr("class", "QPushButton")
                        .attr("name", &format!("button_{}", control_id)),
                )
                .unwrap();
            writer
                .write(XmlEvent::start_element("property").attr("name", "geometry"))
                .unwrap();
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

            writer
                .write(XmlEvent::start_element("property").attr("name", "text"))
                .unwrap();
            writer.write(XmlEvent::characters(text)).unwrap();
            writer.write(XmlEvent::end_element()).unwrap(); // property

            writer.write(XmlEvent::end_element()).unwrap(); // widget
        }
    }

    writer.write(XmlEvent::end_element()).unwrap(); // widget
    writer.write(XmlEvent::end_element()).unwrap(); // ui
}
