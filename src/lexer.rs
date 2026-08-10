use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Constant(i32),
    IntKeyword,
    VoidKeyword,
    ReturnKeyword,
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    Semicolon,
}

pub fn lex(mut input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let identifier_re = Regex::new(r"^[a-zA-Z_]\w*\b").unwrap();
    let constant_re = Regex::new(r"^[0-9]+\b").unwrap();

    while !input.is_empty() {
        // Remove whitespace from the beginning
        if input.starts_with(char::is_whitespace) {
            input = input.trim_start();
            continue;
        }

        //for now I will skip the comments
        if input.starts_with("//") {
            if let Some(newline_pos) = input.find('\n') {
                input = &input[newline_pos + 1..];
            } else {
                input = "";
            }

            continue;
        }

        //added this because 1  of the test cases has a multiline comment
        if input.starts_with("/*") {
            if let Some(end_pos) = input.find("*/") {
                input = &input[end_pos + 2..];
            } else {
                panic!("Unclosed block comment");
            }

            continue;
        }

        // try to match an identifier/keyword
        if let Some(matched) = identifier_re.find(input) {
            let text = matched.as_str();

            let token = match text {
                "int" => Token::IntKeyword,
                "void" => Token::VoidKeyword,
                "return" => Token::ReturnKeyword,
                _ => Token::Identifier(text.to_string()),
            };

            tokens.push(token);
            input = &input[matched.end()..];
            continue;
        }

        // Try to match an integer constant
        if let Some(matched) = constant_re.find(input) {
            let value = matched.as_str().parse::<i32>().unwrap();

            tokens.push(Token::Constant(value));
            input = &input[matched.end()..];
            continue;
        }

        // Single-character tokens
        let token = match input.chars().next().unwrap() {
            '(' => Token::OpenParenthesis,
            ')' => Token::CloseParenthesis,
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            ';' => Token::Semicolon,
            c => panic!("Unexpected character: {}", c),
        };

        tokens.push(token);

        // These tokens are all one ASCII character,
        // so consume one byte from the input.
        input = &input[1..];
    }

    tokens
}