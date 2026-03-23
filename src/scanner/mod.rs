use crate::tokens::Token;

pub mod imperative_scanner;


#[derive(Debug, Clone)]
pub struct ScannerError{
    pub invalid_text: String,
    pub line: usize,
    pub column: usize,
}
type ScannerResult<T> = Result<T, Vec<ScannerError>>;

pub trait IScanner{
    fn new(string: String) -> Self;
    fn scan(&mut self) -> ScannerResult<Vec<Token>>;
}
