use crate::token::Token;

/// Parser
#[derive(Debug)]
pub struct Lexer {
    /// The raw input of the parser object.
    input: String,

    /// The index of the current character being represented by the `character` field.
    position: usize,
}

#[allow(dead_code)]
impl Lexer {
    /// Creates a new `Parser` object with the provided `String` input.
    pub fn new(input: &str) -> Option<Self> {
        if let Some(_) = input.chars().nth(0) {
            Some(Self {
                input: input.to_string(),
                position: 0,
            })
        } else {
            None
        }
    }

    /// Returns the current read position of the calling `Lexer`.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Returns the current character of the calling `Lexer`.
    pub fn char(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    /// Returns the next read position of the calling `Lexer`.
    pub fn next(&self) -> Option<usize> {
        Some(self.position())
    }

    /// Checks if there is a next character of the calling `Lexer`.
    pub fn has_next(&self) -> bool {
        if let Some(_) = self.next() {
            return true;
        }

        false
    }

    /// Returns the next character of the calling `Lexer`.
    pub fn next_char(&self) -> Option<char> {
        self.input.chars().nth(self.position + 1)
    }

    /// Advances the current position of the parser object by one.
    /// If no valid token has been found, then set the character field to the null bit, '\0'.
    pub fn advance(&mut self) {
        let new_current = self.position + 1;
        if new_current > self.input.len() {
            return;
        }

        let new_char = match self.input.chars().nth(new_current) {
            Some(c) => c,
            None => '\0',
        };

        self.position = new_current;
    }

    /// Traverses the source input until a non-whitespace character is found.
    fn skip_whitespace(&mut self) -> String {
        let start = self.position;
        loop {
            let c = self.char().expect("Expected a valid whitespace character.");
            match c {
                ' ' | '\t' | '\n' | '\r' => self.advance(),
                _ => break,
            }
        }
        let end = self.position;

        self.input[start..end].to_string()
    }

    /// Converts the current character into a token if the underlying character is valid.
    /// This is the public interface to the `lex()` function.
    pub fn tokenize(&mut self) -> Option<Token> {
        Some(self.lex())
    }

    /// The main lexing method of the `Lexer` object. It will translate the current character into
    /// a `TokenType` variant.
    fn lex(&mut self) -> Token {
        // Check if the current character is a whitespace character, and skip until a non-whitespace
        // character is reached.
        let current_char = match self.input.chars().nth(self.position) {
            Some(c) => c,
            None => '\0',
        };

        let token_type = match current_char {
            // Whitespace characters
            ' ' | '\t' | '\n' => {
                todo!("Implement `Whitespace` tokenization");
            }

            // Alphabetical ASCII characters
            'a'..='z' | 'A'..='Z' => {
                todo!("Implement `Character` tokenization");
            }

            // Numerical characters
            '0'..'9' => {
                todo!("Implement `Number` tokenization");
            }

            // Equality Operators
            '=' => {
                // If the next character is an equals '=', then the intended symbol
                // should be an equality operation, '=='. Otherwise, it's just an
                // assignment operation.
                todo!("Implement `Equality` and `Assignment` tokenization")
            }

            '!' => {
                // If the next character is an equals '=', then the intended symbol
                // should be an non-equality operation, '!='. Otherwise, it's just a
                // normal bang symbol.
                todo!("Implement `Negated-Equality (or NotEquals)` tokenization")
            }

            // Separators
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,

            // Operators/Symbols
            '*' => Token::Asterisk,
            '>' => Token::GreaterThan,
            '<' => Token::LessThan,
            '-' => Token::Minus,
            '%' => Token::Percent,
            '+' => Token::Plus,
            '/' => Token::Slash,
            '&' => Token::Ampersand,
            '@' => Token::Asperand,
            '^' => Token::Carrot,
            '$' => Token::Dollar,
            '#' => Token::Pound,
            '~' => Token::Tilde,

            // Delimiters
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '_' => Token::Underscore,

            // The "end-of-file" character.
            '\0' => Token::EOF,

            _ => {
                println!("Encountered illegal TokenType: {}", current_char);
                Token::Illegal(current_char)
            }
        };

        self.advance();

        token_type
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::Token;

    #[test]
    fn test_lexer_creation() {
        // lexers will accept a string of bytes which
        // requires the user of the Lexer struct
        // to read the input from the file `first`
        const TEST_INPUT: &str = "let a = 1";

        // and then pass the contents to the lexer
        let test_lexer = Lexer::new(TEST_INPUT);
        if let Some(lexer) = test_lexer {
            dbg!(&lexer);
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_lexing_of_symbols() {
        const TEST_INPUT: &str = "*<>-%+/@^$#~";

        let mut test_lexer = Lexer::new(TEST_INPUT).unwrap();

        let expected_tokens = vec![
            Token::Asterisk,
            Token::LessThan,
            Token::GreaterThan,
            Token::Minus,
            Token::Percent,
            Token::Plus,
            Token::Slash,
            Token::Asperand,
            Token::Carrot,
            Token::Dollar,
            Token::Pound,
            Token::Tilde,
        ];

        for expected in expected_tokens {
            let actual = test_lexer.tokenize().unwrap();
            dbg!(&actual, &expected);
            assert!(actual == expected);
        }
    }

    #[test]
    fn test_lexing_of_delimiters() {
        const TEST_INPUT: &str = ",;";

        let mut test_lexer = Lexer::new(TEST_INPUT).unwrap();

        let expected_tokens = vec![Token::Comma, Token::Semicolon];

        for expected in expected_tokens {
            let actual = test_lexer.tokenize().unwrap();
            dbg!(&actual, &expected);
            assert!(actual == expected);
        }
    }
}
