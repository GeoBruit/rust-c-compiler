use crate::lexer::Token;

#[derive(Debug)]
pub struct Program{
    pub function: Function,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub body: Statement,

}

#[derive(Debug)]
pub enum Statement {
    Return(Expression)
}

#[derive(Debug)]
pub enum Expression {
    Constant(i32),
}


pub fn parse(tokens: &[Token]) {

    let mut  position = 0;
    println!("IN PARSER");
    println!("tokens: {:?}", tokens[0]);

    match &tokens[position] {
        Token::IntKeyword => {
            println!("IntKeyword found!!!");
            position += 1;
        }
        _ =>{
            panic!("Expected int keyword!")
        }
    }

    match &tokens[position] {
        Token::Identifier(name) => {
            println!("Identified is: {:?}", name);
            position += 1;
        }
        _ => {
            panic!("Expected identifier!")
        }
    }

    match &tokens[position] {
        Token::OpenParenthesis => {

            println!(" found: '(' ");
            position+=1
        }
        _ => {
            panic!("Expected open parenthesis!")
        }
    }

    match &tokens[position] {
        Token::VoidKeyword => {
            println!("VoidKeyword");
            position += 1;
        }

        _ => {
            panic!("Expected void keyword!")
        }
    }

    match &tokens[position] {
        Token::CloseParenthesis => {
            println!(" found: ')' ");

            position += 1;
        }

        _ => {
            panic!("Expected close parenthesis!")
        }


    }

    match &tokens[position] {
        Token::OpenBrace => {

            println!("Open Brace");
            position += 1;
        }

        _ => {
            panic!("Expected open brace!")
        }
    }

    match &tokens[position] {
        Token::ReturnKeyword => {
            println!(" found: 'return' ");
            position += 1;
        }

        _ => {
            panic!("Expected return keyword!")
        }
    }

    match &tokens[position] {
        Token::Constant(constant) => {

            println!("Constant found: {:?}", constant);
            position += 1;
        }
        _ => {
            panic!("Expected constant keyword!")
        }
    }

    match &tokens[position] {

        Token::Semicolon => {
            println!(" found: ';' ");
            position += 1;
        }

        _ => {
            panic!("Expected ';' separator!")
        }
    }

    match &tokens[position] {
        Token::CloseBrace => {
            println!(" found: '}}' ");
            position += 1;
        }

        _ => {
            panic!("Expected close brace!")
        }
    }

    if position != tokens.len() {
        panic!("Unexpected tokens after end of program");
    }

}