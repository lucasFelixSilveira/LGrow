use std::fs; // Module used to read the contents of the LGrow file

// Importing folders
use crate::steps::{self, lang::generate::Module, project::validation::Package};

#[derive(Debug)]
pub struct LGrowFile {
    pub path: String,
    pub content: String,
    pub imports: String,
}

pub fn main(directory: String, files: &mut Vec<LGrowFile>, is_module: bool) -> (String, Vec<Module>, String) {

    // Execution of the main functions of the language
    // in correct order. Each function has a certain
    // configuration of parameters that are important
    // to avoid discrepancies between reading the main
    // file and reading other files.
    let mut project: steps::project::validation::Project = steps::project::validation::Project {
        directory: String::from(&directory), package: Package {
            directory: String::new(), name: String::new(), version: String::new()
        }, main: format!("{}", directory)
    };
    
    if !is_module {
        project = steps::project::validation::validate(&directory);
    }

    files.push(LGrowFile {
        path: String::from(&directory),
        content: String::new(),
        imports: String::new()
    });

    let code: String = fs::read_to_string(&project.main).unwrap_or_else(|_| { String::new() });
    let lexical: Vec<steps::lang::lexical::Token> = steps::lang::lexical::analizer(code).unwrap();
    let _: bool = steps::lang::syntax::analizer(&lexical, project.main, &directory);
    let code: (String, Vec<Module>, String) = steps::lang::generate::generator(lexical, directory, is_module, files);

    code
}   