/*
    Este código foi desenvolvido por Lucas F. Sil <lucasdwbfff@gmail.com>
    e com colaboração de Tiago Dinis <tiagodinis33@proton.me>

    22/08/2023 - Data de criação do comentário.
*/

#[derive(Debug, PartialEq, Eq)]
pub struct LexerError {
    pub message: String,
    pub line: usize,
    pub col: usize,
}
#[derive(Debug, Default)]
pub struct Token {
    pub content: String,
    pub line: usize,
    pub col: usize,
}
pub fn analizer(code: String) -> Result<Vec<Token>, LexerError> {
    #[derive(Debug, Clone, Copy, PartialEq)]
    enum CommentState {
        SingleLine {
            line: usize,
            col: usize
        },
        MultiLine {
            line: usize,
            col: usize
        },
        None
    }
    let mut current_token = Token::default();
    let mut tokens: Vec<Token> = vec![];
    let mut line = 1;
    let mut col = 0;
    let mut inside_string = false;
    let mut comment_state: CommentState = CommentState::None;
    for letter in code.chars() {
        col += 1;
        if letter == '\n' {
            line += 1;
            col = 0;
            match comment_state {
                CommentState::SingleLine {..} => comment_state = CommentState::None,
                CommentState::MultiLine {..}  => continue,
                CommentState::None => {}
            }
        }
        
        match comment_state {
            CommentState::SingleLine {..} => continue,
            CommentState::MultiLine {..} if letter == '/' && current_token.content == "*" => {
                current_token = Token::default();
                comment_state = CommentState::None;
                continue;
            }
            CommentState::MultiLine {..} => {
                if letter == '*' {
                    current_token = Token {content: String::new(), col, line};
                }
                continue;
            },
            CommentState::None => {}
        }
        match letter {
            // If we found a '"' and were out of a string, it means that we need to enter the string state
            '"' if !inside_string => {
                // We don't need to push the current_token if it's empty.
                if !current_token.content.is_empty(){
                    tokens.push(current_token);
                    current_token = Token {content: String::new(), col, line};
                } else {
                    current_token.col = col;
                    current_token.line = line;
                }
                inside_string = true;
                current_token.content.push(letter);
            }
            // If we found a '"' and we were inside a string,
            // means we need to get out of the string state
            '"' if inside_string => {
                inside_string = false;
                current_token.content.push(letter);
                tokens.push(current_token);
                current_token = Token::default();
            }
            // If we find a whitespace and are not inside a string
            // We need to push the current_token for tokens and clean up the current_token
            letter if letter.is_whitespace() && !inside_string => {
                // If current_token're empty, we just don't do anything
                if !current_token.content.is_empty(){
                    tokens.push(current_token);
                    current_token = Token::default();
                }
            }
            // But if we are inside a string, any character that is found has to add to the current_token
            letter if inside_string => {
                current_token.content.push(letter);
            }
            // Treat f@ as a separate token
            '@' if current_token.content == "f" => {
                current_token.content.push(letter);
            }
            // If current_token is "/" and the current character is a '/'
            // It means that we found a "//", so it means that it is a 1-line comment
            '/' if current_token.content == "/" => {
                current_token = Token::default();
                comment_state = CommentState::SingleLine {line, col};
            }

            // If current_token is "/" and the current character is a '*'
            // It means we found a "/*", so it means it's a multi-line comment
            '*' if current_token.content == "/" => {
                current_token = Token::default();
                comment_state = CommentState::MultiLine {line, col};
            }
            // If we find a /, and the current_token is empty, it could be a sign of a comment, 
            // so we haven't added it directly to the tokens yet
            '/' if current_token.content.is_empty() => {
                current_token.col = col;
                current_token.line = line;
                current_token.content.push('/');
            }

            // If we find a special character, we push what was in the current_token
            // We clean up what's in the current_token and add the separate character
            ',' | '.' | '(' | ')' | ';' | ':' | '{' | '}' | '=' | '&' | '-' | '+' | '/' | '*' | '<' | '>' | '[' | ']' | '@' => {
                // If current_token're empty, we just don't do anything
                if !current_token.content.is_empty(){
                    tokens.push(current_token);
                }
                current_token = Token::default();
                tokens.push(Token { content: letter.to_string(), line, col });
            }
            // If it's something else add to the current_token
            _ => {
                if current_token.content.is_empty() {
                    current_token.col = col;
                    current_token.line = line;
                }
                current_token.content.push(letter);
            }
        }
    }
    
    // Verify that all tokens have location (in debug mode)
    debug_assert!(tokens.iter().all(|t| t.col > 0 && t.line > 0));
    if let CommentState::MultiLine {line, col} = comment_state {
        Err(LexerError {message: "open multiline comment".to_string(), line, col})
    } else {
        tokens.push(Token { col: 5, line: 1, content: String::from("__ignore@__0x10010") });
        Ok(tokens)
    }
}