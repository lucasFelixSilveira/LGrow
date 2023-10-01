use std::process; // Process methods and informations
use colored::*; // Import colors methods

pub fn error(msg: String) {
    print!("{}\n{}", "An error happened in your code".yellow(), "error".red());
    println!(": {}", msg);
    process::exit(1);
}
// This function shows the user any mistake made by him or at the time of executing the code.