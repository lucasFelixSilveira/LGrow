use std::fs;

pub fn crate_target(project_path: &String) {
    let b = std::path::MAIN_SEPARATOR;

    let folder_path = format!("{}{b}target", project_path);
    let target = fs::read_dir(folder_path.clone()).is_ok();
    if !target {
        let _ = fs::create_dir(folder_path).is_ok();
    }
}

pub fn create_bin(project_path: &String) {
    let b = std::path::MAIN_SEPARATOR;

    let folder_path = format!("{}{b}target{b}bin", project_path);
    let target = fs::read_dir(folder_path.clone()).is_ok();
    if !target {     
        let _ = fs::create_dir(folder_path).is_ok();
    }
}

pub fn create_write_c_file(project_path: &String, filename: String, content: String) {
    let b = std::path::MAIN_SEPARATOR;

    let folder_path = format!("{}{b}target{b}bin{b}{}.c", project_path, filename);
    let _ = fs::write(folder_path, content).is_ok();
}

pub fn create_write_h_file(project_path: &String, filename: String, content: String) {
    let b = std::path::MAIN_SEPARATOR;

    let folder_path = format!("{}{b}target{b}bin{b}{}.h", project_path, filename);
    let target = fs::read(folder_path.clone()).is_ok();
    if !target {     
        let _ = fs::write(folder_path, content).is_ok();
    }
}


pub fn delete_bin(project_path: &String) {
    let b = std::path::MAIN_SEPARATOR;

    let folder_path = format!("{}{b}target{b}bin", project_path);
    let target = fs::read_dir(folder_path.clone()).is_ok();
    if target {
        let _ = fs::remove_dir(folder_path).is_ok();
    }
}