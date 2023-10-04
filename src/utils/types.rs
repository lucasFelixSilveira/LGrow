use crate::steps::lang::lexical::Token;

pub fn is_valid_type(type_: &Token) -> bool {
    let x = match type_.content.as_str() {
        "String" => "x".to_string(),
        "Short" => "x".to_string(),
        "Int" => "x".to_string(),
        "Giant" => "x".to_string(),  
        "Byte" | "uShort" => "x".to_string(),
        "uInt" => "x".to_string(),
        "uGiant" => "x".to_string(),     
        "Float" => "x".to_string(),      
        "Double" => "x".to_string(),
        _ => "Undefined".to_string()
    };

    x != "Undefined"
}

pub fn get_adaptive_type(_type: String, param_token: &String, is_param: bool, is_array: bool, response: bool) -> String {
    if is_param {
        match _type.as_str() {
            "char []" => {
                if is_array { format!("char *{param_token}[]") }
                else { 
                    format!("char {param_token}[]") 
                }
            },
            _ => format!("{_type} {param_token}")
        }
    } else {
        match _type.as_str() {
            "char []" => {
                if is_array {
                    String::from("char**")
                } else {
                    if response { String::from("char*") }
                    else { String::from("char[]") }
                }
            },
            _ => {
                if is_array == true {
                    return format!("{_type}[]")
                }

                _type
            }
        }
    }
}

pub fn get_respective_type(type_: &Token) -> &str {
    match type_.content.as_str() {
        "String" => "char []",
        "Short" => "short",
        "Int" => "int",
        "Giant" => "long long",
        "Byte" | "uShort" => "unsigned short",
        "uInt" => "unsigned int",
        "uGiant" => "unsigned long long",
        "Float" => "float",      
        "Double" => "double",
        _ => "Undefined"
    }
}