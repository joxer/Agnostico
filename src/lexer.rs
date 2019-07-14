use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum StringToken {
    #[end]
    End,

    #[error]
    Error,

    #[regex = "(\\+|\\-|/|\\*)"]
    Operator,

    #[regex = "[a-zA-Z]+"]
    Text,

    #[regex = "[A-Z]+"]
    Constant,

    #[regex = "[0-9]+(\\.[0-9]+)?"]
    Number,

    #[regex = "\\("]
    LeftParen,

    #[regex = "\\)"]
    RightParen,

    #[regex = "\\|\\|"]
    OperatorOr,

    #[regex = "&&"]
    OperatorAnd,

    #[regex = "!"]
    OperatorNot,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub string: String,
    pub token: StringToken,
}

pub fn run_lexer(str: &String) -> Vec<Token> {
    let mut lexer = StringToken::lexer(str.as_str());
    let mut current = lexer.token.clone();
    let mut vec = Vec::new();
    while current != StringToken::End {
        //        println!("{}, {:?}", lexer.slice(), &current);
        vec.push(Token {
            string: lexer.slice().to_string(),
            token: current,
        });
        lexer.advance();
        current = lexer.token.clone();
    }
    vec.push(Token {
        string: "".to_string(),
        token: StringToken::End,
    });
    vec
}
