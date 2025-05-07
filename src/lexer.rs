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

    /// Returns the next character of the calling `Lexer`.
    pub fn next_char(&self) -> Option<char> {
        self.input.chars().nth(self.position + 1)
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

    /// Advances the current position of the parser object by one.
    /// If no valid token has been found, then set the character field to the null bit, '\0'.
    fn advance(&mut self) {
        let new_position = self.position + 1;
        if new_position > self.input.len() {
            return;
        }

        self.position = new_position;
    }

    fn discriminate_character<F>(&mut self, predicate: F) -> (usize, usize, String)
    where
        F: Fn(char) -> bool,
    {
        let start = self.position();
        while let Some(c) = self.char() {
            if !predicate(c) {
                break;
            }

            self.advance();
        }
        let end = self.position();

        return (start, end, self.input[start..end].to_string());
    }

    /// The main lexing method of the `Lexer` object. It will translate the current character into
    /// a `TokenType` variant.
    fn lex(&mut self) -> Token {
        // skip any whitespace characters
        let skipped = self.discriminate_character(|c| c.is_ascii_whitespace());
        println!(
            "Detected whitespace characters from {} to {}",
            skipped.0, skipped.1
        );

        // Check if the current character is a whitespace character, and skip until a non-whitespace
        // character is reached.
        let current_char = match self.input.chars().nth(self.position) {
            Some(c) => c,
            None => '\0',
        };

        let token_type = match current_char {
            // Alphabetical ASCII characters
            'a'..='z' | 'A'..='Z' => {
                let results = self.discriminate_character(|c| c.is_ascii_alphabetic());
                Token::check_if_keyword(results.2.to_string())
            }

            // Numerical characters
            '0'..'9' => {
                let result = self.discriminate_character(|c| c.is_ascii_alphanumeric());
                Token::Number(result.2)
            }

            // Equality Operators
            '=' => {
                // If the next character is an equals '=', then the intended symbol
                // should be an equality operation, '=='. Otherwise, it's just an
                // assignment operation.
                let next_char = match self.next_char() {
                    Some(c) => c,
                    None => return Token::Illegal(self.char().expect("expected an illegal character")),
                };

                if next_char == '=' {
                    // TODO: Fix manually adjusting position by 2
                    self.position += 2;
                    return Token::Equals
                }

                Token::Assignment
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

    /// Converts the current character into a token if the underlying character is valid.
    /// This is the public interface to the `lex()` function.
    pub fn tokenize(&mut self) -> Token {
        self.lex()
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
            let actual = test_lexer.tokenize();
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
            let actual = test_lexer.tokenize();
            dbg!(&actual, &expected);
            assert!(actual == expected);
        }
    }

    #[test]
    fn test_lexing_of_keyword() {
        const TEST_INPUT: &str = "let add";

        // and then pass the contents to the lexer
        let mut test_lexer = Lexer::new(TEST_INPUT).unwrap();

        let expected_tokens = vec![Token::Let, Token::Identifier("add".to_string())];

        for expected in expected_tokens {
            let actual = test_lexer.tokenize();
            dbg!(&actual, &expected);
            assert!(actual == expected);
        }
    }

    #[test]
    fn test_lexing_of_a_number() {
        const TEST_INPUT: &str = "123456 654321";

        // and then pass the contents to the lexer
        let mut test_lexer = Lexer::new(TEST_INPUT).unwrap();

        let expected_tokens = vec![
            Token::Number("123456".to_string()),
            Token::Number("654321".to_string()),
        ];

        for expected in expected_tokens {
            let actual = test_lexer.tokenize();
            dbg!(&actual, &expected);
            assert!(actual == expected);
        }
    }

    #[test]
    fn test_lexing_of_assignment() {
        const TEST_INPUT: &str = "let a = 5";

        // and then pass the contents to the lexer
        let mut test_lexer = Lexer::new(TEST_INPUT).unwrap();

        let expected_tokens = vec![
            Token::Let,
            Token::Identifier("a".to_string()),
            Token::Assignment,
            Token::Number("5".to_string())
        ];

        for expected in expected_tokens {
            let actual = test_lexer.tokenize();
            dbg!(&actual, &expected);
            assert!(actual == expected);
        }
    }

    #[test]
    fn test_lexing_of_equality() {
        const TEST_INPUT: &str = "a == 5";

        // and then pass the contents to the lexer
        let mut test_lexer = Lexer::new(TEST_INPUT).unwrap();

        let expected_tokens = vec![
            Token::Identifier("a".to_string()),
            Token::Equals,
            Token::Number("5".to_string())
        ];

        for expected in expected_tokens {
            let actual = test_lexer.tokenize();
            dbg!(&actual, &expected);
            assert!(actual == expected);
        }
    }
}
