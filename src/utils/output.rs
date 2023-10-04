use std::process; // Process methods and informations
use colored::*; // Import colors methods

// This function shows the user any mistake made by
// him or at the time of executing the code.
pub fn error(msg: String) {
    print!("{}\n{}", "An error happened in your code".yellow(), "error".red());
    println!(": {}", msg);
    process::exit(1);
}

pub fn sucess(msg: String, process: &str) {
    println!("{} - {}: {msg}", "Sucess".green(), process.blue());
}

pub fn processing(msg: String, process: &str) {
    println!("{} - {}: {msg}", "Processing".yellow(), process.blue());
}