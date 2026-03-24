use crate::scanner::{IScanner, imperative_scanner};
use crate::scanner::imperative_scanner::ImperativeScanner;
use crate::tokens::Token;

mod scanner;
mod tokens;
mod colorizer;

fn main() {
    let args = std::env::args();
    let path = args.skip(1).next().unwrap_or_else(||{
        eprintln!("USAGE: ./rlox FILE");
        std::process::exit(-1);
    });

    let content = std::fs::read_to_string(path.as_str()).unwrap_or_else(|e| {
        eprintln!("Failed to read file: {}", e);
        std::process::exit(-1);
    });

    let mut scanner = ImperativeScanner::new(content);
    match scanner.scan(){
        Ok(tokens) => {
            let new_path = format!("{}.html", path);
            colorizer::colorize_to_file(tokens, new_path.as_str()).unwrap();
        }
        Err(err) => {
            for e in err{
                eprintln!("ERROR: LINE: {}, COL: {}, TEXT: {}", e.line, e.column, e.invalid_text);
            }
        }
    }

}


fn print_tokens(tokens: &[Token]) {
    let mut line = 0;
    for token in tokens {
        if token.line != line
        {
            println!();
            line+=1;
        }
        print!("{:?} ", token.token_type);
    }
}