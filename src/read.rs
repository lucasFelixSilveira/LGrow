use std::fs; // Module used to read the contents of the LGrow file

// Importing folders
use crate::steps::{self, lang::generate::Module};

#[derive(Debug)]
pub struct LGrowFile {
    pub path: String,
    pub content: String,
}

pub fn main(directory: String, files: &mut Vec<LGrowFile>) -> (String, Vec<Module>) {

    // Execution of the main functions of the language
    // in correct order. Each function has a certain
    // configuration of parameters that are important
    // to avoid discrepancies between reading the main
    // file and reading other files.
    let project: steps::project::validation::Project = steps::project::validation::validate(&directory);
    let code: String = fs::read_to_string(&project.main).unwrap_or_else(|_| { String::new() });
    let lexical: Vec<steps::lang::lexical::Token> = steps::lang::lexical::analizer(code).unwrap();
    let _: bool = steps::lang::syntax::analizer(&lexical, project.main);
    let code: (String, Vec<Module>) = steps::lang::generate::generator(lexical, directory, false, files);

    (code.0, code.1)
}

pub fn _module(directory: String, file_name: String, files: &mut Vec<LGrowFile>) -> LGrowFile {

    // Configures the path of the new file so that the
    // analysis phases can correctly recognize the code,
    // where these definitions are used to be collected.
    let b: char = std::path::MAIN_SEPARATOR;
    let file_path = format!("{directory}{b}{file_name}");

    // Execution of the main functions of the language
    // in correct order. Each function has a certain
    // configuration of parameters that are important
    // to avoid discrepancies between reading the main
    // file and reading other files.
    let code: String = fs::read_to_string(&file_path).unwrap_or_else(|_| { String::new() });
    let lexical: Vec<steps::lang::lexical::Token> = steps::lang::lexical::analizer(code).unwrap();
    let _: bool = steps::lang::syntax::analizer(&lexical, file_path);
    let code: (String, Vec<Module>) = steps::lang::generate::generator(lexical, directory.clone(), true, files);

    println!("{}", code.0);

    LGrowFile { 
        path: format!("{}{b}{}", directory, file_name),
        content: code.0
    }
}