use std::{env, process::exit, fs}; // Import required for CORRECT use of the CLI

// Creating imports so that the functions within each folder/file
// defined here can be accessed in other files
mod read;
mod steps;
mod utils;

fn main() {
    // Collects arguments informed when executing the CLI
    let arguments: Vec<String> = env::args().skip(1).collect();

    let b = std::path::MAIN_SEPARATOR;

    if &arguments[0] == "syncModule" {
        let module = &arguments[1][0..arguments[1].len()];
        let x = env::current_dir().unwrap();
        let mod_folder = format!("{}{b}src{b}steps{b}std{b}{module}", x.display());

        let c_content = fs::read_to_string(format!("{mod_folder}{b}{module}.c")).unwrap().replace('"', "\\\"");
        let h_content = fs::read_to_string(format!("{mod_folder}{b}{module}.h")).unwrap().replace('"', "\\\"");
        let c = format!("pub fn get_c() -> String {{\n    String::from(\"{c_content}\")\n}}");
        let h = format!("pub fn get_h() -> String {{\n    String::from(\"{h_content}\")\n}}");

        let storage = format!("{c}\n\n{h}");   

        let _ = fs::write(format!("{mod_folder}{b}storage.rs"), storage).is_ok();

        exit(1)
    }

    // Collects the first important parameter when executing the CLI,
    // this parameter refers to the directory/reference in which the
    // LGrow project is located. It uses the arguments collected above.
    let mut directory: String = String::new();

    // Execution process for directory collection
    let mut i = 1;
    while i != 0 {
        let x = &arguments[i - 1];
        if x == "0x2" { i = 0 }
        else {
            let mut prefix = "";
            if i != 1 { prefix = " " }
            directory.push_str(&format!("{prefix}{x}"));
            i += 1;     
        }
    }

    steps::project::template::crate_target(&directory);
    steps::project::template::create_bin(&directory);

    // Executes processes in a specific order normalized by other compilers.
    // These processes are necessary for the best performance and execution
    // of the compiler.
    let path = directory.clone();
    let mut files: Vec<read::LGrowFile> = vec![];
    let main_content: (String, Vec<steps::lang::generate::Module>) = read::main(directory, &mut files);
    
    steps::project::template::create_write_c_file(&path, "main".to_string(), main_content.0);
    
    let modules = main_content.1;
    for module in modules {
        if module.standard {
            let m = &module;
            let result = match module.name.as_str() {
                "fmt" => {
                    let c = steps::std::fmt::storage::get_c();
                    let h = steps::std::fmt::storage::get_h();
                    (c, h)
                },
                _ => (String::new(), String::new())
            };

            steps::project::template::create_write_c_file(&path, m.name.to_string(), result.0);
            steps::project::template::create_write_h_file(&path, m.name.to_string(), result.1);
        }
    }

}