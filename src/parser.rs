use crate::lexer::{self, StringToken, Token};

/*
source: https://ryanflannery.net/teaching/common/recursive-descent-parsing/parser.c.html
Our BNF:
 *        Formula ::= Constant | Proposition | UnaryFormula | BinaryFormula
 *       Constant ::= "T" | "F"
 *    Proposition ::= [a-z0-9]+
 *   UnaryFormula ::= LeftParen UnaryOperator Formula RightParen
 *  BinaryFormula ::= LeftParen BinaryOperator Formula Forumla RightParen
 *      LeftParen ::= "("
 *     RightParen ::= ")"
 *  UnaryOperator ::= "\neg"
 * BinaryOperator ::= "\vee" | "\wedge" | "\rightarrow" | "\leftrightarrow"
 */

trait ParserInterface {
    fn formula(tokens: &mut Vec<Token>) -> bool;
    fn constant(tokens: &mut Vec<Token>) -> bool;
    fn proposition(tokens: &mut Vec<Token>) -> bool;
    fn unary_formula(tokens: &mut Vec<Token>) -> bool;
    fn binary_formula(tokens: &mut Vec<Token>) -> bool;
    fn left_paren(tokens: &mut Vec<Token>) -> bool;
    fn right_paren(tokens: &mut Vec<Token>) -> bool;
    fn unary_operator(tokens: &mut Vec<Token>) -> bool;
    fn binary_operator(tokens: &mut Vec<Token>) -> bool;
}

pub struct Parser {}
impl Parser {
    pub fn match_token(tokens: &mut Vec<Token>, token_type: StringToken) -> bool {
        println!("m {:?}, {:?}", tokens[0], token_type);
        if tokens[0].token == token_type {
            tokens.remove(0);
            return true;
        }
        false
    }

    pub fn formula_wrapper(token: &mut Vec<Token>) -> bool {
        Self::formula(token);
        println!("fm w {:?}", token[0]);
        if token[0].token == StringToken::End {
            return true;
        }
        false
    }

    pub fn parse(input: &String) {
        let mut lex = lexer::run_lexer(input);
        let ret = Self::formula_wrapper(&mut lex);
        println!("{}", ret);
    }
}

impl ParserInterface for Parser {
    fn formula(tokens: &mut Vec<Token>) -> bool {
        println!("formula {:?}", tokens[0]);
        if Self::constant(tokens)
            || Self::proposition(tokens)
            || Self::unary_formula(tokens)
            || Self::binary_formula(tokens)
        {
            return true;
        }
        false
    }

    fn unary_formula(tokens: &mut Vec<Token>) -> bool {
        let token = tokens[0].clone();
        println!("uformula {:?}", tokens[0]);
        if !Self::left_paren(tokens) {
            tokens.insert(0, token);
            return false;
        }
        println!("here {:?}", tokens[0]);
        if !Self::unary_operator(tokens) {
            tokens.insert(0, token);
            return false;
        }

        if !Self::formula(tokens) {
            tokens.insert(0, token);
            return false;
        }
        if !Self::right_paren(tokens) {
            tokens.insert(0, token);
            return false;
        }
        true
    }
    fn binary_formula(tokens: &mut Vec<Token>) -> bool {
        let token = tokens[0].clone();
        println!("bformula {:?}", tokens[0]);
        if !(Self::left_paren(tokens)) {
            tokens.insert(0, token);
            return false;
        }
        if !Self::binary_operator(tokens) {
            tokens.insert(0, token);
            return false;
        }
        if !Self::formula(tokens) {
            tokens.insert(0, token);
            return false;
        }

        if !Self::formula(tokens) {
            tokens.insert(0, token);
            return false;
        }
        if !Self::right_paren(tokens) {
            tokens.insert(0, token);
            return false;
        }
        true
    }

    fn left_paren(tokens: &mut Vec<Token>) -> bool {
        println!("lparen {:?}", tokens[0]);
        if Self::match_token(tokens, StringToken::LeftParen) {
            return true;
        }
        false
    }

    fn right_paren(tokens: &mut Vec<Token>) -> bool {
        println!("rparen {:?}", tokens[0]);
        if Self::match_token(tokens, StringToken::RightParen) {
            return true;
        }
        false
    }

    fn constant(tokens: &mut Vec<Token>) -> bool {
        println!("constant {:?}", tokens[0]);
        if Self::match_token(tokens, StringToken::Constant) {
            return true;
        }
        false
    }

    fn proposition(tokens: &mut Vec<Token>) -> bool {
        println!("proposition {:?}", tokens[0]);
        if Self::match_token(tokens, StringToken::Text) {
            return true;
        }
        false
    }

    fn unary_operator(tokens: &mut Vec<Token>) -> bool {
        println!("uoperator {:?}", tokens[0]);
        if Self::match_token(tokens, StringToken::OperatorNot) {
            return true;
        }
        false
    }
    fn binary_operator(tokens: &mut Vec<Token>) -> bool {
        println!("boperator {:?}", tokens[0]);
        if Self::match_token(tokens, StringToken::OperatorOr)
            || Self::match_token(tokens, StringToken::OperatorAnd)
        {
            return true;
        }
        false
    }
}
