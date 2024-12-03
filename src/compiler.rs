/*
    Compiler module for the language
    This module contains the main compiler logic for the language.
    It is responsible for parsing the AST and generating the output code.
*/

pub mod compilers {
    use crate::compilers::collection::*;
    use crate::compilers::conditional::conditional_compilers::compile_if_elif_else_statement;
    use crate::compilers::conditional::conditional_compilers::compile_try_catch_finally;
    use crate::compilers::function::*;
    use crate::compilers::loops::loop_compilers::{compile_for_loop, compile_while_loop};
    use crate::compilers::variable::operation;
    use crate::compilers::variable::{
        compile_dot_statement, compile_variable_call, parse_variable_declaration,
    };
    use crate::globals::{IF_ELSE_SKIP, MAKE_LOOP};
    use crate::node::nodes::{ASTNode, IntNode};
    use std::error::Error;

    pub fn set_make_loop(value: bool) {
        unsafe {
            MAKE_LOOP = value;
        }
    }

    // Custom error type for better error messages
    #[derive(Debug)]
    pub enum CompilerError {
        DivisionByZero,
        UnrecognizedOperator(String),
        InvalidSyntax(String),
    }

    impl std::fmt::Display for CompilerError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                CompilerError::DivisionByZero => write!(f, "Division by zero"),
                CompilerError::UnrecognizedOperator(op) => {
                    write!(f, "Unrecognized operator: {}", op)
                }
                CompilerError::InvalidSyntax(s) => write!(f, "Invalid syntax: {}", s),
            }
        }
    }

    impl Error for CompilerError {}

    // Use Result for proper error handling
    pub fn parse_operator(
        left: &ASTNode,
        operator: &ASTNode,
        right: &ASTNode,
    ) -> Result<ASTNode, Box<dyn Error>> {
        match operator {
            ASTNode::Operator(o) => match o.operator.as_str() {
                "+" => {
                    if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                        let result = left_val.value + right_val.value;
                        return Ok(ASTNode::Int(IntNode { value: result }));
                    }
                }
                "-" => {
                    if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                        let result = left_val.value - right_val.value;
                        return Ok(ASTNode::Int(IntNode { value: result }));
                    }
                }
                "*" => {
                    if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                        let result = left_val.value * right_val.value;
                        return Ok(ASTNode::Int(IntNode { value: result }));
                    }
                }
                "/" => {
                    if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                        if right_val.value != 0 {
                            let result = left_val.value / right_val.value;
                            return Ok(ASTNode::Int(IntNode { value: result }));
                        } else {
                            return Err(Box::new(CompilerError::DivisionByZero));
                        }
                    }
                }
                _ => {
                    return Err(Box::new(CompilerError::UnrecognizedOperator(
                        o.operator.clone(),
                    )))
                }
            },
            _ => {
                return Err(Box::new(CompilerError::InvalidSyntax(
                    "Expected an operator.".to_string(),
                )))
            }
        }
        Ok(ASTNode::None) // Return a neutral node on failure (though this should likely be handled better)
    }

    pub fn route_to_parser(
        expression: &mut Vec<ASTNode>,
        index: Option<usize>,
    ) -> Result<bool, Box<dyn Error>> {
        let mut index = index.unwrap_or(0); // Default to 0 if None

        // Main loop through the expression
        while index < expression.len() {
            let node = &expression[index]; // Access node by index
            let next_node = |expression: &Vec<ASTNode>, index: usize| -> ASTNode {
                if index < expression.len() {
                    return expression[index].clone();
                }
                return ASTNode::None;
            };
            let next_next_node = |expression: &Vec<ASTNode>, index: usize| -> ASTNode {
                if index + 1 < expression.len() {
                    return expression[index + 1].clone();
                }
                return ASTNode::None;
            };

            match node {
                ASTNode::Dot(_d) => {
                    let result = compile_dot_statement(expression);
                    //if result is Ok return true
                    return Ok(true);
                }
                ASTNode::LeftCurly => {}

                ASTNode::If(_i) => {
                    // Call the function and store the result
                    let result = compile_if_elif_else_statement(expression);
                    match result {
                        Ok(true) => {
                            index += 2; // Skip to the next statement after processing `if`
                            unsafe { IF_ELSE_SKIP = true };
                            continue; // Continue with the next iteration of the loop
                        }
                        Ok(false) => {
                            return Ok(false); // Return false if the condition was false
                        }
                        Err(e) => {
                            return Err(e); // Return the error if there was one
                        }
                    }
                }

                ASTNode::Elif(_i) => {
                    let result = compile_if_elif_else_statement(expression);
                    match result {
                        Ok(true) => {
                            index += 2; // Skip to the next statement after processing `elif`
                            unsafe { IF_ELSE_SKIP = true };
                            continue;
                        }
                        Ok(false) => {
                            return Ok(true);
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                ASTNode::For(_f) => {
                    let result = compile_for_loop(expression);
                    match result {
                        Ok(true) => {
                            set_make_loop(true);
                            return Ok(true);
                        }
                        Ok(false) => {
                            set_make_loop(false);

                            return Ok(false);
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                ASTNode::While(_w) => {
                    // Evaluate the condition
                    let condition_result = compile_while_loop(expression);

                    set_make_loop(false);
                    match condition_result {
                        Ok(true) => {
                            return Ok(true);
                        }
                        Ok(false) => {
                            return Ok(true);
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }

                ASTNode::Try(_t) => {
                    // Compile the Try block
                    let result = compile_try_catch_finally(expression, 0)?;

                    //check next for catch
                    match next_node(expression, index + 1) {
                        ASTNode::Catch(_c) => {
                            // Compile the Catch block
                            let result = compile_try_catch_finally(expression, 1)?;
                            match next_next_node(expression, index + 2) {
                                ASTNode::Finally(_f) => {
                                    // Compile the Finally block
                                    let result = compile_try_catch_finally(expression, 2)?;
                                    index += 2;
                                    return Ok(result);
                                }
                                _ => {
                                    return Ok(result);
                                }
                            }
                        }
                        _ => {
                            index += 1;
                            return Ok(result);
                        }
                    }
                }

                ASTNode::Collection(_c) => {
                    let value = parse_collection_declaration(expression);
                    match value {
                        Ok(()) => {
                            return Ok(true);
                        }
                        Err(e) => {
                            return Err(e.into());
                        }
                    }
                }
                ASTNode::Variable(_v) => {
                    let end = parse_variable_declaration(expression);
                    return Ok(end?);
                }
                ASTNode::Else => {}
                ASTNode::Int(_n) => {
                    if expression.len() == 1 {
                        println!("Int: {}", _n.value);
                    } else {
                        let _result = operation(expression)?;
                        break;
                    }
                }
                ASTNode::Function(_f) => {
                    let end = parse_function_declaration(expression);

                    match end {
                        Ok(true) => {
                            return Ok(true);
                        }
                        Ok(false) => {
                            return Ok(false);
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                ASTNode::RightCurly => {}
                ASTNode::String(s) => {
                    println!("String: {}", s.value);
                }
                ASTNode::Char(c) => {
                    println!("Char: {}", c.value);
                }
                ASTNode::FunctionCall(_f) => {
                    let function_expression: Vec<ASTNode> = expression[index..].to_vec();
                    let result = parse_function_call(
                        &function_expression,
                        "None".to_string(),
                        None,
                        None,
                        None,
                    );
                    match result {
                        Ok(_result) => {
                            return Ok(true);
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                ASTNode::VariableCall(_v) => {
                    let call_result = compile_variable_call(expression);
                    return Ok(call_result?);
                }
                ASTNode::Comment(_c) => {
                    return Ok(true);
                }
                ASTNode::LeftParenthesis => {
                    let _value = operation(expression)?;
                    break;
                }
                ASTNode::None => {
                    println!("Invlid Syntax: {:?}", node);
                    let error = "Unhandle node type in compiler";
                    return Err(Box::new(CompilerError::InvalidSyntax(error.into())));
                }
                ASTNode::RightParenthesis => {}
                _ => {
                    println!("Invlid Syntax: {:?}", node);
                    return Err(Box::new(CompilerError::InvalidSyntax(
                        "Unhandled node type in compiler.".to_string(),
                    )));
                }
            }

            index += 1; // Move to the next node
        }

        Ok(true) // Return true when done processing
    }
}

#[cfg(test)]
mod complier_tests {
    use crate::compiler::compilers::parse_operator;
    use crate::node::nodes::{ASTNode, IntNode, OperatorNode};

    use crate::compilers::variable::operation;
    //test parse operator
    #[test]
    fn test_parse_operator_addition() {
        let left = ASTNode::Int(IntNode { value: 5 });
        let operator = ASTNode::Operator(OperatorNode {
            operator: "+".to_string(),
        });
        let right = ASTNode::Int(IntNode { value: 5 });
        let result = parse_operator(&left, &operator, &right);
        match result {
            Ok(ASTNode::Int(n)) => {
                assert_eq!(n.value, 10);
            }
            _ => {
                panic!("Result is not an IntNode");
            }
        }
    }

    #[test]
    fn test_parse_operator_subtraction() {
        let left = ASTNode::Int(IntNode { value: 5 });
        let operator = ASTNode::Operator(OperatorNode {
            operator: "-".to_string(),
        });
        let right = ASTNode::Int(IntNode { value: 5 });
        let result = parse_operator(&left, &operator, &right);
        match result {
            Ok(ASTNode::Int(n)) => {
                assert_eq!(n.value, 0);
            }
            _ => {
                panic!("Result is not an IntNode");
            }
        }
    }

    #[test]
    fn test_parse_operator_multiplication() {
        let left = ASTNode::Int(IntNode { value: 5 });
        let operator = ASTNode::Operator(OperatorNode {
            operator: "*".to_string(),
        });
        let right = ASTNode::Int(IntNode { value: 5 });
        let result = parse_operator(&left, &operator, &right);
        match result {
            Ok(ASTNode::Int(n)) => {
                assert_eq!(n.value, 25);
            }
            _ => {
                panic!("Result is not an IntNode");
            }
        }
    }

    #[test]
    fn test_parse_operator_divition() {
        let left = ASTNode::Int(IntNode { value: 5 });
        let operator = ASTNode::Operator(OperatorNode {
            operator: "/".to_string(),
        });
        let right = ASTNode::Int(IntNode { value: 5 });
        let result = parse_operator(&left, &operator, &right);
        match result {
            Ok(ASTNode::Int(n)) => {
                assert_eq!(n.value, 1);
            }
            _ => {
                panic!("Result is not an IntNode");
            }
        }
    }

    /*
    #[test]
    fn test_operator_unrecognized() {
        let left = ASTNode::Int(IntNode { value: 5 });
        let operator = ASTNode::Operator(OperatorNode {
            operator: "%".to_string(),
        });
        let right = ASTNode::Int(IntNode { value: 5 });
        let result = parse_operator(&left, &operator, &right);
        match result {
            ASTNode::None => {}
            _ => {
                panic!("Result is not None");
            }
        }
    }
    */

    #[test]
    fn test_operation_addition() {
        let mut expression: Vec<ASTNode> = vec![
            ASTNode::Int(IntNode { value: 5 }),
            ASTNode::Operator(OperatorNode {
                operator: "+".to_string(),
            }),
            ASTNode::Int(IntNode { value: 5 }),
        ];
        let result = operation(&mut expression);
        match result {
            Ok(ASTNode::Int(n)) => {
                assert_eq!(n.value, 10);
            }
            _ => {
                panic!("Result is not an IntNode");
            }
        }
    }

    #[test]
    fn test_operation_subtraction() {
        let mut expression: Vec<ASTNode> = vec![
            ASTNode::Int(IntNode { value: 5 }),
            ASTNode::Operator(OperatorNode {
                operator: "-".to_string(),
            }),
            ASTNode::Int(IntNode { value: 5 }),
        ];
        let result = operation(&mut expression);
        match result {
            Ok(ASTNode::Int(n)) => {
                assert_eq!(n.value, 0);
            }
            _ => {
                panic!("Result is not an IntNode");
            }
        }
    }

    #[test]
    fn test_operation_multiplication() {
        let mut expression: Vec<ASTNode> = vec![
            ASTNode::Int(IntNode { value: 5 }),
            ASTNode::Operator(OperatorNode {
                operator: "*".to_string(),
            }),
            ASTNode::Int(IntNode { value: 5 }),
        ];
        let result = operation(&mut expression);
        match result {
            Ok(ASTNode::Int(n)) => {
                assert_eq!(n.value, 25);
            }
            _ => {
                panic!("Result is not an IntNode");
            }
        }
    }

    #[test]
    fn test_operation_divition() {
        let mut expression: Vec<ASTNode> = vec![
            ASTNode::Int(IntNode { value: 5 }),
            ASTNode::Operator(OperatorNode {
                operator: "/".to_string(),
            }),
            ASTNode::Int(IntNode { value: 5 }),
        ];
        let result = operation(&mut expression);
        match result {
            Ok(ASTNode::Int(n)) => {
                assert_eq!(n.value, 1);
            }
            _ => {
                panic!("Result is not an IntNode");
            }
        }
    }
}
