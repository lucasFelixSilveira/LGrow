use crate::{steps::lang::lexical::Token, utils::file::valid};

use colored::*;

// use crate::language;
use crate::utils;


// Declaration of classes and structures for error sampling. 
// They are important for sampling the entire line where the
// error occurred, in addition to being able to point out the
// locations where the error occurs, something very important
// for the compiler and its syntactic analysis phase. 
pub struct Location {
    pub line: usize,
    pub col: usize,
    pub file: String,
}

// Here happens the collection and return of the location 
// of the error in the file.
impl Location {
    
    // Return of data
    pub fn from_token(token: &Token, file: String) -> Self {
        Location {
            file: file,
            line: token.line,
            col: token.col,
        }
    }

    // Reading and collecting data
    pub fn read_line(&self) -> std::io::Result<String> {
        Ok(std::fs::read_to_string(&self.file)?
            .lines()
            .nth(self.line - 1)
            .expect("line number exceeds file line number")
            .to_string())
    }
}

// Formats correctly how to show the location of the error 
// in the file
impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}:{}:{}", self.file, self.line, self.col))?;
        Ok(())
    }
}


pub fn is_string(file: &Token) -> bool {
    file.content.starts_with('"') && file.content.ends_with('"')
}

pub fn string_value(file: &Token) -> String {
    file.content[1..file.content.len()-1].to_string()
}


// Function where the magic of syntactic analysis happens. Here checks, 
// validations take place and we see if it is compatible with the
// standard syntax of the language.
pub fn analizer(tokens: &Vec<Token>, file: String) -> bool {
    // Stores the index where the parser is. The list comes from Syntax Analyzer. 
    let mut token_counter: usize = 0;
    // Separates the file name from the full project directory.
    let filename = file.rsplit_once(std::path::MAIN_SEPARATOR).unwrap().1.to_string();
    
    let mut keys = 0;

    // Function used to generate "pretty" errors for the user, this
    // allows a better understanding of where and how the error occurred.
    let point_err = |token: &Token, file: String, reason: &str| {
        // Check if it is the spare token, if it is, ignore it.
        if token.content != "__ignore@__0x10010" {

            // Collects error location, column, row and file.
            let location = Location::from_token(token, file);
            let Ok(line) = location.read_line() else {
                utils::output::error(format!("syntax error at {}: {reason}\n  (failed to read file)", location));
                unreachable!()
            };

            // It uses the previously collected location to improve
            // itself using this formatted String.
            let local = format!("{}:{}:{}", 
                filename,
                location.col, location.line
            );

            // Error appearance generation
            let col_indicator = (|| -> String {
                if token.col >= 4 { return " ".repeat(token.col-4) + &"^".red().bold() };
                " ".repeat(token.col) + &"^".red().bold()
            })();
            let line_number_str = location.line.to_string();
            let indicator_padding  = " ".repeat(10+line_number_str.len());
            utils::output::error(format!(r#"syntax error at {}: {reason}
    {} {} {line}
{indicator_padding}{col_indicator}"#, local, line_number_str.bright_blue().bold(), "|".bright_blue().bold()));
        }
    }; 

    let other_err = |reason: &str| {
        utils::output::error(format!("syntax alert at {filename}:\n{} {reason}", ">".yellow()));        
    }; 

    // Function to search for the next valid token within the list
    // of Tokens. Use the "Token Counter" utility
    let next_token = |token_counter: &mut usize, is_keyword: bool| {
        let token = match tokens.get(*token_counter) {
            Some(token) => token,
            None => {
                let real_last = &tokens[*token_counter - 2];
                point_err(real_last, file.clone(), "Apparently the code is incomplete.");
                panic!("No has error");
            }
        };
        *token_counter += 1;

        if token.content == "__ignore@__0x10010" && !is_keyword {
            let real_last = &tokens[*token_counter - 2];
            point_err(real_last, file.clone(), "Apparently the code is incomplete.");
        }

        token
    };

    // This while makes us pass can within each Token that this list
    // contains. This allows us to check for syntax errors in your
    // code and report the error.
    while token_counter < tokens.len() {

        // Collection of the "keyword", or "initial token"
        let keyword = next_token(&mut token_counter, true);
        
        // Makes a sequence of checks on top of this token.
        match keyword.content.as_str() {
            
            // If it is a function, it will perform function verification runs
            "fun" | "void" => {

                // saves whether it is a void or a normal function
                let is_void = keyword.content == "void";

                // saves the function name token inside this variable
                let name_or_p = next_token(&mut token_counter, false);

                // Checks whether it is an unnamed function, or a named function.
                match name_or_p.content.as_str() {

                    _ => {

                        if !utils::variables::is_valid_var(name_or_p) {
                            point_err(name_or_p, file.clone(), "The name entered is invalid");
                        }
                    
                        if name_or_p.content == "main" && keyword.content != "void" {
                            point_err(keyword, file.clone(), "The Main function cannot be a function, only a void");
                        }

                        let p = next_token(&mut token_counter, false);
                        if p.content != "(" {
                            point_err(p, file.clone(), "Expected '('");
                        }

                        let mut first_param = true;

                        loop {

                            let mut p = next_token(&mut token_counter, false);
                            
                            if p.content == ")" {
                                
                                let key = next_token(&mut token_counter, false);

                                if key.content != "{" {
                                    point_err(key, file.clone(), &format!("In \"{}()\" it was expected after: {{", keyword.content));
                                }   

                                if !is_void {


                                    let equals = next_token(&mut token_counter, false);
                                    let x = || { point_err(equals, file.clone(), "After declaring a normal function, it is necessary to inform a return type, the \"Keyword\" that indicates the declaration of this return type is \"=>\" use after opening the function's keys."); };
                                    if equals.content != "=" { x() }

                                    let m = next_token(&mut token_counter, false);
                                    if m.content != ">" { x() }

                                    let type_ = next_token(&mut token_counter, false);
                                    let mut avance = 0;
                                    
                                    loop {
                                        let t = &tokens[token_counter + avance];
                                        if 
                                            t.content == "["
                                            || t.content == "]" {
                                                avance += 1;
                                        } else {
                                            token_counter += avance;
                                            break;
                                        }
                                    }

                                    if !utils::types::is_valid_type(type_) { 
                                        point_err(type_, file.clone(), &format!("Type \"{}\" was not found.", type_.content));
                                    }

                                }

                                keys += 1;

                                break;
                            }

                            if !first_param {
                                
                                if p.content != "," {
                                    point_err(p, file.clone(), "Expected ','")
                                }

                                p = next_token(&mut token_counter, false);

                            }

                            let param = &p;
                            if !utils::variables::is_valid_var(param) { point_err(param, file.clone(), "The paremether name entered is invalid"); }

                            let collon = next_token(&mut token_counter, false);
                            if collon.content != ":" {
                                point_err(p, file.clone(), "Expected 'param: Type'");
                            }

                            let type_ = next_token(&mut token_counter, false);

                            if !utils::types::is_valid_type(type_) { 
                                point_err(type_, file.clone(), &format!("Type \"{}\" was not found.", type_.content));
                            }

                            let mut avance = 0;
                            loop {
                                let t = &tokens[token_counter + avance];
                                if 
                                    t.content == "["
                                    || t.content == "]" {
                                        avance += 1;
                                } else {
                                    token_counter += avance;
                                    break;
                                }
                            }
                            
                            first_param = false;

                        }   

                    }
                }
            },

            "import" => {

                let content = next_token(&mut token_counter, false);

                match content.content.as_str() {
                    "file" => {
                        
                        let file_name = next_token(&mut token_counter, false);

                        if !is_string(file_name) {
                            point_err(file_name, file.clone(), "The content provided after the \"file\" keyword in an import must be a String.")
                        }
                        
                        if !utils::file::valid(&string_value(file_name)) {
                            point_err(file_name, file.clone(), "You need to provide a \".lgw\" file")
                        }

                        let comlon = next_token(&mut token_counter, false);
                        
                        if comlon.content != ";" {
                            point_err(file_name, file.clone(), "';' was expected to end the expression.")
                        }

                    },
                    _ => {
                        if is_string(content) {
                            let cont = string_value(content);

                            // println!("{}", cont);
                            
                            if [
                                String::from("fmt")
                            ].contains(&cont) {
                                let collon = next_token(&mut token_counter, false);
                                if collon.content != ";" {
                                    point_err(collon, file.clone(), "Please finish the import using ';'")
                                }
                            }

                        }
                    }
                }

            },

            "const" | "let" => {
                let mut prefix = "";
                if keyword.content == "const" { prefix = "static" }

                let varname = next_token(&mut token_counter, false);
                let collon = next_token(&mut token_counter, false);
            }

            "}" if keys > 0 => { { keys -= 1; } },

            _ => point_err(keyword, file.clone(), "Unkdown keyword")
        }
    }

    if keys > 0 { other_err("You have not closed all possible open keys in your code.") }

    true

}