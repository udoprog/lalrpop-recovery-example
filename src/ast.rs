use lexer::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule<'input> {
    A(&'input str),
    B(&'input str),
    Error(Vec<(usize, Token<'input>, usize)>),
}
