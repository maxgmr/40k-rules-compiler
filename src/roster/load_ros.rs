use minidom::Element;
use std::fs;

pub fn read_ros(file_name: &String) -> std::io::Result<String> {
    let ros_string = fs::read_to_string(file_name)?;
    return Ok(ros_string);
}

pub fn get_xml(s: &String) -> Element {
    s.parse().unwrap()
}
