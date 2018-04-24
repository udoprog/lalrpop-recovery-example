extern crate lalrpop_util;

mod lexer;
mod parser;
mod ast;

#[test]
fn test() {
    use lexer::Token;

    let tokens: Vec<Result<(usize, Token, usize), lexer::Error>> = vec![
        Ok((0, Token::A("a"), 1)),
        Ok((0, Token::B("b"), 1)),
    ];

    let parser = parser::RuleParser::new();
    let out = parser.parse(tokens.into_iter()).expect("parse error");

    // NOTE: actually expected: [Token::A("a"), Token::B("b")]
    let expected = ast::Rule::Error(vec![(0, Token::B("b"), 1)]);

    assert_eq!(expected, out);
}
