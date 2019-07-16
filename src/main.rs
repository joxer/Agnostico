use agnostico::parser;

fn main() {
    parser::Parser::parse(&String::from("(+ (+ 2 BC) 3)"));
}
