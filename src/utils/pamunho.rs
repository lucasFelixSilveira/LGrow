use colored::*;
use std::io;
use whoami;
use std::thread;
use std::time::Duration;
use std::process::Command;

use crate::steps::project::template::{create_util, delete_util};
use crate::{utils, assets};

use super::output::sucess;

pub fn ask(ask: &str, responses: Vec<&str>, ppath: &String, id: i8) {
    let username = whoami::username();
    
    let make = || {
        println!("🐹 💬 - {}: Hello {username}, {} [{}]", "Pamunho says".red(), ask, responses.join("/").yellow());
        println!("😃 💬 - {}:", format!("{username} says").blue());

        thread::sleep(Duration::from_secs(1));

        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Falha ao ler a entrada");
        answer = answer.to_lowercase();

        let content: Vec<String> = responses.iter().map(|x| x.to_lowercase()).collect();
        let is_valid = content.join(" ").contains(&answer.to_lowercase().trim());
        if is_valid {
            make_do(id, &answer.trim().to_lowercase(), ppath);
        } else {
            let _ = utils::pamunho::ask(ask, responses, ppath, id);
        }
    };

    make();
}

pub fn make_do(att: i8, answer: &str, ppath: &String) {
    let target_f = format!("{}{}target", ppath, std::path::MAIN_SEPARATOR);
    
    if answer == "yes" && att == -8 {
        
        if cfg!(target_os = "windows") {
            std::process::Command::new("cmd").arg("/c").arg("cls").status().unwrap();
        } else {
            std::process::Command::new("clear").status().unwrap();
        }

        create_util(ppath, "run".to_string(), assets::storage::get_run_and_print());
        sucess("Running the executable.".to_string(), "LGrow");

        thread::sleep(Duration::from_secs(3));

        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", "run.sh"])
                .current_dir(&target_f)
                .output()
        } else {
            Command::new("sh")
                .arg("run.sh")
                .current_dir(&target_f)
                .output()
        };

        match output {
            Ok(_) => {
                sucess("All finished, thank you.".to_string(), "LGrow");
                delete_util(&String::from(target_f), "run".to_string());
            }
            Err(error) => {
                println!("Error executing script: {}", error);
            }
        }
    } else if answer == "no" && att == -8 {
        sucess("All finished, thank you.".to_string(), "LGrow");
    }
}