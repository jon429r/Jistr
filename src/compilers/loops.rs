pub mod loop_compilers {
    use crate::compiler::compilers::route_to_parser;
    use crate::compilers::conditional::conditional_compilers::compile_conditional_statement;
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
        // check the condition and run
        // for i in 0..10 {}
        // set i to 0 iterate until 10
        // condtion is i < 10
        // for i in 4..10 {}
        // set i to 4 iterate until 10
        // i is var call, may need to be declared, in token range: { range }
        //

        let mut tokenized: Vec<ParseInfo> = Vec::new();
        let mut index = 0;

        while index < expression.len() {
            let node = &expression[index];
            match node {
                ASTNode::For(ifnode) => {
                    let tokenized_statement = tokenize(ifnode.condition.clone());

                    tokenized.extend(tokenized_statement.clone());
                    let mut nodes: Vec<ASTNode> = Vec::new();
                    // convert to ast nodes
                    for token in tokenized_statement {
                        nodes.push(match_token_to_node(token));
                    }

                    // call the operation function or make custom function for conditional operations
                    let result = compile_conditional_statement(&mut nodes);
                    match result {
                        Ok(result) => {
                            return Ok(result);
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                ASTNode::Else => {}
                _ => {}
            }
            index += 1;
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
