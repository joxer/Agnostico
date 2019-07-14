use agnostico::{parser};

fn main() {
    parser::Parser::parse(&String::from("(&& A B)"));
}
