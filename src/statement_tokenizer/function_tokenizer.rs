pub mod function_tokenizers {
    use crate::statement_tokenizer::tokenizer::tokenizers::ParseInfo;
    use crate::token_type::token_types::TokenTypes;

    pub fn read_function_assignment(expression: &String, index: usize) -> ParseInfo {
        let chars: Vec<char> = expression.chars().collect();
        let mut j = index;

        while j < chars.len() {
            let char = chars[j];
            let nextchar = if j + 1 < chars.len() {
                chars[j + 1]
            } else {
                '\0' // Null character for out-of-bounds safety
            };

            if char == '-' && nextchar == '>' {
                j += 2; // Move past `->`

                // Skip whitespace after `->`
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }

                // Collect the return type (lowercase type)
                let mut return_type = String::new();
                while j < chars.len() {
                    let char = chars[j];
                    if char.is_lowercase() || char == '_' {
                        return_type.push(char);
                        j += 1;
                    } else {
                        break;
                    }
                }

                // Check if we collected a valid return type
                if !return_type.is_empty() {
                    return ParseInfo::new(
                        TokenTypes::ReturnTypeAssignment,
                        (j - index).try_into().unwrap(),
                        return_type,
                    );
                }
            } else {
                j += 1; // Continue scanning if `->` not found
            }
        }
        ParseInfo::new(TokenTypes::None, 0, "none".to_string())
    }

    static mut PARSEFUNCTIONCALL: bool = false;

    pub fn read_function_call(expression: &String, index: usize) -> ParseInfo {
        let mut j = index;
        let mut function_name = String::new();
        let chars: Vec<char> = expression.chars().collect();

        let reserved_chars: Vec<char> = ['+', '-', '*', '/', '!'].into();

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

    pub fn read_function_declaration(expression: &String, index: usize) -> ParseInfo {
        let mut j = index;
        let mut function: String = String::new();
        let func_compare = "func";

        let mut function_name = String::new();
        let mut function_arguments: Vec<(String, String, String)> = Vec::new(); // Tuples for argument name and type
        let mut return_type = String::new();

        // Collect alphabetic characters forming the function name
        while j < expression.len() {
            if let Some(char) = expression.chars().nth(j) {
                if char.is_alphabetic() {
                    function.push(char);
                } else {
                    break;
                }
            }
            j += 1;

            // If we've collected 4 characters, check if they form "func"
            if function.len() == 4 {
                if function == func_compare {
                    // Skip whitespace and collect the actual function name
                    while j < expression.len() {
                        if let Some(char) = expression.chars().nth(j) {
                            if char.is_alphabetic() {
                                function_name.push(char);
                            } else if char == '(' {
                                break; // End of function name, start of arguments
                            } else if char != ' ' {
                                return ParseInfo::new(
                                    TokenTypes::None,
                                    (j - index).try_into().unwrap(),
                                    "Invalid function declaration".to_string(),
                                );
                            }
                        }
                        j += 1;
                    }

                    if let Some('(') = expression.chars().nth(j) {
                        j += 1; // Skip the '('
                        let mut arg_name = String::new();
                        let mut arg_type = String::new();
                        let mut arg_value = String::from("null"); // Default value set to "null"
                        let mut is_type = false;
                        let mut is_value = false;

                        while j < expression.len() {
                            if let Some(char) = expression.chars().nth(j) {
                                if char == ')' {
                                    if !arg_name.is_empty() && !arg_type.is_empty() {
                                        // Push the argument with its name, type, and value
                                        function_arguments.push((
                                            arg_name.clone(),
                                            arg_type.clone(),
                                            arg_value.clone(),
                                        ));
                                    }
                                    break; // End of arguments
                                } else if char == ':' {
                                    is_type = true; // Start collecting type
                                    is_value = false; // Ensure we stop collecting value
                                } else if char == '=' {
                                    is_value = true; // Start collecting default value
                                } else if char == ',' {
                                    // End of an argument, push collected data
                                    if !arg_name.is_empty() && !arg_type.is_empty() {
                                        function_arguments.push((
                                            arg_name.clone(),
                                            arg_type.clone(),
                                            arg_value.clone(),
                                        ));
                                    }
                                    // Clear the buffers for the next argument
                                    arg_name.clear();
                                    arg_type.clear();
                                    arg_value = String::from("null"); // Reset default value to "null"
                                    is_type = false;
                                    is_value = false;
                                } else if char.is_alphanumeric() || char == '_' {
                                    // Collect the argument name, type, or value based on flags
                                    if is_value {
                                        arg_value.push(char);
                                    } else if is_type {
                                        arg_type.push(char);
                                    } else {
                                        arg_name.push(char);
                                    }
                                }
                            }
                            j += 1;
                        }
                    }

                    // Look for the return type after the `->`
                    while j < expression.len() {
                        if let Some(char) = expression.chars().nth(j) {
                            if char == '-' && expression.chars().nth(j + 1) == Some('>') {
                                j += 2; // Skip `->`
                                while j < expression.len()
                                    && expression.chars().nth(j).unwrap().is_whitespace()
                                {
                                    j += 1; // Skip whitespace
                                }
                                while j < expression.len() {
                                    if let Some(char) = expression.chars().nth(j) {
                                        if char.is_alphanumeric() || char == '_' {
                                            return_type.push(char);
                                        } else {
                                            break;
                                        }
                                    }
                                    j += 1;
                                }
                                break;
                            }
                        }
                        j += 1;
                    }

                    // Return ParseInfo with the parsed function details
                    return ParseInfo::new(
                        TokenTypes::Function {
                            name: function_name.clone(),
                            arguments: function_arguments.clone(),
                            return_type: return_type.clone(),
                        },
                        (j - index).try_into().unwrap(),
                        function_name.to_string(),
                    );
                } else {
                    return ParseInfo::new(
                        TokenTypes::None,
                        (j - index).try_into().unwrap(),
                        "none".to_string(),
                    );
                }
            }

            // If more than 4 characters, remove the first character (sliding window)
            if function.len() > 4 {
                function.remove(0);
            }
        }

        // Return default ParseInfo if function not found
        ParseInfo::new(TokenTypes::None, 0, "none".to_string())
    }
}
