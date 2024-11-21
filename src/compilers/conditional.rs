pub mod conditional_compilers {
    use std::error::Error;

    use crate::base_variable::base_types::BaseTypes;
    use crate::compilers::function::parse_function_call;
    use crate::compilers::variable::compile_dot_statement;
    use crate::compilers::variable::parse_operator;
    use crate::compilers::variable::parse_variable_call;
    use crate::node::nodes::from_base_type;
    use crate::node::nodes::match_token_to_node;
    use crate::node::nodes::to_base_type;
    use crate::node::nodes::ASTNode;
    use crate::statement_tokenizer::tokenizer::tokenizers::tokenize;
    use crate::statement_tokenizer::tokenizer::tokenizers::ParseInfo;

    /// Compiles a conditional statement
    /// returns a boolean value or an error
    ///
    /// params: expression: &mut Vec<ASTNode> -> The expression to be compiled
    ///
    /// returns: Result<bool, Box<dyn Error>> -> true if the conditional statement is true and false if not
    pub fn compile_conditional_statement(
        expression: &mut Vec<ASTNode>,
    ) -> Result<bool, Box<dyn Error>> {
        let mut index = 0;
        let mut first_value: ASTNode = ASTNode::None;
        let mut operation: ASTNode = ASTNode::None;
        let mut second_value: ASTNode = ASTNode::None;

        while index < expression.len() {
            let node = &expression[index];
            match node {
                ASTNode::Dot(_) => {
                    //put node into a vec and pass it to compile_dot_statement
                    let mut vec_node: Vec<ASTNode> = Vec::new();
                    vec_node.push(node.clone());
                    let value = compile_dot_statement(&mut vec_node);

                    first_value = if first_value == ASTNode::None {
                        from_base_type(value?)
                    } else {
                        first_value
                    };
                }
                ASTNode::VariableCall(_) => {
                    if let Ok((_, value)) = parse_variable_call(node) {
                        first_value = if first_value == ASTNode::None {
                            from_base_type(value)
                        } else {
                            first_value
                        };
                    } else {
                        return Err("Error: Unable to parse variable call".into());
                    }
                }

                ASTNode::FunctionCall(_) => {
                    let mut function_call: Vec<ASTNode> = Vec::new();
                    while index < expression.len() {
                        match &expression[index] {
                            ASTNode::RightParenthesis => {
                                function_call.push(expression[index].clone());
                                break;
                            }
                            _ => function_call.push(expression[index].clone()),
                        }
                        index += 1;
                    }

                    if first_value == ASTNode::None {
                        let value = parse_function_call(
                            &function_call,
                            "None".to_string(),
                            None,
                            None,
                            None,
                        );
                        first_value = from_base_type(value.unwrap());
                    } else {
                        let value = parse_function_call(
                            &function_call,
                            "None".to_string(),
                            None,
                            None,
                            None,
                        );
                        second_value = from_base_type(value.unwrap());
                    }
                }

                ASTNode::Operator(o) => {
                    operation = ASTNode::Operator(o.clone());
                }

                ASTNode::Int(n) => {
                    if first_value == ASTNode::None {
                        first_value = ASTNode::Int(n.clone());
                    } else {
                        second_value = ASTNode::Int(n.clone());
                    }
                }

                ASTNode::Float(n) => {
                    if first_value == ASTNode::None {
                        first_value = ASTNode::Float(n.clone());
                    } else {
                        second_value = ASTNode::Float(n.clone());
                    }
                }

                ASTNode::String(n) => {
                    if first_value == ASTNode::None {
                        first_value = ASTNode::String(n.clone());
                    } else {
                        second_value = ASTNode::String(n.clone());
                    }
                }

                ASTNode::Char(c) => {
                    if first_value == ASTNode::None {
                        first_value = ASTNode::Char(c.clone());
                    } else {
                        second_value = ASTNode::Char(c.clone());
                    }
                }

                ASTNode::Bool(b) => {
                    if first_value == ASTNode::None {
                        first_value = ASTNode::Bool(b.clone());
                    } else {
                        second_value = ASTNode::Bool(b.clone());
                    }
                }

                _ => {}
            }
            index += 1;
        }

        let ast_result = parse_operator(&first_value, &operation, &second_value)
            .map_err(|e| format!("Error:\n Unable to parse operator:\n {}", e))?;

        // Safely convert AST result to BaseTypes, with error handling
        let base_result: BaseTypes = match to_base_type(&ast_result) {
            Some(result) => result,
            None => {
                return Err("Error: Unable to convert to BaseTypes".into());
            }
        };

        // Convert BaseTypes result into i32 and return true/false
        let result: i32 = base_result.into();
        let bool_result = result == 1;
        Ok(bool_result)
    }

    /// Compiles an if/elif/else statements
    ///
    ///params: expression: &mut Vec<ASTNode> -> The expression to be compiled
    ///
    ///returns: Result<bool, Box<dyn Error>> -> true if the conditional statement is true else false
    pub fn compile_if_elif_else_statement(
        expression: &mut Vec<ASTNode>,
    ) -> Result<bool, Box<dyn Error>> {
        let mut tokenized: Vec<ParseInfo> = Vec::new();
        let index = 0;

        let node = &expression[index];
        match node {
            ASTNode::If(ifnode) => {
                let tokenized_statement = tokenize(ifnode.condition.clone());

                tokenized.extend(tokenized_statement.clone());
                let mut nodes: Vec<ASTNode> = Vec::new();
                // convert to ast nodes
                for token in tokenized_statement {
                    nodes.push(match_token_to_node(token));
                }

                // call the operation function or make custom function for conditional operations
                let result = compile_conditional_statement(&mut nodes);

                //if result is true or false, return the result
                //if result is an error, return the error
                match result {
                    Ok(result) => Ok(result),
                    Err(e) => Err(e),
                }
            }
            ASTNode::Elif(elifnode) => {
                let tokenized_statement = tokenize(elifnode.condition.clone());
                tokenized.extend(tokenized_statement.clone());
                let mut nodes: Vec<ASTNode> = Vec::new();
                // convert to ast nodes
                for token in tokenized_statement {
                    nodes.push(match_token_to_node(token));
                } // call the operation function or make custom function for conditional operations
                let result = compile_conditional_statement(&mut nodes);
                match result {
                    Ok(result) => Ok(result),
                    Err(e) => Err(e),
                }
            }
            ASTNode::Else => {
                // if there is an else statement, return true
                Ok(true)
            }
            _ => Err("Error: Invalid statement".into()),
        }
    }
}
