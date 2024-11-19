pub mod loop_tokenizers {
    use crate::statement_tokenizer::tokenizer::tokenizers::ParseInfo;
    use crate::token_type::token_types::TokenTypes;

    fn extract_condition(chars: &[char], mut index: usize) -> (String, usize) {
        let mut condition = String::new();
        let mut parenthesis_count = 1;

        // Extract content within parentheses
        while index < chars.len() && parenthesis_count > 0 {
            let c = chars[index];
            if c == '(' {
                parenthesis_count += 1;
            } else if c == ')' {
                parenthesis_count -= 1;
                if parenthesis_count == 0 {
                    index += 1;

                    break;
                }
            }
            condition.push(c);
            index += 1;
        }
        //tokenize the condition
        (condition, index)
    }

    fn extract_block(chars: &[char], mut index: usize) -> (Vec<String>, usize) {
        let mut block: Vec<String> = Vec::new();
        let mut line: String = String::new();

        let mut curly_brace_count = 1; // Start at 1 because we've entered an outer `{`.

        while index < chars.len() && curly_brace_count > 0 {
            let c = chars[index];

            match c {
                '{' => {
                    curly_brace_count += 1;
                    line.push(c);
                }
                '}' => {
                    curly_brace_count -= 1;
                    if curly_brace_count == 0 {
                        index += 1; // Move past the closing brace
                        break;
                    } else {
                        line.push(c);
                    }
                }
                ';' => {
                    line.push(c);
                    // if next is } then add } to end of block
                    // search for next character that is not is_whitespace
                    while index < chars.len() && chars[index].is_whitespace() {
                        index += 1;
                    }

                    if chars[index] == '}' {
                        line.push('}');
                        index += 1;
                    }
                    if !line.trim().is_empty() {
                        block.push(line.trim().to_string());
                        line.clear();
                    }
                }
                '\n' | '\r' | '\t' => {
                    // Ignore specific whitespace characters
                }
                _ => {
                    line.push(c);
                }
            }

            index += 1;
        }

        // If any content is left in `line`, add it to the block
        if !line.trim().is_empty() {
            block.push(line.trim().to_string());
        }

        // If block is empty, add an empty string
        if block.is_empty() {
            block.push(String::new());
        }

        if let Some(last) = block.last() {
            if last.trim() == "}" {
                // Remove the standalone '}' and append it to the previous line
                block.pop();
                let mut new_last = block.pop().unwrap();
                new_last.push('}');
                block.push(new_last);
            }
        }

        (block, index)
    }

    fn parse_keyword(expression: &str, index: usize, keyword: &str) -> Option<usize> {
        let slice = &expression[index..];
        if slice.starts_with(keyword) {
            Some(index + keyword.len())
        } else {
            None
        }
    }

    pub fn tokenize_for_while_statement(expression: &str, index: usize) -> ParseInfo {
        let chars: Vec<char> = expression.chars().collect();
        let mut j = index;

        while j < chars.len() {
            let c = chars[j];
            if c.is_whitespace() {
                j += 1;
                continue;
            }

            // Tokenize the "for" loop
            if let Some(new_index) = parse_keyword(expression, j, "for") {
                j = new_index;

                // Look for '(' and extract the loop condition
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }

                if chars[j] == '(' {
                    let (condition, mut new_j) = extract_condition(&chars, j);
                    //look for starting {
                    while new_j < chars.len() && chars[new_j].is_whitespace() || chars[new_j] == '{'
                    {
                        new_j += 1;
                    }
                    let (resulting_block, final_index) = extract_block(&chars, new_j);
                    return ParseInfo::new(
                        TokenTypes::For {
                            statement: condition,
                            block: resulting_block,
                        },
                        final_index.try_into().unwrap(),
                        "for".to_string(),
                    );
                }

            // Tokenize the "while" loop
            } else if let Some(new_index) = parse_keyword(expression, j, "while") {
                j = new_index;

                // Look for '(' and extract the loop condition
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }

                if chars[j] == '(' {
                    let (condition, mut new_j) = extract_condition(&chars, j + 1);
                    //look for starting {
                    while new_j < chars.len() && chars[new_j].is_whitespace() || chars[new_j] == '{'
                    {
                        new_j += 1;
                    }

                    let (resulting_block, final_index) = extract_block(&chars, new_j);

                    return ParseInfo::new(
                        TokenTypes::While {
                            statement: condition,
                            block: resulting_block,
                        },
                        final_index.try_into().unwrap(),
                        "while".to_string(),
                    );
                }
            }

            j += 1;
        }

        ParseInfo::new(TokenTypes::None, 0, "none".to_string())
    }
}
