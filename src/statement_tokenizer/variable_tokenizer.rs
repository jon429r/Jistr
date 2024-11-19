pub mod variable_tokenizers {
    use crate::base_variable::variables::VARIABLE_STACK;
    use crate::statement_tokenizer::tokenizer::tokenizers::ParseInfo;
    use crate::token_type::token_types::TokenTypes;

    static mut VARIABLE_DECLARATION: bool = false;
    pub fn read_variable_declaration(expression: &String, index: usize) -> ParseInfo {
        let mut j = index;
        let let_compare = "let";
        let mut variable: String = String::new();
        let mut variable_name = String::new();

        // Collect characters to check for the "let" keyword
        while j < expression.len() {
            if let Some(char) = expression.chars().nth(j) {
                if char.is_alphabetic() {
                    variable.push(char);
                } else {
                    break;
                }
            }
            j += 1;

            // Check if the collected string equals "let"
            if variable.len() == 3 && variable == let_compare {
                // Reset variable to start collecting the actual variable name
                variable.clear();
                while j < expression.len() {
                    if let Some(char) = expression.chars().nth(j) {
                        if char.is_alphabetic() || char == '_' {
                            variable_name.push(char);
                        } else if char == ':' {
                            // Found the type declaration, break to parse the type
                            break;
                        } else if !char.is_whitespace() {
                            // If it's neither whitespace nor ':', exit
                            return ParseInfo::new(TokenTypes::None, 0, "none".to_string());
                        }
                    }
                    j += 1;
                }

                // Check for ':' and potentially a collection type
                while j < expression.len() {
                    if expression.chars().nth(j) == Some(':') {
                        j += 1; // Move past ':'
                                // Skip whitespace after ':'
                        while j < expression.len()
                            && expression.chars().nth(j).unwrap().is_whitespace()
                        {
                            j += 1;
                        }

                        // Check for '<' indicating a collection
                        if expression.chars().nth(j) == Some('<') {
                            println!(
                                "Invalid variable declaration for collections: {}",
                                variable_name
                            );
                            // Return None for invalid declaration
                            return ParseInfo::new(TokenTypes::None, 0, "none".to_string());
                        } else {
                            // Handle normal variable type (no collection)
                            let type_declaration_start = j;
                            while j < expression.len()
                                && expression.chars().nth(j).unwrap().is_alphabetic()
                            {
                                j += 1;
                            }

                            let next_char = expression.chars().nth(j).unwrap_or('\0');
                            let type_declaration = &expression[type_declaration_start..j];

                            if type_declaration.contains('<') || next_char == '<' {
                                return ParseInfo::new(TokenTypes::None, 0, "none".to_string());
                            }

                            unsafe {
                                VARIABLE_DECLARATION = true; // Update global state
                            }

                            println!(
                                /*"Variable Name: '{}', Type Declaration: '{}'",
                                variable_name, type_declaration */
                            );
                            return ParseInfo::new(
                                TokenTypes::Variable,
                                variable_name.len().try_into().unwrap(),
                                variable_name,
                            );
                        }
                    }
                    j += 1; // Continue scanning
                }
                // Return None if we reach here without valid declarations
                return ParseInfo::new(TokenTypes::None, 0, "none".to_string());
            }
            // Remove the first character if more than 3 characters are collected
            if variable.len() > 3 {
                variable.remove(0);
            }
        }

        // Return default ParseInfo if function not found
        ParseInfo::new(TokenTypes::None, 0, "none".to_string())
    }

    pub fn read_variable_assignment(expression: &String, index: usize) -> ParseInfo {
        let chars: Vec<char> = expression.chars().collect();
        let mut j = index;
        let original_index = index;

        // Look for the `:` pattern
        while j < chars.len() {
            let char = chars[j];

            if char == ':' {
                j += 1; // Move past `:`

                // Skip whitespace after `:`
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }

                // Collect the type (assuming it is lowercase or an identifier)
                let mut var_type = String::new();
                while j < chars.len() {
                    let char = chars[j];
                    if char.is_lowercase() || char.is_alphanumeric() || char == '_' {
                        var_type.push(char);
                        j += 1;
                    } else {
                        break;
                    }
                }

                // Skip whitespace after the type and check for `=`
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }

                if j < chars.len() && chars[j] == '=' {
                    // Print for debugging
                    /*
                        print!(

                            "Variable Type: '{}', Length: {}\n",
                            var_type,
                            j - original_index
                        );
                    */
                    unsafe { VARIABLE_DECLARATION = false };
                    return ParseInfo::new(
                        TokenTypes::VarTypeAssignment,
                        (j - original_index).try_into().unwrap(),
                        var_type,
                    );
                }
            } else {
                j += 1; // Continue scanning if `:` not found
            }
        }

        // Return None if no valid variable assignment found
        unsafe { VARIABLE_DECLARATION = false };
        ParseInfo::new(TokenTypes::None, 0, "none".to_string())
    }

    pub fn read_variable_call(expression: &String, index: usize) -> ParseInfo {
        let mut j = index;
        let mut variable_name = String::new();

        // Iterate over characters in the string starting at 'index'
        let chars = expression.chars().skip(index).enumerate();

        for (_i, char) in chars {
            // Check if the current or next character is '=' (assignment)
            let next_char = expression.chars().nth(j + 1).unwrap_or('\0');
            // Check if its a dot for dot notation then grab the function call as well as a string
            if char == '.' || next_char == '.' {
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
                        object: (variable_name.clone()),
                        method: (function_call.clone()),
                    },
                    (k - index).try_into().unwrap(),
                    "Dot Notation".to_string(),
                );
            } else if char == '(' || next_char == '(' {
                return ParseInfo::new(
                    TokenTypes::FunctionCall,
                    (j - index).try_into().unwrap(),
                    variable_name,
                );
            } else if char == '=' || next_char == '=' {
                return ParseInfo::new(
                    TokenTypes::VariableCall,
                    (j - index).try_into().unwrap(),
                    variable_name,
                );
            }

            // Collect valid variable name characters (alphanumeric and '_')

            if char.is_alphanumeric() || char == '_' {
                variable_name.push(char);
            } else if char.is_whitespace() {
                continue;
            } else {
                break;
            }

            j += 1;
        }

        // After collecting the variable name, check if it exists in the stack
        if !variable_name.is_empty() {
            for variable in unsafe { &VARIABLE_STACK } {
                if variable.name == variable_name {
                    return ParseInfo::new(
                        TokenTypes::VariableCall,
                        (j - index).try_into().unwrap(),
                        variable_name,
                    );
                } else {
                    print!("Variable '{}' not found in the stack", variable_name);
                }
            }
        }

        // Return None if no valid variable call found
        ParseInfo::new(
            TokenTypes::None,
            0,
            "No valid variable call found".to_string(),
        )
    }
}
