use crate::token::Token;

/// Parser
pub struct Lexer {
    /// The raw input of the parser object.
    input: String,

    /// The index of the current character being represented by the `character` field.
    current: usize,

    /// The character that is at the current position in the input string.
    character: char,
}

impl Lexer {
    /// Creates a new `Parser` object with the provided `String` input.
    pub fn new(input: String) -> Option<Self> {
        if let Some(character) = input.chars().nth(0) {
            Some(Self {
                input,
                current: 0,
                character,
            })
        } else {
            None
        }
    }

    /// Returns the current read position of the calling `Lexer`.
    pub fn current(&self) -> Option<usize> {
        if self.current == (self.input.len() - 1) {
            return None;
        }

        Some(self.current)
    }

    /// Returns the current character of the calling `Lexer`.
    pub fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.current)
    }

    /// Returns the next read position of the calling `Lexer`.
    pub fn next(&self) -> Option<usize> {
        if (self.current + 1) > self.input.len() {
            return None;
        }

        Some(self.current + 1)
    }

    /// Returns the next character of the calling `Lexer`.
    pub fn next_char(&self) -> Option<char> {
        self.input.chars().nth(self.current + 1)
    }

    /// Advances the current position of the parser object by one.
    /// If no valid token has been found, then set the character field to the null bit, '\0'.
    pub fn advance(&mut self) {
        let new_current = self.current + 1;
        if new_current > self.input.len() {
            return;
        }

        let new_char = match self.input.chars().nth(new_current) {
            Some(c) => c,
            None => '\0',
        };

        self.current = new_current;
        self.character = new_char;
    }

    /// Traverses the source input until a non-whitespace character is found.
    fn skip_whitespace(&mut self) -> String {
        let start = self.current;
        loop {
            let c = self
                .current_char()
                .expect("Expected a valid whitespace character.");
            match c {
                ' ' | '\t' | '\n' | '\r' => self.advance(),
                _ => break,
            }
        }
        let end = self.current;

        self.input[start..end].to_string()
    }

    /// Converts the current character into a token if the underlying character is valid.
    /// This is the public interface to the `lex()` function.
    pub fn tokenize(&mut self) -> Option<Token> {
        if self.character == '\0' {
            return None;
        }

        Some(self.lex())
    }

    /// Attempts to read consecutive ASCII characters until a whitespace is encountered.
    /// This lexer method is typically used to tokenize symbols or identifiers,
    /// such as those in variables, function names, class names, trait names, etc.
    fn read_identifier(&mut self) -> String {
        let start = self.current;
        while self.character.is_ascii_alphabetic() {
            self.advance();
        }
        let end = self.current;

        self.input[start..end].to_string()
    }

    /// Attempts to read consecutive ASCII digits until a non-ASCII digit is enountered.
    /// This is the primary lexer method for tokenizing numerical values.
    fn read_number(&mut self) -> String {
        let start = self.current;
        while self.character.is_ascii_digit() {
            self.advance();
        }
        let end = self.current;

        println!("{:?}", self.input[start..end].to_string());
        self.input[start..end].to_string()
    }

    /// The main lexing method of the `Lexer` object. It will translate the current character into
    /// a `TokenType` variant.
    fn lex(&mut self) -> Token {
        // Check if the current character is a whitespace character, and skip until a non-whitespace
        // character is reached.
        let mut literal = self.character.to_string();
        let token_type = match self.character {
            // Whitespace characters
            ' ' | '\t' | '\n' => {
                let skipped = self.skip_whitespace();
                Token::Whitespace(skipped)
            }

            // Alphabetical ASCII characters
            'a'..='z' | 'A'..='Z' => {
                // todo!("Implement `Character` tokenization");
                let identifier = self.read_identifier();
                Token::Identifier(identifier)
            }

            // Numerical characters
            '0'..'9' => {
                todo!("Implement `Number` tokenization");
            }

            // Separators
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,

            // Operators
            '*' => Token::Asterisk,
            '>' => Token::GreaterThan,
            '<' => Token::LessThan,
            '-' => Token::Minus,
            '%' => Token::Percent,
            '+' => Token::Plus,
            '/' => Token::Slash,

            // Equality Operators
            '=' => {
                // If the next character is an equals '=', then the intended symbol
                // should be an equality operation, '=='. Otherwise, it's just an
                // assignment operation.
                todo!("Implement `Equality` and `Assignment` tokenization");
            }
            '!' => {
                // If the next character is an equals '=', then the intended symbol
                // should be an non-equality operation, '!='. Otherwise, it's just a
                // normal bang symbol.
                todo!("Implement `Negated-Equality (or NotEquals)` tokenization")
            }

            // Unused Symbols
            '&' => Token::Ampersand,
            '@' => Token::Asperand,
            '^' => Token::Carrot,
            '$' => Token::Dollar,
            '#' => Token::Pound,
            '~' => Token::Tilde,

            // Other Characters
            ';' => Token::Semicolon,
            ',' => Token::Comma,

            // The "end-of-file" character.
            '\0' => Token::EOF,

            _ => {
                println!("Encountered illegal TokenType: {}", self.character);
                Token::Illegal(literal)
            }
        };

        // advance a final time
        self.advance();

        token_type
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer;
    use crate::token::Token;

    const TEST_INPUT: &str = "let five = 5;\
    let ten = 10;\
    \
    let add = func(x, y) {\
      x + y;\
    };\
    \
    ~-/*5\
    5 < 10 > 5\
    ";

    #[test]
    fn test_lexer_creation() {
        if let Some(created_lexer) = lexer::Lexer::new(TEST_INPUT.to_string()) {
            assert!(created_lexer.character == 'l');
            assert!(created_lexer.current == 0);
            assert!(created_lexer.next().unwrap() == 1);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_advance() {
        let mut test_lexer = lexer::Lexer::new(TEST_INPUT.to_string()).unwrap();
        test_lexer.advance();

        assert!(test_lexer.character == 'e');
        assert!(test_lexer.current == 1);
        assert!(test_lexer.next().unwrap() == 2);
    }

    #[test]
    fn test_lexing_of_symbols() {
        const TEST_INPUT: &str = "~-/*&@^$#";
        let expected_tokens = vec![
            Token::Tilde,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Ampersand,
            Token::Asperand,
            Token::Carrot,
            Token::Dollar,
            Token::Pound,
        ];

        let mut test_lexer = lexer::Lexer::new(TEST_INPUT.to_string()).unwrap();
        for expected in expected_tokens {
            let actual = test_lexer.tokenize().unwrap();
            println!("ACTUAL=`{:?}`, EXPECTED=`{:?}`", &actual, &expected);
            assert!(actual == expected);
        }
    }

    #[test]
    fn test_let_assignment() {
        const TEST_INPUT: &str = "let five = 5;\nlet ten = 10;";

        let mut test_lexer = lexer::Lexer::new(TEST_INPUT.to_string()).unwrap();

        let expected_tokens = vec![
            // Line 1
            Token::Let,
            Token::Identifier("five".to_string()),
            Token::Assignment,
            Token::Number("5".to_string()),
            Token::Semicolon,
            // Line 2
            Token::Let,
            Token::Identifier("ten".to_string()),
            Token::Assignment,
            Token::Number("10".to_string()),
            Token::Semicolon,
        ];

        for expected in expected_tokens {
            let actual = test_lexer.tokenize().unwrap();
            println!("ACTUAL=`{:?}`, EXPECTED=`{:?}`", &actual, &expected);
            assert!(actual == expected);
        }
    }

    #[test]
    fn test_function_assignment() {
        const TEST_INPUT: &str = "let add = func(x, y) {\
          return x + y;\
        }";

        let mut test_lexer = lexer::Lexer::new(TEST_INPUT.to_string()).unwrap();
        let expected_tokens = vec![
            Token::keyword("let"),
            Token::Identifier("add".to_string()),
            Token::Assignment,
            Token::Function,
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::Comma,
            Token::Identifier("y".to_string()),
            Token::RightParen,
            Token::LeftBrace,
            Token::keyword("return"),
            Token::Identifier("x".to_string()),
            Token::Plus,
            Token::Identifier("y".to_string()),
            Token::Semicolon,
            Token::RightBrace,
        ];

        for expected in expected_tokens {
            let actual = test_lexer.tokenize().unwrap();
            println!("ACTUAL=`{:?}`, EXPECTED=`{:?}`", &actual, &expected);
            assert!(actual == expected);
        }
    }

    #[test]
    fn test_equality_symbols() {
        const INPUT: &str = "10 == 10;\
            10 != 9;";

        let mut test_lexer = lexer::Lexer::new(INPUT.to_string()).unwrap();
        let expected_tokens = vec![
            // Line 1
            Token::Number("10".to_string()),
            Token::Equals,
            Token::Number("10".to_string()),
            Token::Semicolon,
            // Line 2
            Token::Number("10".to_string()),
            Token::NotEquals,
            Token::Number("9".to_string()),
            Token::Semicolon,
        ];

        for expected in expected_tokens {
            let actual = test_lexer.tokenize().unwrap();
            println!("ACTUAL=`{:?}`, EXPECTED=`{:?}`", &actual, &expected);
            assert!(actual == expected);
        }
    }
}
