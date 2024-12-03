pub mod conditional_tokenizers {

    use crate::statement_tokenizer::loop_tokenizer::loop_tokenizers::extract_block;
    use crate::statement_tokenizer::tokenizer::tokenizers::ParseInfo;
    use crate::token_type::token_types::TokenTypes;

    fn extract_statement(chars: &[char], mut index: usize) -> (String, usize) {
        let mut statement = String::new();
        let mut parenthesis_count = 1;

        // Extract content within parentheses
        while index < chars.len() && parenthesis_count > 0 {
            let c = chars[index];
            if c == '(' {
                parenthesis_count += 1;
            } else if c == ')' {
                parenthesis_count -= 1;
                if parenthesis_count == 0 {
                    break;
                }
            }
            statement.push(c);
            index += 1;
        }
        (statement, index)
    }

    fn parse_keyword(expression: &str, index: usize, keyword: &str) -> Option<usize> {
        let slice = &expression[index..];
        if slice.starts_with(keyword) {
            Some(index + keyword.len())
        } else {
            None
        }
    }

    pub fn tokenize_if_elif_else_statement(expression: &str, index: usize) -> ParseInfo {
        let chars: Vec<char> = expression.chars().collect();
        let mut j = index;
        let i = 0;

        let keyword = String::new();

        while j < chars.len() {
            // Handle 'if'
            if let Some(new_index) = parse_keyword(expression, j, "if") {
                j = new_index;
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }
                if chars[j] == '(' {
                    let (statement, new_j) = extract_statement(&chars, j + 1);
                    return ParseInfo::new(
                        TokenTypes::If { statement },
                        new_j.try_into().unwrap(),
                        "if".to_string(),
                    );
                }
            }
            // Handle 'elif'
            else if let Some(new_index) = parse_keyword(expression, j, "elif") {
                j = new_index;
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }
                if chars[j] == '(' {
                    let (statement, new_j) = extract_statement(&chars, j + 1);
                    return ParseInfo::new(
                        TokenTypes::Elif { statement },
                        new_j.try_into().unwrap(),
                        "elif".to_string(),
                    );
                }
            }
            // Handle 'else'
            else if let Some(new_index) = parse_keyword(expression, j, "else") {
                j = new_index;
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }
                if chars[j] == '{' {
                    // Consume the '{' and create an Else token
                    j += 1; // skip over '{'
                    return ParseInfo::new(
                        TokenTypes::Else, // You may want to create a new TokenTypes variant if you need additional info.
                        j.try_into().unwrap(),
                        "else".to_string(),
                    );
                }
            }

            j += 1;
        }

        ParseInfo::new(TokenTypes::None, 0, "none".to_string())
    }

    pub fn tokenize_try_catch_finally_statement(expression: &str, index: usize) -> ParseInfo {
        let chars: Vec<char> = expression.chars().collect();
        let mut j = index;

        while j < chars.len() {
            let c = chars[j];
            if c.is_whitespace() {
                j += 1;
                continue;
            }

            if let Some(new_index) = parse_keyword(expression, j, "try") {
                let body: Vec<String>;
                let index_of_curly_brace = expression[j..].find('{').unwrap();
                let mut new_index = new_index + index_of_curly_brace;

                (body, new_index) = extract_block(&chars, new_index + 1);

                return ParseInfo::new(
                    TokenTypes::Try { block: body },
                    new_index.try_into().unwrap(),
                    "try".to_string(),
                );
            } else if let Some(new_index) = parse_keyword(expression, j, "catch") {
                let body: Vec<String>;
                let mut new_index = new_index;

                (body, new_index) = extract_block(&chars, new_index + 2);

                return ParseInfo::new(
                    TokenTypes::Catch { block: body },
                    (new_index - index).try_into().unwrap(),
                    "catch".to_string(),
                );
            } else if let Some(new_index) = parse_keyword(expression, j, "finally") {
                let body: Vec<String>;
                let mut new_index: usize = new_index;

                (body, new_index) = extract_block(&chars, new_index + 2);

                return ParseInfo::new(
                    TokenTypes::Finally { block: body },
                    (new_index - index).try_into().unwrap(),
                    "finally".to_string(),
                );
            }

            j += 1;
        }

        ParseInfo::new(TokenTypes::None, 0, "none".to_string())
    }
}
