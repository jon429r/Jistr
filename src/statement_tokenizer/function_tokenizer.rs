pub mod function_tokenizers {
    use crate::statement_tokenizer::loop_tokenizer::loop_tokenizers::extract_block;
    use crate::statement_tokenizer::tokenizer::tokenizers::ParseInfo;
    use crate::token_type::token_types::TokenTypes;

    fn parse_keyword(expression: &str, index: usize, keyword: &str) -> (bool, Option<usize>) {
        let slice = &expression[index..];
        if slice.starts_with(keyword) {
            (true, Some(index + keyword.len()))
        } else {
            (false, None)
        }
    }

    static mut PARSEFUNCTIONCALL: bool = false;

    pub fn read_function_call(expression: &String, index: usize) -> ParseInfo {
        let mut j = index;
        let mut function_name = String::new();
        let chars: Vec<char> = expression.chars().collect();

        let reserved_chars: Vec<char> = ['+', '-', '*', '/', '!'].into();

        let function_found = parse_keyword(expression, 0, "func");
        if function_found.0 {
            return ParseInfo::new(TokenTypes::None, 0, "none".to_string());
        }

        // Collect the function name
        while j < chars.len() {
            let char = chars[j];
            //let next_char = chars.get(j + 1).cloned().unwrap_or('\0');
            if char == '.' {
                let mut function_call = String::new();
                let mut k = j + 1;
                // loop and collect the function call until we hit ';'
                while k < expression.len() {
                    let char = expression.chars().nth(k).unwrap();
                    if char == ';' {
                        break;
                    }
                    function_call.push(char);
                    k += 1;
                }
                return ParseInfo::new(
                    TokenTypes::Dot {
                        object: (function_name.clone()), //please excuse the naming, it's a bit misleading
                        method: (function_call.clone()),
                    },
                    (k - index).try_into().unwrap(),
                    "Dot Notation".to_string(),
                );
            }

            if char == '=' {
                return ParseInfo::new(TokenTypes::None, 0, "none".to_string());
            }
            if char == '(' {
                // Detected function call: mark it and return info
                unsafe { PARSEFUNCTIONCALL = true };
                return ParseInfo::new(
                    TokenTypes::FunctionCall,
                    (j - index).try_into().unwrap(),
                    function_name.clone(),
                );
            } else if reserved_chars.contains(&char) {
                return ParseInfo::new(TokenTypes::None, 0, "none".to_string());
            } else if !char.is_whitespace() {
                // Build up function name
                function_name.push(char);
            }

            j += 1;
        }

        // Parse the function parameters
        if unsafe { PARSEFUNCTIONCALL } {
            let mut parameter = String::new();
            let mut j = index;

            while j < chars.len() {
                let char = chars[j];
                //let next_char = chars.get(j + 1).cloned().unwrap_or('\0');

                if char == ')' {
                    // End of function call parameters
                    unsafe { PARSEFUNCTIONCALL = false };
                    return ParseInfo::new(
                        TokenTypes::FunctionArguments,
                        (j - index).try_into().unwrap(),
                        parameter.to_string(),
                    );
                } else if char == ',' {
                    // Handle function arguments separated by commas
                    if !parameter.is_empty() {
                        return ParseInfo::new(
                            TokenTypes::FunctionArguments,
                            (j - index).try_into().unwrap(),
                            parameter.clone(),
                        );
                    }
                    parameter.clear();
                } else if !char.is_whitespace() {
                    // Collect parameter characters
                    parameter.push(char);
                } else {
                    parameter.push(char);
                }

                j += 1;
            }
        }

        // Default return if no valid function call or parameters found
        ParseInfo::new(TokenTypes::None, 0, "none".to_string())
    }

    pub fn read_return_statement(expression: &str, index: usize) -> ParseInfo {
        let mut chars = expression.chars().peekable();
        let mut j = index;
        let mut return_statement = String::new();
        let return_value;
        // Check for the `return` keyword
        let return_found = parse_keyword(expression, 0, "return");
        if return_found.0 {
            j = return_found.1.unwrap();
        } else {
            return ParseInfo::new(TokenTypes::None, 0, "none".to_string());
        }
        // Skip "return " keyword
        for _ in 0..6 {
            chars.next();
        }
        // Extract return value
        while let Some(&char) = chars.peek() {
            if char == ';' {
                return_statement.push(';');
                break;
            }
            return_statement.push(char);
            chars.next();
        }
        return_value = return_statement.trim().to_string();
        // Return the parsed result
        ParseInfo::new(
            TokenTypes::ReturnStatement {
                value: return_value.clone(),
            },
            expression.chars().count().try_into().unwrap(),
            return_value,
        )
    }

    pub fn read_function_declaration(expression: &str, index: usize) -> ParseInfo {
        let mut chars = expression.chars().peekable();
        let mut j = index;

        let mut function_name = String::new();
        let mut function_arguments = Vec::new(); // Vec of (name, type, default value)
        let mut return_type = String::new();
        let function_block;

        // Check for the `func` keyword
        let function_found = parse_keyword(expression, 0, "func");
        if function_found.0 {
            j = function_found.1.unwrap();
        } else {
            return ParseInfo::new(TokenTypes::None, 0, "none".to_string());
        }

        // Skip "func " keyword
        for _ in 0..4 {
            chars.next();
        }

        // Extract function name
        while let Some(&char) = chars.peek() {
            if char == '(' {
                chars.next(); // Consume `(`
                break;
            }
            function_name.push(char);
            chars.next();
        }

        // Extract arguments inside parentheses
        let mut between_parentheses = String::new();
        while let Some(&char) = chars.peek() {
            if char == ')' {
                break;
            }
            between_parentheses.push(char);
            chars.next();
        }

        // Split arguments and parse
        let args = between_parentheses
            .split(',')
            .filter(|arg| !arg.trim().is_empty())
            .collect::<Vec<_>>();

        for arg in args {
            // split by : then 1 by =
            let arg_parts = arg.split(':').collect::<Vec<_>>();
            let arg_name = arg_parts[0].to_string();
            let mut arg_value = String::new();
            let arg_type;

            if arg_parts.contains(&"=") {
                let arg_parts_2 = arg_parts[1].split('=').collect::<Vec<_>>();
                arg_type = arg_parts_2[0].to_string();
                arg_value = arg_parts_2[1].to_string();
            } else {
                arg_type = arg_parts[1].to_string();
            }

            function_arguments.push((
                arg_name.trim().to_string(),
                arg_type.trim().to_string(),
                arg_value.trim().to_string(),
            ));
        }

        if expression.contains("->") {
            //isolating the return type between -> and {
            let index = expression.find("->").unwrap();
            return_type = expression[index + 2..].to_string();
            return_type = return_type.split('{').collect::<Vec<_>>()[0].to_string();
        }

        // Parse function block
        let char_vec: Vec<char> = chars.collect();
        while j < char_vec.len() {
            if char_vec[j] == '{' {
                j += 1;
                break;
            }
            j += 1;
        }
        let (block, consumed) = extract_block(&char_vec, j);
        function_block = block;
        j += consumed;

        // Return the parsed result
        ParseInfo::new(
            TokenTypes::Function {
                name: function_name.clone().trim().to_string(),
                arguments: function_arguments,
                return_type: return_type.trim().to_string(),
                block: function_block,
            },
            expression.chars().count().try_into().unwrap(),
            function_name.trim().to_string(),
        )
    }
}
