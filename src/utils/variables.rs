use crate::steps::lang::lexical::Token;

pub fn is_valid_var(var: &Token) -> bool {
    let mut has_special_chars = false;
    for c in var.content.chars() {
        if !c.is_alphanumeric() && c != '_' {
            has_special_chars = true;
            break;
        }
    }

    if has_special_chars {
        false
    } else {
        true
    }
}