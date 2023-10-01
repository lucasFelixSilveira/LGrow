use crate::utils::output::*;
use crate::utils::lgxt::*;
use crate::utils::file::*;
use std::fs;

#[derive(Debug)]
pub struct Package {
    pub directory: String,
    pub name: String,
    pub version: String
}

#[derive(Debug)]
pub struct Project {
    pub directory: String,
    pub package: Package,
    pub main: String
}

pub fn validate(directory: &String) -> Project {
    let b: char = std::path::MAIN_SEPARATOR;

    let p_package = format!("{directory}{b}package.lgxt");
    let package = fs::read_to_string(&p_package).unwrap_or_else(|_| {
        String::new()
    });


    // Verify if exist in package...
    // Project name
    let Some(name) = collect(&package, "Project", "name") else {
        error("The project must have a name declared in the package".to_string());
        unreachable!();
    };
    
    // Project main
    let Some(main) = collect(&package, "Project", "main") else {
        error("The project must have a main declared in the package".to_string());
        unreachable!();  
    };

    let p_main = format!("{}{b}{}", directory, main);
    if valid(&p_main) == false {
        error("The file declared in main in the package must be a .lgw file".to_string());
    }

    if exist(&p_main) == false {
        error("The file declared in main in the package must be valid, that is, you must be sure that it was created.".to_string());
    }

    // Project version
    let Some(version) = collect(&package, "Project", "version") else {
        error("The project must have a version declared in the package.".to_string());
        unreachable!();
    };

    // Project description
    let Some(_) = collect(&package, "Project", "description") else {
        error("The project must have a description declared in the package.".to_string());
        unreachable!();
    };
    
    Project {
        package: Package {
            directory: p_package,
            name, version
        },
        main: p_main,
        directory: directory.to_string()
    }

}