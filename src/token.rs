/// Parser API

// #[derive(Debug)]
// pub struct Token {
//     pub token_type: TokenType,
//     pub literal: String,
// }
//
// impl Token {
//     pub fn new(token_type: TokenType, literal: String) -> Self {
//         Self {
//             token_type,
//             literal,
//         }
//     }
// }

#[derive(Debug, PartialEq)]
pub enum Token {
    // a-z, A-Z
    Character(char),

    // 0-9
    Number(String),

    // ()
    LeftParen,
    RightParen,

    // []
    RightBracket,
    LeftBracket,

    // {}
    RightBrace,
    LeftBrace,

    // Other characters
    Semicolon,
    Comma,

    // Unused Symbols
    Ampersand,
    Asperand,
    Carrot,
    Dollar,
    Pound,
    Tilde,

    // Operators
    Assignment,
    Asterisk,
    Bang,
    Equals,
    GreaterThan,
    LessThan,
    Minus,
    NotEquals,
    Percent,
    Plus,
    Slash,

    // Keywords
    Define,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    ElseIf,
    Return,

    // Special lexer types
    Unknown(String),
    Identifier(String),
    Illegal(String),
    Whitespace(String),
    EOF,
}

impl Token {
    pub fn keyword(keyword_str: &str) -> Token {
        match keyword_str {
            "func" => Token::Function,
            "let" => Token::Let,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "else if" => Token::ElseIf,
            "return" => Token::Return,
            _ => Token::Identifier(keyword_str.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {}
