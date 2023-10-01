use crate::steps::lang::lexical::Token;

pub fn _table(token: &Token, next_token: &Token) -> (String, (String, bool)) {
    let brackets = [
        String::from("("), String::from(")"),
        String::from("["), String::from("]"),
        String::from("{"), String::from("}")
    ];

    let operator = [
        String::from("equals"),
        String::from("and"),
        String::from("not"),
        String::from("or"),
        String::from("<"),
        String::from(">"),
        String::from("+"),
        String::from("-"),
        String::from("/"),
        String::from("%"),
        String::from("#"),
        String::from("*"),
        String::from("$"),
        String::from("^")
    ];

    let symbols = [
        String::from("&"),
        String::from("@"),
        String::from("!"),
        String::from("."),
        String::from(":"),
        String::from(";"),
        String::from("=")
    ];

    let keyword = [
        String::from("const"),
        String::from("let"),
        String::from("var"),
        String::from("void")
    ];

    let value: &String = &token.content;
    let mut has_verifyed_next_token = false;

    let x: String = (|value: &String| -> String{
        if keyword.contains(&value) {
            let result: String = (|| -> String {
                match value.as_str() {
                    "local" => return String::from("Need verify if static"),
                    "var" | "let" => return String::from("Mutable"),
                    "void" => return String::from("Function"),
                    "const" => return String::from("Static"),
                    _ => return String::from("Method")
                }
            })();

            if result == "Need verify if static" {
                has_verifyed_next_token = true;

                if next_token.content == "static" { return String::from("Static") }
                else { return String::from("Mutable") }
            } else { return result }
        }

        String::new()
    })(value);

    println!("{}", value);
    if brackets.contains(&value) { (String::from("Bracket"), (String::from("Method"), has_verifyed_next_token)) }
    else if operator.contains(&value) { (String::from("Operator"), (String::from("Method"), has_verifyed_next_token)) }
    else if symbols.contains(&value) { (String::from("Symbol"), (String::from("Method"), has_verifyed_next_token)) }
    else if keyword.contains(&value) { (String::from("Keyword"), (x, has_verifyed_next_token)) }
    else { (String::from("Variable"), (String::from("ID"), has_verifyed_next_token)) }
}