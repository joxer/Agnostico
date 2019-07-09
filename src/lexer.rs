use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
enum Token {
    #[end]
    End,

    #[error]
    Error,

    #[regex = "(\\+|\\-|/|\\*)"]
    Operator,

    #[regex = "[a-zA-Z]+"]
    Text,

    #[regex = "[0-9]+(\\.[0-9]+)?"]
    Number,
}

pub fn run_lexer(str: &String) {
    let mut lexer = Token::lexer(str.as_str());
    let mut current = lexer.token.clone();

    while current != Token::End {
        println!("{}, {:?}", lexer.slice(), &current);
        lexer.advance();
        current = lexer.token.clone();
    }
}
