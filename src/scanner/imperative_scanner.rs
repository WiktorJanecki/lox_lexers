use crate::scanner::{IScanner, ScannerError, ScannerResult};
use crate::tokens::{Token, TokenType};

pub struct ImperativeScanner {
    text: String,
    current_line: usize,
    current_position: usize,
    current_index: usize,
    literal_buffer: String,
    token_buffer: Vec<Token>,
    errors: Vec<ScannerError>,
}

impl ImperativeScanner {
    fn is_end(self: &ImperativeScanner) -> bool {
        self.current_index >= self.text.len()
    }

    fn pop(self: &mut ImperativeScanner) -> Option<char> {
        self.current_index += 1;
        self.current_position += 1;
        self.text.chars().nth(self.current_index - 1)
    }
    fn peek(self: &ImperativeScanner) -> Option<char> {
        self.text.chars().nth(self.current_index)
    }
    fn peek_plus_n(self: &ImperativeScanner, n: usize) -> Option<char> {
        self.text.chars().nth(self.current_index + n )
    }

    // returns if constructed
    fn handle_trivial(self: &mut ImperativeScanner, c: char) -> bool {
        let mut append = |ttype: TokenType| {
            let token = Token {
                token_type: ttype,
                line: self.current_line,
                position: self.current_position,
            };
            self.token_buffer.push(token);
            return true;
        };
        let constructed = match c {
            '*' => append(TokenType::Star),
            '+' => append(TokenType::Plus),
            '-' => append(TokenType::Minus),
            ';' => append(TokenType::Semicolon),
            ',' => append(TokenType::Comma),
            '.' => append(TokenType::Dot),
            '(' => append(TokenType::OpenParentheses),
            ')' => append(TokenType::CloseParentheses),
            '{' => append(TokenType::OpenBrace),
            '}' => append(TokenType::CloseBrace),
            _ => false,
        };
        constructed
    }
    fn handle_new_line(self: &mut ImperativeScanner, c: char) -> bool {
        if c != '\n' {
            return false;
        }
        let token = Token{token_type: TokenType::NewLine, line: self.current_line, position: self.current_position};
        self.token_buffer.push(token);
        self.current_line += 1;
        self.current_position = 0;
        true
    }

    fn handle_identifier(self: &mut ImperativeScanner, c: char) -> bool {
        if !matches!(c, 'a'..='z'| 'A'..='Z') {
            return false;
        }
        self.literal_buffer.clear();
        let matches = |c: char| matches!(c,'a'..='z'| 'A'..='Z'| '0'..='9' | '_' | '-');

        let mut curr = c;
        while (matches(curr)) {
            self.literal_buffer.push(curr);
            if let Some(c) = self.pop() {
                curr = c;
            } else {
                break;
            }
        }
        self.current_position -= 1;
        self.current_index -= 1;

        let mut append = |ttype: TokenType| {
            let token = Token {
                token_type: ttype,
                line: self.current_line,
                position: self.current_position,
            };
            self.token_buffer.push(token);
            return true;
        };

        match self.literal_buffer.as_str() {
            "class" => append(TokenType::Class),
            "else" => append(TokenType::Else),
            "false" => append(TokenType::False),
            "for" => append(TokenType::For),
            "fun" => append(TokenType::Fun),
            "if" => append(TokenType::If),
            "nil" => append(TokenType::Nil),
            "or" => append(TokenType::Or),
            "and" => append(TokenType::And),
            "print" => append(TokenType::Print),
            "return" => append(TokenType::Return),
            "super" => append(TokenType::Super),
            "this" => append(TokenType::This),
            "true" => append(TokenType::True),
            "var" => append(TokenType::Var),
            "while" => append(TokenType::While),
            _ => append(TokenType::Identifier(self.literal_buffer.clone())),
        };

        true
    }

    fn handle_number(self: &mut ImperativeScanner, c: char) -> bool {
        if !matches!(c, '0'..='9') {
            return false;
        }
        self.literal_buffer.clear();
        let matches_number = |c: char| matches!(c, '0'..='9');

        let mut curr = c;
        let mut dotted = false;
        while (matches_number(curr)) {
            self.literal_buffer.push(curr);
            if let Some(c) = self.pop() {
                curr = c;
                if !dotted && curr == '.' {
                    dotted = true;
                    self.literal_buffer.push('.');
                    if let Some(c) = self.pop() {
                        curr = c;
                    } else {
                        self.errors.push(ScannerError { invalid_text: self.literal_buffer.clone(), line: self.current_line, column: self.current_position });
                        return true;
                    }
                }
            } else {
                break;
            }
        }
        self.current_position -= 1;
        self.current_index -= 1;

        let result = self.literal_buffer.parse::<f64>();

        match result {
            Ok(number) => {
                let token = Token {
                    token_type: TokenType::NumberLiteral(number),
                    line: self.current_line,
                    position: self.current_position,
                };
                self.token_buffer.push(token);
            }
            Err(_) => {
                let err = ScannerError {
                    invalid_text: self.literal_buffer.clone(),
                    line: self.current_line,
                    column: self.current_position,
                };
                self.errors.push(err);
            }
        }
        true
    }
    fn handle_string(self: &mut ImperativeScanner, c: char) -> bool {
        if c != '\"' {
            return false;
        }
        self.literal_buffer.clear();

        let popped = self.pop();
        if popped.is_none(){
            self.errors.push(ScannerError { invalid_text: c.to_string(), line: self.current_line, column: self.current_position });
            return true;
        }
        let mut curr = popped.unwrap();

        while curr != '"' {
            self.literal_buffer.push(curr);
            if let Some(c) = self.pop(){
                curr = c;
            }
            else{
                self.errors.push(ScannerError { invalid_text: self.literal_buffer.clone(), line: self.current_line, column: self.current_position });
                return true;
            }
        }
        let token = Token {
            token_type: TokenType::StringLiteral(self.literal_buffer.clone()),
            line: self.current_line,
            position: self.current_position,
        };
        self.token_buffer.push(token);
        true
    }
    fn handle_comments(self: &mut ImperativeScanner, c: char) -> bool {
        if !matches!(c, '/') {
            return false;
        }
        if !matches!(self.peek().unwrap_or('!'), '/'){
            self.token_buffer.push(Token{token_type:TokenType::Slash,line: self.current_line, position: self.current_position} );
            return true;
        }

       self.literal_buffer.clear();

        loop{
            if let Some(c) = self.pop() {
                if c == '\n' {
                    let token = Token{token_type: TokenType::Comment(self.literal_buffer.clone()), line: self.current_line, position: self.current_position };
                    self.token_buffer.push(token);
                    self.current_line += 1;
                    self.current_position = 0;
                    break;
                }
                self.literal_buffer.push(c);
                continue;
            }
            break;
        }
        let token = Token{token_type: TokenType::Comment(self.literal_buffer.clone()), line: self.current_line, position: self.current_position };
        self.token_buffer.push(token);
        true
    }
    fn handle_whitespace(self: &mut ImperativeScanner, c: char) -> bool {
        if !matches!(c, ' ' | '\t') {
            return false;
        }
        let token = Token{token_type: TokenType::Space, line: self.current_line, position: self.current_position };
        self.token_buffer.push(token);
        true

    }
    fn handle_doublers(self: &mut ImperativeScanner, c: char) -> bool {
        if !matches!(c, '>' | '<' | '=' | '!') {
            return false;
        }

        let mut lambda = |first: char, second: char, first_tok: TokenType, second_tok: TokenType| {
            if c != first{
                return;
            }
            if self.peek().unwrap_or('a')  != second {
                let token = Token{token_type: first_tok, line: self.current_line, position: self.current_position};
                self.token_buffer.push(token);
                return
            }
            let _ = self.pop();
            let token = Token{token_type: second_tok, line: self.current_line, position: self.current_position};
            self.token_buffer.push(token);
            return
        };

        lambda('>', '=', TokenType::CloseAngleBracket, TokenType::GreaterEqual);
        lambda('<', '=', TokenType::OpenAngleBracket, TokenType::LessEqual);
        lambda('!', '=', TokenType::Bang, TokenType::BangEqual);
        lambda('=','=', TokenType::Equal, TokenType::DoubleEqual);
        true
    }
}

impl IScanner for ImperativeScanner {
    fn new(text: String) -> Self {
        Self {
            text,
            current_line: 0,
            current_position: 0,
            current_index: 0,
            literal_buffer: String::new(),
            token_buffer: Vec::new(),
            errors: Vec::new(),
        }
    }

    fn scan(&mut self) -> ScannerResult<Vec<Token>> {
        while !self.is_end() {
            let char = self.pop().unwrap();
            if self.handle_whitespace(char) {
                continue;
            }
            if self.handle_new_line(char) {
                continue;
            }
            if self.handle_trivial(char) {
                continue;
            }
            if self.handle_identifier(char) {
                continue;
            }
            if self.handle_number(char) {
                continue;
            }
            if self.handle_string(char) {
                continue;
            }
            if self.handle_comments(char){
                continue;
            }
            if self.handle_doublers(char){
                continue;
            }
        }
        self.token_buffer.push(Token{token_type: TokenType::EOF,line: self.current_line, position: self.current_position });

        if (self.errors.len() > 0) {
            return Err(self.errors.clone());
        }
        Ok(self.token_buffer.clone())
    }
}
