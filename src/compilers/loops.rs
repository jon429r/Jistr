pub mod loop_compilers {
    use crate::base_variable::{variable, variables::VARIABLE_STACK};

    use crate::base_variable::base_types::BaseTypes;
    use crate::compiler::compilers::route_to_parser;
    use crate::compilers::conditional::conditional_compilers::compile_conditional_statement;
    use crate::compilers::variable::search_for_var_name;
    use crate::node::nodes::match_token_to_node;
    use crate::node::nodes::ASTNode;
    use crate::statement_tokenizer::tokenizer::tokenizers::tokenize;
    use crate::statement_tokenizer::tokenizer::tokenizers::ParseInfo;
    use std::error::Error;

    static mut MAKE_LOOP: bool = false;

    fn set_make_loop(value: bool) {
        unsafe {
            MAKE_LOOP = value;
        }
    }

    pub fn compile_for_loop(expression: &Vec<ASTNode>) -> Result<bool, Box<dyn Error>> {
        for node in expression {
            match node {
                ASTNode::For(fornode) => {
                    // Check if variable exists, else initialize it
                    if !search_for_var_name(fornode.variable.clone()) {
                        let new_var = variable::Variable::new(
                            fornode.variable.clone(),
                            fornode.iterable.0.into(),
                            BaseTypes::Int(0),
                        );
                        unsafe { VARIABLE_STACK.push(new_var) };
                    }

                    // Main iteration logic
                    let mut iter_result = false;
                    for var in unsafe { VARIABLE_STACK.iter_mut() } {
                        if var.name == fornode.variable {
                            let current_value: i32 = var.value.clone().into();
                            if current_value <= fornode.iterable.1 {
                                iter_result = true;
                            }
                        }
                    }

                    if iter_result {
                        // Execute the loop body
                        for stmt in &fornode.block {
                            let tokenized_body = tokenize(stmt.to_string());
                            let mut nodes: Vec<ASTNode> = tokenized_body
                                .into_iter()
                                .map(match_token_to_node)
                                .collect();
                            route_to_parser(&mut nodes, 0.into())?;
                        }
                        for var in unsafe { VARIABLE_STACK.iter_mut() } {
                            if var.name == fornode.variable {
                                var.increment();
                            }
                        }
                    } else {
                        return Ok(false);
                    }
                }
                _ => {
                    // Handle other node types if necessary
                }
            }
        }
        Ok(true)
    }
    pub fn compile_while_loop(expression: &mut Vec<ASTNode>) -> Result<bool, Box<dyn Error>> {
        let mut tokenized: Vec<ParseInfo> = Vec::new();
        let mut index = 0;

        if expression.len() <= 1 {
            return Err("Error: Empty body in while loop.".into());
        }

        while index < expression.len() {
            let node = &expression[index].clone();
            match node {
                ASTNode::While(while_node) => {
                    // Tokenize and evaluate the condition
                    let tokenized_statement = tokenize(while_node.condition.clone());
                    tokenized.extend(tokenized_statement.clone());

                    // Convert tokens to AST nodes
                    let mut condition_nodes: Vec<ASTNode> = tokenized_statement
                        .into_iter()
                        .map(match_token_to_node)
                        .collect();

                    // Evaluate the initial condition
                    let mut evaluation_result: bool = false;
                    match compile_conditional_statement(&mut condition_nodes) {
                        Ok(result) => {
                            evaluation_result = result; // Store the result from the function
                            if !result {
                                return Ok(false); // Exit the loop if the condition is false
                            }
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                    while evaluation_result {
                        set_make_loop(true);

                        //println!("Entering while loop body");

                        // Process the body of the while loop
                        for i in while_node.block.clone() {
                            let tokenized_body = tokenize(i.clone());
                            let mut nodes: Vec<ASTNode> = Vec::new();
                            // Convert to AST nodes
                            for token in tokenized_body {
                                nodes.push(match_token_to_node(token));
                            }
                            let Result = route_to_parser(&mut nodes, 0.into());
                        }

                        // Re-evaluate the while loop condition after each iteration
                        match compile_conditional_statement(&mut condition_nodes.clone()) {
                            Ok(result) => {
                                evaluation_result = result; // Store the result from the function
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }

                        //println!("Condition re-evaluation result: {}", result);

                        if !evaluation_result {
                            return Ok(false); // Exit the loop if the condition is false
                        }
                    }

                    set_make_loop(false);

                    // Increment index to move to the next node after the while
                    index += 1;
                    //println!("Moving to next node after while loop.");
                    continue; // Skip to the next iteration
                }
                _ => {
                    println!("Unhandled node: {:?}", node);
                }
            }
            index += 1; // Move to the next node
        }
        //println!("While loop processing completed.");
        return Ok(true); // Indicate successful processing
    }
}
