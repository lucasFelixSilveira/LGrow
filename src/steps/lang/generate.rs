use crate::{steps::{lang::lexical::Token, self}, utils, read::LGrowFile};

pub struct Module {
    pub name: String,
    pub standard: bool,
    pub lgrow_file: bool
}

// Function where the magic of syntactic analysis happens. Here checks, 
// validations take place and we see if it is compatible with the
// standard syntax of the language.
pub fn generator(tokens: Vec<Token>, directory: String, is_module: bool, files: &mut Vec<LGrowFile>) -> (String, Vec<Module>) {
    // Stores the index where the parser is. The list comes from Lexical Analyzer. 
    let mut token_counter: usize = 0;

    // The code generated has saved here
    let mut final_code: String = String::from("#include <stdio.h>");
    let mut modules: Vec<Module> = vec![];

    // Function to search for the next valid token within the list
    // of Tokens. Use the "Token Counter" utility
    let next_token = |token_counter: &mut usize| {
        let token = match tokens.get(*token_counter) {
            Some(token) => token,
            None => {
                panic!("No has error");
            }
        };
        *token_counter += 1;
        token
    };

    let mut keys = 0;
    let mut is_main = false;

    // This while makes us pass can within each Token that this list
    // contains. This allows us to check for syntax errors in your
    // code and report the error.
    while token_counter < tokens.len() {

        // Collection of the "keyword", or "initial token"
        let keyword = next_token(&mut token_counter);
        
        // Makes a sequence of checks on top of this token.
        match keyword.content.as_str() {
            
            // If it is a function, it will perform function verification runs
            "fun" | "void" => {
                let is_void = keyword.content == "void";
                
                let mut prefix: String = "".to_string();

                let mut def_prefix = |x: &str| { prefix = x.to_string() };

                let function_name = next_token(&mut token_counter);
                let name_fun = String::from(&function_name.content);
                if function_name.content == "main" {
                    def_prefix("int");
                    is_main = true;
                }
                else if is_void { def_prefix("void") }

                token_counter += 1;

                let mut params = String::new(); 

                let mut first_param = true;

                loop {

                    let mut p = next_token(&mut token_counter);
                    if p.content == ")" {
                        token_counter += 1;

                        if !is_void {

                            token_counter += 2;

                            let type_ = next_token(&mut token_counter);
                            let mut is_array = false;

                            let array_definer = &tokens[token_counter];
                            if array_definer.content == "[" {
                                token_counter += 2;
                                is_array = true
                            }

                            let _type = utils::types::get_respective_type(type_).to_string();
                            println!("{}", _type);
                            let respective_type = utils::types::get_adaptive_type(_type, &"".to_string(), false, is_array);

                            def_prefix(&respective_type);

                            final_code.push_str(&format!("\n\n{prefix} {}({}) {{", name_fun.clone(), params));
                        } else {
                            final_code.push_str(&format!("\n\n{prefix} {}({}) {{", name_fun.clone(), params));
                        }

                        keys += 1;

                        break;
                    }

                    if !first_param {

                        p = next_token(&mut token_counter);

                        params.push_str(", ")

                    }

                    token_counter += 1;

                    let type_ = next_token(&mut token_counter);
                    let mut is_array = false;

                    let mut _type = utils::types::get_respective_type(type_).to_string();

                    let array_definer = &tokens[token_counter];
                    if array_definer.content == "[" {
                        token_counter += 2;
                        is_array = true
                    }

                    params.push_str(&utils::types::get_adaptive_type(_type, &p.content, true, is_array));

                    first_param = false;

                }

            },
            
            "import" => {

                let content = next_token(&mut token_counter);

                match content.content.as_str() {
                    "file" => {
                        
                        let file_name = next_token(&mut token_counter);

                        final_code.push_str(&format!("\n#include \"{}.h\";", utils::file::get_name(&steps::lang::syntax::string_value(file_name.clone()))));

                        token_counter += 1;

                    },
                    _ => {
                        let cont = steps::lang::syntax::string_value(content);
                    
                        token_counter += 1;
                        final_code.push_str(&format!("\n#include \"{}.h\"", cont));
                        modules.push(Module { name: cont, standard: true, lgrow_file: false })
                    }
                }

            },

            "}" => {

                keys -= 1;
                if keys == 0 && is_main {
                    final_code.push_str("\nreturn 0;");
                    is_main = false;    
                }

                final_code.push_str("\n}");
            },

            _ => {}
        }
    }

    (final_code, modules)

}