use std::process::Command;
use crate::steps::project::template::{create_util, self, delete_util};
use crate::{utils::*, assets};
use std::thread;
use std::time::Duration;

pub fn runtime(files: Vec<String>, path: &String) {
    let b = std::path::MAIN_SEPARATOR;
    let target_f = &format!("{}{b}target", path);

    thread::sleep(Duration::from_secs(4));

    let mut binding: Command = Command::new("gcc");
    let output: &mut Command = binding
        .arg("-o")
        .arg("compiled")
        .arg("main.c");

    for file in files {
        output.arg(file);
    }

    let result: std::process::Output = output
        .current_dir(format!("{}/target/bin", path))
        .output()
        .expect("Falha ao executar o comando");

    if result.status.success() {
        
        output::sucess("Your project finishes the compilation phase. Let's move on to executing the last processes.".to_string(), "Compiler");
        create_util(path, "move".to_string(), assets::storage::get_move());
    
        let script_path = "move.sh";

        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", script_path])
                .current_dir(target_f)
                .output()
        } else {
            Command::new("sh")
                .arg(script_path)
                .current_dir(target_f)
                .output()
        };

        match output {
            Ok(_) => {
                template::delete_bin(path);
                delete_util(target_f, "move".to_string());
                output::sucess("All stages have been completed!".to_string(), "LGrow");
                output::processing("Creating and running the \"Pamunho\" chatbot...".to_string(), "Pamunho");

                thread::sleep(Duration::from_secs(4));
                pamunho::ask("Do you want to run the program?", vec!["Yes", "No"], path, -8);
            }
            Err(error) => {
                println!("Error executing script: {}", error);
            }
        }

    
    } else {
        let error_message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&result.stderr);
        output::error(format!("Erro ao compilar: {}", error_message));
    }
}