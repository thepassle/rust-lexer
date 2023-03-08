use crate::token::{Token, TokenType};

pub struct Lexer<'a> {
    // todo instead of owned strings I can probably use borrowed slices based on the input
    // since the lifetime of the input string is the same as the lifetime of the lexer
    input: &'a str,
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let chars = input.chars().collect();
        Lexer {
            input,
            chars,
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.chars.len()
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.chars.get(self.pos).cloned();
        if c.is_some() {
            self.pos += 1;
            self.col += 1;
        }
        c
    }

    fn peek_char(&self) -> Option<char> {
        self.chars.get(self.pos).cloned()
    }

    fn peek_next_char(&self) -> Option<char> {
        self.chars.get(self.pos + 1).cloned()
    }

    fn add_token(&self, kind: TokenType, length: usize) -> Token<'a> {
        let start = self.pos - length;
        let value = &self.input[start..self.pos];

        let token = Token {
            kind,
            value,
            start,
            end: self.pos,
            line: self.line,
        };
        println!("{:?}", token);
        token
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(c) = self.peek_char() {
            match c {
                ' ' | '\t' | '\r' => {
                    self.next_char();
                }
                '\n' => {
                    self.next_char();
                    self.line += 1;
                    self.col = 1;
                }
                '(' => {
                    self.next_char();
                    println!("{}", self.pos);
                    tokens.push(self.add_token(TokenType::LeftParen, 1));
                }
                ')' => {
                    self.next_char();
                    tokens.push(self.add_token(TokenType::RightParen, 1));
                }
                '{' => {
                    self.next_char();
                    tokens.push(self.add_token(TokenType::LeftBrace, 1));
                }
                '}' => {
                    self.next_char();
                    tokens.push(self.add_token(TokenType::RightBrace, 1));
                }
                '[' => {
                    self.next_char();
                    tokens.push(self.add_token(TokenType::LeftBracket, 1));
                }
                ']' => {
                    self.next_char();
                    tokens.push(self.add_token(TokenType::RightBracket, 1));
                }
                '*' => {
                    self.next_char();
                    tokens.push(self.add_token(TokenType::Asterisk, 1));
                }
                ',' => {
                    self.next_char();
                    tokens.push(self.add_token(TokenType::Comma, 1));
                }
                '.' => {
                    self.next_char();
                    tokens.push(self.add_token(TokenType::Dot, 1));
                }
                ';' => {
                    self.next_char();
                    tokens.push(self.add_token(TokenType::Semicolon, 1));
                }
                '+' => {
                    self.next_char();
                    tokens.push(self.add_token(TokenType::Plus, 1));
                }
                '-' => {
                    self.next_char();
                    tokens.push(self.add_token(TokenType::Minus, 1));
                }
                '/' => {
                    self.next_char();
                    if self.peek_char() == Some('/') {
                        // Single-line comment
                        while self.peek_char() != Some('\n') && !self.is_eof() {
                            self.next_char();
                        }
                    } else if self.peek_char() == Some('*') {
                        // Multi-line comment
                        self.next_char(); // consume the '*'
                        let mut is_comment_closed = false;
                        while let Some(c) = self.next_char() {
                            if c == '*' && self.peek_char() == Some('/') {
                                is_comment_closed = true;
                                self.next_char(); // consume the '/'
                                break;
                            }
                        }
                        if !is_comment_closed {
                            panic!(
                                "Unclosed multi-line comment at line {} column {}",
                                self.line, self.col
                            );
                        }
                    } else {
                        tokens.push(self.add_token(TokenType::Slash, 1));
                    }
                }
                '%' => {
                    self.next_char();
                    tokens.push(self.add_token(TokenType::Modulo, 1));
                }
                ':' => {
                    self.next_char();
                    tokens.push(self.add_token(TokenType::Colon, 1));
                }
                '<' => {
                    self.next_char();
                    if self.peek_char() == Some('=') {
                        self.next_char();
                        tokens.push(self.add_token(TokenType::LessEqual, 2));
                    } else {
                        tokens.push(self.add_token(TokenType::Less, 1));
                    }
                }
                '>' => {
                    self.next_char();
                    if self.peek_char() == Some('=') {
                        self.next_char();
                        tokens.push(self.add_token(TokenType::GreaterEqual, 2));
                    } else {
                        tokens.push(self.add_token(TokenType::Greater, 1));
                    }
                }
                '=' => {
                    self.next_char();
                    if self.peek_char() == Some('=') {
                        self.next_char();
                        tokens.push(self.add_token(TokenType::EqualEqual, 2));
                    } else {
                        tokens.push(self.add_token(TokenType::Equal, 1));
                    }
                }
                '!' => {
                    self.next_char();
                    if self.peek_char() == Some('=') {
                        self.next_char();
                        tokens.push(self.add_token(TokenType::BangEqual, 2));
                    } else {
                        tokens.push(self.add_token(TokenType::Bang, 1));
                    }
                }
                '"' | '\'' => {
                    let quote_char = self.next_char().unwrap();
                    let mut value = String::new();
                    while self.peek_char() != Some(quote_char) && !self.is_eof() {
                        if self.peek_char() == Some('\\') {
                            self.next_char(); // consume the backslash
                            match self.peek_char() {
                                Some(ch) if ch == quote_char => {
                                    value.push(quote_char);
                                    self.next_char();
                                }
                                Some('\\') => {
                                    value.push('\\');
                                    self.next_char();
                                }
                                Some('n') => {
                                    value.push('\n');
                                    self.next_char();
                                }
                                Some('t') => {
                                    value.push('\t');
                                    self.next_char();
                                }
                                _ => {
                                    value.push('\\');
                                }
                            }
                        } else {
                            value.push(self.next_char().unwrap());
                        }
                    }
                    if self.next_char() != Some(quote_char) {
                        panic!(
                            "Unterminated string at line {} column {}",
                            self.line, self.col
                        );
                    }
                    // TODO: the len passed to add_token doesnt account for the quotes
                    tokens.push(self.add_token(TokenType::StringLiteral, value.len()));
                }
                _ if c.is_digit(10) => {
                    let mut value = String::new();
                    while let Some(digit) = self.peek_char() {
                        if digit.is_digit(10) || digit == '_' {
                            if let Some(next_char) = self.peek_next_char() {
                                if digit == '_' && !next_char.is_digit(10) {
                                    panic!("Numeric separators are not allowed at the end of numeric literals at line {} column {}", self.line, self.col);
                                }
                            }

                            value.push(digit);
                            self.next_char();
                        } else {
                            break;
                        }
                    }
                    tokens.push(self.add_token(TokenType::NumericLiteral, value.len()));
                }
                _ if c.is_alphabetic() => {
                    let mut value = String::new();
                    while let Some(c) = self.peek_char() {
                        if c.is_alphanumeric() || c == '_' {
                            value.push(c);
                            self.next_char();
                        } else {
                            break;
                        }
                    }
                    let token_type = match value.as_str() {
                        "abstract" => TokenType::Abstract,
                        "arguments" => TokenType::Arguments,
                        "assert" => TokenType::Assert,
                        "await" => TokenType::Await,
                        "boolean" => TokenType::Boolean,
                        "break" => TokenType::Break,
                        "byte" => TokenType::Byte,
                        "case" => TokenType::Case,
                        "catch" => TokenType::Catch,
                        "char" => TokenType::Char,
                        "class" => TokenType::Class,
                        "const" => TokenType::Const,
                        "continue" => TokenType::Continue,
                        "debugger" => TokenType::Debugger,
                        "default" => TokenType::Default,
                        "delete" => TokenType::Delete,
                        "do" => TokenType::Do,
                        "double" => TokenType::Double,
                        "else" => TokenType::Else,
                        "enum" => TokenType::Enum,
                        "eval" => TokenType::Eval,
                        "export" => TokenType::Export,
                        "extends" => TokenType::Extends,
                        "false" => TokenType::False,
                        "final" => TokenType::Final,
                        "finally" => TokenType::Finally,
                        "float" => TokenType::Float,
                        "for" => TokenType::For,
                        "from" => TokenType::From,
                        "function" => TokenType::Function,
                        "goto" => TokenType::Goto,
                        "if" => TokenType::If,
                        "implements" => TokenType::Implements,
                        "import" => TokenType::Import,
                        "in" => TokenType::In,
                        "instanceof" => TokenType::Instanceof,
                        "int" => TokenType::Int,
                        "interface" => TokenType::Interface,
                        "let" => TokenType::Let,
                        "long" => TokenType::Long,
                        "native" => TokenType::Native,
                        "new" => TokenType::New,
                        "null" => TokenType::Null,
                        "package" => TokenType::Package,
                        "private" => TokenType::Private,
                        "protected" => TokenType::Protected,
                        "public" => TokenType::Public,
                        "return" => TokenType::Return,
                        "short" => TokenType::Short,
                        "static" => TokenType::Static,
                        "super" => TokenType::Super,
                        "switch" => TokenType::Switch,
                        "synchronized" => TokenType::Synchronized,
                        "this" => TokenType::This,
                        "throw" => TokenType::Throw,
                        "throws" => TokenType::Throws,
                        "transient" => TokenType::Transient,
                        "true" => TokenType::True,
                        "try" => TokenType::Try,
                        "typeof" => TokenType::Typeof,
                        "undefined" => TokenType::Undefined,
                        "var" => TokenType::Var,
                        "void" => TokenType::Void,
                        "volatile" => TokenType::Volatile,
                        "while" => TokenType::While,
                        "with" => TokenType::With,
                        "yield" => TokenType::Yield,
                        _ => TokenType::Identifier,
                    };
                    tokens.push(self.add_token(token_type, value.len()));
                }
                _ => {
                    panic!(
                        "Unexpected character '{}' at line {} column {}",
                        c, self.line, self.col
                    );
                }
            }
        }

        tokens
    }
}
