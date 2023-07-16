#[derive(Debug)]
pub struct ScanningError {
    line: usize,
    message: String,
}

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    line: usize,
}

#[derive(Clone, Debug)]
enum TokenKind {
    // single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier(String),
    String(String),
    Number(f64),

    // keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

impl Scanner {
    pub fn from(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token>, Vec<ScanningError>> {
        let mut line = 1;
        let mut chars = self.source.chars().peekable();
        let mut errors = Vec::new();

        while let Some(c) = chars.next() {
            let kind = match c {
                '(' => TokenKind::LeftParen,
                ')' => TokenKind::RightParen,
                '{' => TokenKind::LeftBrace,
                '}' => TokenKind::RightBrace,
                ',' => TokenKind::Comma,
                '.' => TokenKind::Dot,
                '-' => TokenKind::Minus,
                '+' => TokenKind::Plus,
                ';' => TokenKind::Semicolon,
                '*' => TokenKind::Star,
                '!' => match chars.peek() {
                    Some('=') => {
                        chars.next();
                        TokenKind::BangEqual
                    }
                    _ => TokenKind::Bang,
                },
                '=' => match chars.peek() {
                    Some('=') => {
                        chars.next();
                        TokenKind::EqualEqual
                    }
                    _ => TokenKind::Equal,
                },
                '<' => match chars.peek() {
                    Some('=') => {
                        chars.next();
                        TokenKind::LessEqual
                    }
                    _ => TokenKind::Less,
                },
                '>' => match chars.peek() {
                    Some('=') => {
                        chars.next();
                        TokenKind::GreaterEqual
                    }
                    _ => TokenKind::Greater,
                },
                '/' => match chars.peek() {
                    Some('/') => {
                        while let Some(c) = chars.next() {
                            if c == '\n' {
                                line += 1;
                            }
                        }
                        continue;
                    }
                    Some('*') => {
                        let mut nesting = 1;
                        while let Some(curr) = chars.next() {
                            match (curr, chars.peek()) {
                                ('\n', _) | (_, Some('\n')) => line += 1,
                                ('*', Some('/')) => {
                                    nesting -= 1;
                                    chars.next();

                                    if nesting == 0 {
                                        break;
                                    }
                                }
                                ('/', Some('*')) => {
                                    nesting += 1;
                                    chars.next();
                                }
                                _ => continue,
                            }
                        }
                        continue;
                    }
                    _ => TokenKind::Slash,
                },
                '"' => {
                    let literal = chars
                        .by_ref()
                        .take_while(|&c| c != '"')
                        .inspect(|&c| {
                            if c == '\n' {
                                line += 1;
                            }
                        })
                        .collect();

                    if chars.peek().is_none() {
                        errors.push(ScanningError {
                            line,
                            message: "Unterminated string".to_string(),
                        });
                        continue;
                    }

                    TokenKind::String(literal)
                }
                c if c.is_ascii_digit() => {
                    let mut literal = c.to_string();

                    while let Some(c) = chars.next() {
                        if c.is_ascii_digit() {
                            literal.push(c);
                        } else if c == '.' {
                            if let Some(c) = chars.peek().copied() {
                                if c.is_ascii_digit() {
                                    literal.push('.');
                                } else {
                                    break;
                                }
                            }
                        } else {
                            break;
                        }
                    }

                    #[allow(clippy::unwrap_used)]
                    TokenKind::Number(literal.parse().unwrap())
                }
                c if c.is_ascii_alphabetic() || c == '_' => {
                    let lexeme = c.to_string()
                        + &chars
                            .by_ref()
                            .take_while(|&c| c != ' ' && c.is_ascii_alphanumeric())
                            .collect::<String>();

                    match lexeme.as_str() {
                        "and" => TokenKind::And,
                        "class" => TokenKind::Class,
                        "else" => TokenKind::Else,
                        "false" => TokenKind::False,
                        "for" => TokenKind::For,
                        "fun" => TokenKind::Fun,
                        "if" => TokenKind::If,
                        "nil" => TokenKind::Nil,
                        "or" => TokenKind::Or,
                        "print" => TokenKind::Print,
                        "return" => TokenKind::Return,
                        "super" => TokenKind::Super,
                        "this" => TokenKind::This,
                        "true" => TokenKind::True,
                        "var" => TokenKind::Var,
                        "while" => TokenKind::While,
                        _ => TokenKind::Identifier(lexeme),
                    }
                }
                ' ' | '\r' | '\t' => {
                    continue;
                }
                '\n' => {
                    line += 1;
                    continue;
                }
                _ => {
                    errors.push(ScanningError {
                        line,
                        message: "Unexpected character".to_string(),
                    });
                    continue;
                }
            };

            self.tokens.push(Token { kind, line });
        }

        self.tokens.push(Token {
            kind: TokenKind::Eof,
            line,
        });

        Ok(self.tokens)
    }
}
