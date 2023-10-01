/// Search for a field inside the LGXT file
/// `package` is the LGXT file contents, `section` is the section to look for and `value` is the field inside the section
pub fn collect(package: &String, section: &str, value: &str) -> Option<String> {
    let mut current_section: String = "".to_string();
    fn get_section_name(line: &str) -> String {
        line[2..line.len()-2].to_string()
    }
    for line in package.lines() {
        if line.starts_with("[\"") && line.ends_with("\"]") {
            current_section = get_section_name(line);
        }
        if section == current_section && line.starts_with(value) {
            let (_, value) = line.split_once(" = ").unwrap();
            return Some(value[1..value.len()-1].to_string());
        }
    } 
    None
}