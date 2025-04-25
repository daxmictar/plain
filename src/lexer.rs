use crate::token::{TokenType, Token};

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
                character
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
    pub fn advance(&mut self, verbose: bool) {
        let new_current = self.current + 1;
        if new_current > self.input.len() {
            if verbose {
                println!("Could not advance to {} from {}", self.current, new_current);
            }

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
    fn skip_whitespace(&mut self) {
        loop {
            let c = self.current_char().expect("Expected a valid whitespace character.");
            match c {
                ' ' | '\t' | '\n' | '\r' => self.advance(false),
                _ => break,
            }
        }
    } 

    /// Converts the current character into a token if the underlying character is valid.
    /// This is the public interface to the `lex()` function.
    pub fn tokenize(&mut self) -> Option<Token> {
        if self.character == '\0' {
            return None
        }

        Some(self.lex())
    }

    /// Attempts to read consecutive ASCII characters until a whitespace is encountered. 
    /// This lexer method is typically used to tokenize symbols or identifiers, 
    /// such as those in variables, function names, class names, trait names, etc.
    fn read_identifier(&mut self) -> String {
        let start = self.current;
        while self.character.is_ascii_alphabetic() {
            self.advance(true);
        }
        let end = self.current;

        self.input[start..end].to_string()
    }

    /// Attempts to read consecutive ASCII digits until a non-ASCII digit is enountered.
    /// This is the primary lexer method for tokenizing numerical values.
    fn read_number(&mut self) -> String {
        let start = self.current;
        while self.character.is_ascii_digit() {
            self.advance(true);
        }
        let end = self.current;

        self.input[start..end].to_string()
    }

    /// The main lexing method of the `Lexer` object. It will translate the current character into
    /// a `TokenType` variant.
    fn lex(&mut self) -> Token {
        // Check if the current character is a whitespace character, and skip until a non-whitespace
        // character is reached.
        self.skip_whitespace();

        let mut literal = self.character.to_string();
        let token_type = match self.character {
            // Alphabetical ASCII characters
            'a'..='z' | 'A'..='Z' => {
                todo!()
            },

            // Separators
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            '[' => TokenType::LeftBracket,
            ']' => TokenType::RightBracket,

            // Operators
            '*' => TokenType::Asterisk,
            '>' => TokenType::GreaterThan,
            '<' => TokenType::LessThan,
            '-' => TokenType::Minus,
            '%' => TokenType::Percent,
            '+' => TokenType::Plus,
            '/' => TokenType::Slash,

            // Equality Operators
            '=' =>  {
                // If the next character is an equals '=', then the intended symbol
                // should be an equality operation, '=='. Otherwise, it's just an
                // assignment operation.
                todo!("Implement `Equality` and `Assignment` tokenization")
            },
            '!' =>  {
                // If the next character is an equals '=', then the intended symbol
                // should be an non-equality operation, '!='. Otherwise, it's just a
                // normal bang symbol.
                todo!("Implement `Negated-Equality (or NotEquals)` tokenization")
            },

            // Unused Symbols
            '&' => TokenType::Ampersand,
            '@' => TokenType::Asperand,
            '^' => TokenType::Carrot,
            '$' => TokenType::Dollar,
            '#' => TokenType::Pound,
            '~' => TokenType::Tilde,

            // Other Characters
            ';' => TokenType::Semicolon,
            ',' => TokenType::Comma,

            // The "end-of-file" character.
            '\0' => TokenType::EOF,

            _ => {
                if self.character.is_ascii_alphabetic() {
                    literal = self.read_identifier();
                    TokenType::keyword(&literal)
                } else if self.character.is_ascii_digit() {
                    literal = self.read_number(); 
                    println!("{}", self.current);
                    TokenType::Number
                } else {
                    TokenType::Illegal
                }
            }
        };

        // Ensure that we advance our scanning index by one each time we determine the correct
        // token type for the current lex.
        self.advance(true);
        Token::new(token_type, literal)
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer;
    use crate::token::{TokenType, Token};

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
        test_lexer.advance(false);

        assert!(test_lexer.character == 'e');
        assert!(test_lexer.current == 1);
        assert!(test_lexer.next().unwrap() == 2);
    }

    #[test]
    fn test_lexing_of_symbols() {
        const TEST_INPUT: &str = "~-/*5&@^$#";

        let mut test_lexer = lexer::Lexer::new(TEST_INPUT.to_string()).unwrap();

        let expected_tokens = vec![
            Token{token_type:TokenType::Tilde, literal:"~".to_string()},
            Token{token_type:TokenType::Minus, literal:"-".to_string()},
            Token{token_type:TokenType::Slash, literal:"/".to_string()},
            Token{token_type:TokenType::Asterisk, literal:"*".to_string()},
            Token{token_type:TokenType::Number, literal:"5".to_string()},
            Token{token_type:TokenType::Ampersand, literal:"&".to_string()},
            Token{token_type:TokenType::Asperand, literal:"@".to_string()},
            Token{token_type:TokenType::Carrot, literal:"^".to_string()},
            Token{token_type:TokenType::Dollar, literal:"$".to_string()},
            Token{token_type:TokenType::Pound, literal:"#".to_string()},
        ];

        for expected in expected_tokens {
            let actual = test_lexer.tokenize().unwrap();
            dbg!(&actual.literal, &expected.literal);
            dbg!(&actual.token_type, &expected.token_type); 
            assert!(&actual.literal == &expected.literal);
            assert!(&actual.token_type == &expected.token_type);
        }
    }

    #[test]
    fn test_let_assignment() {
        const TEST_INPUT: &str = "let five = 5;\nlet ten = 10;";

        let mut test_lexer = lexer::Lexer::new(TEST_INPUT.to_string()).unwrap();

        let expected_tokens = vec![
            // Line 1
            Token{token_type: TokenType::Let, literal: "let".to_string()},
            Token{token_type: TokenType::Identifier, literal: "five".to_string()},
            Token{token_type: TokenType::Assignment, literal: "=".to_string()},
            Token{token_type: TokenType::Number, literal: "5".to_string()},
            Token{token_type: TokenType::Semicolon, literal: ";".to_string()},
            // Line 2
            Token{token_type: TokenType::Let, literal: "let".to_string()},
            Token{token_type: TokenType::Identifier, literal: "ten".to_string()},
            Token{token_type: TokenType::Assignment, literal: "=".to_string()},
            Token{token_type: TokenType::Number, literal: "10".to_string()},
            Token{token_type: TokenType::Semicolon, literal: ";".to_string()},
        ];

        for expected in expected_tokens {
            let actual = test_lexer.tokenize().expect("Expected a token literal");
            dbg!(&actual.literal, &expected.literal, &actual.token_type, &expected.token_type, "");
            // dbg!(&actual.token_type, &expected.token_type); 
            assert!(actual.literal == expected.literal);
            assert!(actual.token_type == expected.token_type);
        }
    }

    #[test]
    fn test_function_assignment() {
        const TEST_INPUT: &str = "let add = func(x, y) {\
          return x + y;\
        }";
        let mut test_lexer = lexer::Lexer::new(TEST_INPUT.to_string()).unwrap();
        let expected_tokens = vec![
            Token{token_type: TokenType::Let, literal: "let".to_string()},
            Token{token_type: TokenType::Identifier, literal: "add".to_string()},
            Token{token_type: TokenType::Assignment, literal: "=".to_string()},
            Token{token_type: TokenType::Function, literal: "func".to_string()},
            Token{token_type: TokenType::LeftParen, literal: "(".to_string()},
            Token{token_type: TokenType::Identifier, literal: "x".to_string()},
            Token{token_type: TokenType::Comma, literal: ",".to_string()},
            Token{token_type: TokenType::Identifier, literal: "y".to_string()},
            Token{token_type: TokenType::RightParen, literal: ")".to_string()},
            Token{token_type: TokenType::LeftBrace, literal: "{".to_string()},
            Token{token_type: TokenType::Return, literal: "return".to_string()},
            Token{token_type: TokenType::Identifier, literal: "x".to_string()},
            Token{token_type: TokenType::Plus, literal: "+".to_string()},
            Token{token_type: TokenType::Identifier, literal: "y".to_string()},
            Token{token_type: TokenType::Semicolon, literal: ";".to_string()},
            Token{token_type: TokenType::RightBrace, literal: "}".to_string()},
        ];

        for expected in expected_tokens {
            let actual = test_lexer.tokenize().unwrap();
            dbg!(&actual.literal, &expected.literal);
            dbg!(&actual.token_type, &expected.token_type); 
            assert!(actual.literal == expected.literal);
            assert!(actual.token_type == expected.token_type);
        }
    }

    #[test]
    fn test_equality_symbols() {
        const INPUT: &str = "10 == 10;\
            10 != 9;";

        let mut test_lexer = lexer::Lexer::new(INPUT.to_string()).unwrap();
        let expected_tokens = vec![
            // Line 1
            Token{token_type: TokenType::Number, literal: "10".to_string()},
            Token{token_type: TokenType::Equals, literal: "==".to_string()},
            Token{token_type: TokenType::Number, literal: "10".to_string()},
            Token{token_type: TokenType::Semicolon, literal: ";".to_string()},
            // Line 2
            Token{token_type: TokenType::Number, literal: "10".to_string()},
            Token{token_type: TokenType::NotEquals, literal: "!=".to_string()},
            Token{token_type: TokenType::Number, literal: "9".to_string()},
            Token{token_type: TokenType::Semicolon, literal: ";".to_string()},
        ];

        for expected in expected_tokens {
            let actual = test_lexer.tokenize().unwrap();
            dbg!(&actual.literal, &expected.literal);
            dbg!(&actual.token_type, &expected.token_type);
            assert!(actual.literal == expected.literal);
            assert!(actual.token_type == expected.token_type);
        }
    }
}
