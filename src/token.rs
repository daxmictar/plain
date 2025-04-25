/// Parser API

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Self {
            token_type,
            literal,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
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

    // a-z
    Character,

    // 1-9
    Number,

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
    Unknown,
    Identifier,
    Illegal,
    EOF,
}

impl TokenType {
    pub fn keyword(keyword_str: &str) -> TokenType {
        match keyword_str {
            "func" => TokenType::Function,
            "let" => TokenType::Let,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "else if" => TokenType::ElseIf,
            "return" => TokenType::Return,
            _ => TokenType::Identifier,
        }
    }
}

#[cfg(test)]
mod tests {

}
