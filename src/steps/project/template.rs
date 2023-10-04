use std::fs;

pub fn crate_target(project_path: &String) {
    let b: char = std::path::MAIN_SEPARATOR;

    let folder_path = format!("{}{b}target", project_path);
    let target = fs::read_dir(folder_path.clone()).is_ok();
    if !target {
        let _ = fs::create_dir(folder_path).is_ok();
    }
}

pub fn create_bin(project_path: &String) {
    let b: char = std::path::MAIN_SEPARATOR;

    let folder_path = format!("{}{b}target{b}bin", project_path);
    let target = fs::read_dir(folder_path.clone()).is_ok();
    if !target {     
        let _ = fs::create_dir(folder_path).is_ok();
    }
}

pub fn create_write_c_file(project_path: &String, filename: String, content: String) {
    let b: char = std::path::MAIN_SEPARATOR;

    let folder_path = format!("{}{b}target{b}bin{b}{}.c", project_path, filename);
    let _ = fs::write(folder_path, content).is_ok();
}

pub fn create_write_h_file(project_path: &String, filename: String, content: String) {
    let b: char = std::path::MAIN_SEPARATOR;

    let folder_path = format!("{}{b}target{b}bin{b}{}.h", project_path, filename);
    let _ = fs::write(folder_path, content).is_ok();
}

pub fn create_util(project_path: &String, filename: String, content: String) {
    let _ = fs::write(format!("{}/target/{}.sh", project_path, filename), content);
}

pub fn delete_util(project_path: &String, filename: String) {
    let b: char = std::path::MAIN_SEPARATOR;

    let path = format!("{}{b}{}.sh", project_path, filename);
    let _ = fs::remove_file(path);
}

pub fn delete_bin(project_path: &String) {
    let b: char = std::path::MAIN_SEPARATOR;

    let folder_path = format!("{}{b}target{b}bin", project_path);
    let _ = fs::remove_dir_all(folder_path);
}