use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Symbols
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    HashSign,
    Dot,
    DotDot,

    // Dice
    Dice,
    DiceOption(String),

    // Math operators
    Minus,
    Plus,
    Asterisk,
    Slash,
    SlashSlash,
    Hat,

    // Logic and relational operators
    Not,
    And,
    Or,
    Equal,
    EqualEqual,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier(String),
    StringLiteral(String),
    Number(i32),

    // keywords
    If,
    Else,
    Function,
    For,
    While,
    End,
    Local,

    // Constants
    Nil,
    True,
    False,

    EOF,
}

fn categorize_word(word: String) -> TokenType {
    match word.as_str() {
        "function" => TokenType::Function,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "for" => TokenType::For,
        "while" => TokenType::While,
        "end" => TokenType::End,
        "local" => TokenType::Local,
        "nil" => TokenType::Nil,
        "true" => TokenType::True,
        "false" => TokenType::False,
        _ => TokenType::Identifier(word),
    }
}

fn escape_char(c: char) -> Option<char> {
    match c {
        '\\' => Some('\\'),
        'n' => Some('\n'),
        't' => Some('\t'),
        _ => None
    }
}

const WHITESPACE_CHARS: &[char] = &[' ', '\n', '\t'];

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub col: u32,
}

impl Token {
    pub fn new(token_type: TokenType, col: u32) -> Self {
        Token { token_type, col }
    }
}

pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            start: 0,
            tokens: Vec::new(),
            current: 0,
        }
    }

    pub fn peek(&self) -> Option<&char> {
        if self.current >= self.source.len() as u32 {
            None
        } else {
            Some(&self.source[self.current as usize])
        }
    }

    pub fn advance(&mut self) -> Option<&char> {
        self.current += 1;
        self.peek()
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        self.tokens.clear();
        let mut expr_depth = 0;
        let mut dice_expr_depths = HashSet::new();

        while let Some(&c) = self.peek() {
            self.start = self.current;
            match c {
                _ if WHITESPACE_CHARS.contains(&c) => (),
                'd' => {
                    if let Some(&c2) = self.advance() {
                        if !self.peek().unwrap().is_numeric() {
                            if let Some(mut name) = self.read_identifier() {
                                name.insert(0, 'd');
                                self.add_token(TokenType::Identifier(name));
                                self.advance();
                                continue;
                            }
                        }
                        self.add_token(TokenType::Dice);
                        if let Some(n) = self.read_number() {
                            self.add_token(TokenType::Number(n));
                            if let Some(s) = self.read_identifier() {
                                self.add_token(TokenType::DiceOption(s));
                            }
                        } else if c2 == '(' {
                            dice_expr_depths.insert(expr_depth + 1);
                            continue;
                        }
                    }
                }
                _ if c.is_alphabetic() || c == '_' => {
                    if let Some(name) = self.read_identifier() {
                        self.add_token(categorize_word(name));
                        continue;
                    }
                }
                '0'..='9' => {
                    let num = self.read_number();
                    if let Some(n) = num {
                        self.add_token(TokenType::Number(n));
                        continue;
                    }
                }
                '(' => {
                    self.add_token(TokenType::LeftParen);
                    expr_depth += 1;
                },
                ')' => {
                    self.add_token(TokenType::RightParen);
                    if dice_expr_depths.contains(&expr_depth) {
                        self.advance();
                        if let Some(word) = self.read_identifier() {
                            self.add_token(TokenType::DiceOption(word));
                        }
                        dice_expr_depths.remove(&expr_depth);
                        continue;
                    }
                    expr_depth -= 1;
                },
                '[' => self.add_token(TokenType::LeftBracket),
                ']' => self.add_token(TokenType::RightBracket),
                '{' => self.add_token(TokenType::LeftBrace),
                '}' => self.add_token(TokenType::RightBrace),
                '+' => self.add_token(TokenType::Plus),
                '-' => self.add_token(TokenType::Minus),
                '*' => self.add_token(TokenType::Asterisk),
                '^' => self.add_token(TokenType::Hat),
                '/' => self.add_ambiguous_token('/', TokenType::Slash, TokenType::SlashSlash),
                '#' => self.add_token(TokenType::HashSign),
                ',' => self.add_token(TokenType::Comma),
                ';' => self.add_token(TokenType::Semicolon),
                '.' => self.add_ambiguous_token('.', TokenType::Dot, TokenType::DotDot),
                '=' => self.add_ambiguous_token('=', TokenType::Equal, TokenType::EqualEqual),
                '>' => self.add_ambiguous_token('=', TokenType::Greater, TokenType::GreaterEqual),
                '<' => self.add_ambiguous_token('=', TokenType::Less, TokenType::LessEqual),
                '"' | '\'' => {
                    let s = self.read_string_literal(c);
                    self.add_token(TokenType::StringLiteral(s));
                    continue;
                },
                _ => {
                    return Err(format!(
                        "Unrecognized syntax {} in position {}",
                        c, self.current
                    ));
                }
            }
            self.advance();
        }

        Ok(self.tokens.clone())
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(token_type, self.start));
    }

    fn read_identifier(&mut self) -> Option<String> {
        let c = self.peek();
        if c.is_some() && c.unwrap().is_alphabetic() {
            self.read_alphanum()
        } else {
            None
        }
    }

    fn read_alphanum(&mut self) -> Option<String> {
        let mut name = String::new();
        while let Some(&c) = self.peek() {
            if !c.is_alphanumeric() {
                break;
            }
            name.push(c);
            self.advance();
        }
        if name.is_empty() {
            None
        } else {
            Some(name)
        }
    }

    fn read_string_literal(&mut self, start_char: char) -> String {
        let mut name = String::new();
        while let Some(&c) = self.peek() {
            if c == start_char {
                break;
            }
            if c == '\\' && self.advance().is_some() {
                if let Some(cc) = escape_char(c) {
                    name.push(cc);
                }
                continue;
            }
            name.push(c);
        }
        name
    }

    fn read_ambiguity(&mut self, ch: char, t1: TokenType, t2: TokenType) -> TokenType {
        let Some(&c) = self.advance() else {
            return t1;
        };
        if c == ch {
            return t2;
        }
        t1
    }

    fn add_ambiguous_token(&mut self, ch: char, t1: TokenType, t2: TokenType) {
        let t = self.read_ambiguity(ch, t1, t2);
        self.add_token(t);
    }

    fn read_number(&mut self) -> Option<i32> {
        let mut num = String::new();
        while let Some(&c) = self.peek() {
            if !c.is_numeric() {
                break;
            }
            num.push(c);
            self.advance();
        }
        num.parse().ok()
    }
}
