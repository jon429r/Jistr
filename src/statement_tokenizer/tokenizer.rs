/*
* This file takes in the wrtten source code and tokenizes it into a vector of tokens
* These tokens can then be used to create an AST
*/

pub mod tokenizers {
    use crate::statement_tokenizer::basic_tokenizer::basic_tokenizers::{
        read_boolean, read_numbers, read_object_call, read_operators, read_strings_chars,
    };
    use crate::statement_tokenizer::collection_tokenizer::collection_tokenizers::read_collection_assignment;
    use crate::statement_tokenizer::function_tokenizer::function_tokenizers::{
        read_function_assignment, read_function_call,
    };
    use crate::statement_tokenizer::variable_tokenizer::variable_tokenizers::{
        read_variable_assignment, read_variable_call, read_variable_declaration,
    };

    use crate::statement_tokenizer::conditional_tokenizer::conditional_tokenizers::tokenize_if_elif_else_statement;

    use crate::statement_tokenizer::conditional_tokenizer::conditional_tokenizers::tokenize_try_catch_finally_statement;
    // Importing the tokenizer for for-while loops
    use crate::statement_tokenizer::loop_tokenizer::loop_tokenizers::tokenize_for_while_statement;

    use crate::token_type::token_types::TokenTypes;
    use std::char;
    use std::process::exit;

    #[derive(Debug, PartialEq, Clone)]
    pub struct ParseInfo {
        pub token: TokenTypes,
        pub chars_read: i32,
        pub value: String,
    }

    impl ParseInfo {
        pub fn new(token: TokenTypes, chars_read: i32, value: String) -> Self {
            ParseInfo {
                token,
                chars_read,
                value,
            }
        }
        pub fn to_string(&self) -> String {
            let mut str = String::new();
            str.push_str("TokenType: ");
            str.push_str(&self.token.to_string());
            str.push_str("Chars read: ");
            str.push_str(&self.chars_read.to_string());
            str.push_str("value: ");
            str.push_str(&self.value.to_string());
            return str;
        }
    }
    impl ParseInfo {
        fn eq(&self, other: &Self) -> bool {
            self.token == other.token && self.chars_read == other.chars_read
        }
    }

    pub struct Token<T> {
        pub value: T,
        pub token_type: TokenTypes,
    }
    impl<T: std::fmt::Display> Token<T> {
        pub fn to_string(&self) -> String {
            let mut str = String::new();
            str.push_str(&self.value.to_string());
            str.push_str(&self.token_type.to_string());
            return str;
        }
    }

    impl<T> Token<T> {
        pub fn new(value: T, token_type: TokenTypes) -> Self {
            Token { value, token_type }
        }
    }
    static mut MULTLINECOMMENT: bool = false;

    pub fn tokenize(expression: String) -> Vec<ParseInfo> {
        let mut token_list: Vec<ParseInfo> = Vec::new();
        let none = ParseInfo::new(TokenTypes::None, 0, "none".to_string());

        let mut index = 0;
        let chars: Vec<char> = expression.chars().collect();

        while index < chars.len() {
            let char = chars[index];
            let nextchar = if index + 1 < chars.len() {
                chars[index + 1]
            } else {
                '\0' // Null character for out-of-bounds safety
            };

            if char == ';' {
                index += 1;
                token_list.push(ParseInfo::new(TokenTypes::SemiColon, 1, ";".to_string()));
                continue;
            }

            if char == ' ' || char == '\n' || char == '\r' || char == '\t' {
                index += 1;
                continue; // Skip whitespace characters
            }

            unsafe {
                if MULTLINECOMMENT {
                    // Skip characters within multi-line comments
                    while index < chars.len() {
                        let char = chars[index];
                        let nextchar = if index + 1 < chars.len() {
                            chars[index + 1]
                        } else {
                            '\0'
                        };
                        if char == '*' && nextchar == '/' {
                            MULTLINECOMMENT = false;
                            index += 2;
                            break;
                        }
                        index += 1;
                    }
                    continue;
                }

                if char == '/' && nextchar == '/' {
                    // Single-line comment - skip the rest of the line
                    let info = ParseInfo::new(
                        TokenTypes::Comment,
                        (chars.len() - index) as i32,
                        "none".to_string(),
                    );
                    token_list.push(info);
                    break;
                } else if char == '/' && nextchar == '*' {
                    MULTLINECOMMENT = true;
                    index += 2;
                    continue;
                }
            }

            // Process regular tokens
            let info: ParseInfo = read_token(&expression, index);
            if info.token != none.token {
                index += info.chars_read as usize;
                token_list.push(info);
                continue;
            }

            index += 1;
        }

        token_list
    }

    pub fn read_token(expression: &String, index: usize) -> ParseInfo {
        let none: ParseInfo = ParseInfo::new(TokenTypes::None, 0, "none".to_string());
        let mut j = index;
        let mut decimals = 0;

        // check for [ and ]
        if expression.chars().nth(j).unwrap() == '[' {
            let info: ParseInfo = ParseInfo::new(TokenTypes::LeftBracket, 1, "[".to_string());
            return info;
        } else if expression.chars().nth(j).unwrap() == ']' {
            let info: ParseInfo = ParseInfo::new(TokenTypes::RightBracket, 1, "]".to_string());
            return info;
        }

        // check for fat arrow
        if expression.chars().nth(j).unwrap() == '=' {
            if expression.chars().nth(j + 1).unwrap() == '>' {
                let info: ParseInfo = ParseInfo::new(TokenTypes::FatArrow, 2, "=>".to_string());
                return info;
            }
        }

        // Loop through the expression
        while j < expression.len() {
            let char: char = expression.chars().nth(j).unwrap();

            // Break if the character is not a digit or decimal point
            if !char.is_digit(10) && char != '.' {
                break;
            }

            // Count decimal points
            if char == '.' {
                decimals += 1;
                // If there are multiple decimal points, return None
                if decimals > 1 {
                    return none;
                }
            }

            j += 1;
        }

        let char = expression.chars().nth(index).unwrap();

        let info = read_object_call(expression);
        if info.token != none.token {
            return info;
        }

        let info = read_boolean(expression.to_string(), index);
        if info.token != none.token {
            return info;
        }

        let info = read_numbers(expression.to_string(), char, index);
        if info.token != none.token {
            return info;
        }

        let next_char = expression.chars().nth(index + 1).unwrap_or('\0');
        let info = read_operators(expression.to_string(), char, next_char, index);
        if info.token != none.token {
            return info;
        }
        let info = read_strings_chars(expression.to_string(), char, index);

        if info.token != none.token {
            return info;
        }

        let info = read_variable_declaration(expression, index);
        if info.token != none.token {
            return info;
        }

        let info = read_variable_assignment(expression, index);
        if info.token != none.token {
            return info;
        }

        let info = tokenize_if_elif_else_statement(expression, index);
        if info.token != none.token {
            return info;
        }

        let info = tokenize_for_while_statement(expression, index);
        if info.token != none.token {
            return info;
        }

        let info = tokenize_try_catch_finally_statement(expression, index);
        if info != none {
            return info;
        }

        let info = read_function_call(expression, index);
        if info.token != none.token {
            return info;
        }

        let info = read_function_assignment(expression, index);
        if info.token != none.token {
            return info;
        }

        let info = read_variable_call(expression, index);
        if info.token != none.token {
            return info;
        }

        let info = read_collection_assignment(expression, index);
        if info.token != none.token {
            return info;
        }
        let error: String = format!(
            "Error: No token found for expression {}",
            expression[index..].to_string()
        );
        println!("{}", error);
        exit(1);

        /*
                // tokenize char value if it matches 'a' to 'z' or 'A' to 'Z'
                if char.is_alphabetic() {
                    let chars_read = 1;
                    return ParseInfo::new(
                        TokenTypes::Char,
                        chars_read.try_into().unwrap(),
                        char.to_string(),
                    );
                }
        */
        /*
        // check for string values
        if char == '"' {
            let mut j = index + 1;
            while j < expression.len() {
                if expression.chars().nth(j).unwrap() == '"' {
                    break;
                }
                j += 1;
            }
            let chars_read = j - index + 1;
            return ParseInfo::new(
                TokenTypes::String,
                chars_read.try_into().unwrap(),
                expression[index..j + 1].to_string(),
            );
        }*/
        return none;
    }
}
