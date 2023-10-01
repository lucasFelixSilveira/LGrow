/*
    Este código foi desenvolvido por Lucas F. Sil <lucasdwbfff@gmail.com>
    e com colaboração de Tiago Dinis <tiagodinis33@proton.me>

    22/08/2023 - Data de criação do comentário.
*/

use std::fs; // File system

/// This function informs the transpilator if the informed file really exists
pub fn exist(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

// Checks if the file extension is the same as lgw
pub fn valid(path: &str) -> bool {
    let path_buf = std::path::PathBuf::from(path);
    let Some(extension) = path_buf.extension() else {
        return false;
    };
    extension == "lgw"
}

// Checks if the file extension is the same as lgw
pub fn get_name(file_name: &String) -> String {
    let x = file_name.rsplit_once(".").unwrap();
    String::from(x.0)
}