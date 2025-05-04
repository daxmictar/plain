#[allow(dead_code)]
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
    Underscore,

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
    Unknown(char),
    Identifier(String),
    Illegal(char),
    Whitespace(char),

    // The 'extra-special' end-of-file character.
    EOF,
}

#[allow(dead_code)]
impl Token {
    pub fn check_if_keyword(keyword_str: String) -> Token {
        println!("{:?}", keyword_str);
        match keyword_str.trim() {
            "func" => Token::Function,
            "let" => Token::Let,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "else if" => Token::ElseIf,
            "return" => Token::Return,
            _ => Token::Identifier(keyword_str),
        }
    }
}

#[cfg(test)]
mod tests {}
