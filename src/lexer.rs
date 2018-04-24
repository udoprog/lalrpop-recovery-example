#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'input> {
    A(&'input str),
    B(&'input str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
}
