use crate::base_variable::variable::Variable;
use crate::collection::collections::{Array, Dictionary};
use crate::collection::{ARRAY_FUNCTIONS, DICTIONARY_FUNCTIONS};
use crate::node::nodes::{match_token_to_node, ASTNode};
use std::process::exit;

use crate::compilers::variable::compile_dot_statement;
use crate::function::functions::FunctionTypes;

use crate::base_variable::base_types::BaseTypes;
use crate::base_variable::variables::VARIABLE_STACK;
use crate::compilers::variable::parse_variable_call;
use crate::function::functions::call_function;
use crate::function::functions::Function;
use crate::function::FUNCTION_STACK;
use crate::function_map::FUNCTIONS;
use std::any::Any;
use std::error::Error;

use crate::node::nodes::to_base_type;
use crate::statement_tokenizer::tokenizer::tokenizers::tokenize;

fn add_to_function_stack(func: Function) {
    FUNCTION_STACK.lock().unwrap().push(func);
    //USER_FUNCTIONS.lock().unwrap().push(func);
    // You can still use `dict` after this line because we cloned it
    //println!("dict pushed to stack")
}

fn find_function_in_stack(function_name: &str) -> Function {
    let function_stack = FUNCTION_STACK.lock().unwrap(); // Lock the Mutex, unwrap if the lock is successful

    for function in function_stack.iter() {
        if function_name == function.name {
            return function.clone();
        }
    }

    eprintln!("Function not in user functions");
    exit(1);
}

pub fn parse_function_declaration(expression: &[ASTNode]) -> Result<bool, Box<dyn Error>> {
    let mut function_name: String = String::new();
    let mut parameters: Vec<Variable> = Vec::new();
    let mut return_type: BaseTypes = BaseTypes::Null;
    let mut function_body: Vec<ASTNode> = Vec::new();

    let mut i = 0;
    while i < expression.len() {
        match &expression[i] {
            ASTNode::Function(f) => {
                // Store function name and return type
                function_name = f.name.clone();
                return_type = BaseTypes::StringWrapper(f.return_type.clone());

                for arg in &f.arguments {
                    let var_type = match arg.2.clone().as_str() {
                        "int" => BaseTypes::Int(0),
                        "float" => BaseTypes::Float(0.0),
                        "string" => BaseTypes::StringWrapper(String::new()),
                        "boolean" => BaseTypes::Bool(false),
                        "char" => BaseTypes::Char('\0'),
                        "null" => BaseTypes::Null,
                        _ => {
                            return Err(
                                "Syntax Error: Unrecognized type in function declaration".into()
                            );
                        }
                    };

                    let var_value = match arg.2.clone().as_str() {
                        "int" => arg
                            .1
                            .parse::<i32>()
                            .map(BaseTypes::Int)
                            .unwrap_or(BaseTypes::Null),
                        "float" => arg
                            .1
                            .parse::<f64>()
                            .map(BaseTypes::Float)
                            .unwrap_or(BaseTypes::Null),
                        "string" => BaseTypes::StringWrapper(arg.1.clone()),
                        "boolean" => arg
                            .1
                            .parse::<bool>()
                            .map(BaseTypes::Bool)
                            .unwrap_or(BaseTypes::Null),
                        "char" => {
                            if let Some(first_char) = arg.1.chars().next() {
                                BaseTypes::Char(first_char)
                            } else {
                                BaseTypes::Null // Handle empty char case
                            }
                        }
                        "null" => BaseTypes::Null,
                        _ => {
                            return Err("Unrecognized type in function declaration".into());
                            // Exit if an unrecognized type is found
                        }
                    };

                    // Create the variable and add it to the parameters
                    let var = Variable::new(
                        arg.0.clone(), // Variable name
                        var_type,      // Variable type
                        var_value,     // Variable value
                    );
                    parameters.push(var);
                }
            }
            ASTNode::LeftCurly => {
                // Now we need to store the function body
                function_body.clear(); // Clear any previous function body
                i += 1; // Move to the next node after '{'

                // Collect nodes until we reach the matching right curly brace
                let mut curly_brace_count = 1; // We've encountered one '{'

                while i < expression.len() {
                    match &expression[i] {
                        ASTNode::LeftCurly => curly_brace_count += 1,
                        ASTNode::RightCurly => {
                            curly_brace_count -= 1;
                            if curly_brace_count == 0 {
                                break; // Found matching '}'
                            }
                        }
                        _ => {}
                    }
                    function_body.push(expression[i].clone());
                    i += 1;
                }

                // After collecting the function body, create the Function object
                let function = Function::new(
                    function_name.clone(),
                    return_type.clone(),
                    parameters.clone(),
                    function_body.clone(),
                );
                // add to FUNCTION_STACK
                println!("Function: {}", function);
                add_to_function_stack(function);
            }
            _ => println!("Unhandled node: {:?}", expression[i]),
        }
        i += 1;
    }
    // Placeholder return value, should likely be more meaningful
    return Ok(true);
}

pub fn parse_function_call(
    expression: &Vec<ASTNode>,
    dot_notation: String,
    array: Option<Array>,
    dictionary: Option<Dictionary>,
    variable: Option<Variable>,
) -> Result<BaseTypes, Box<dyn Error>> {
    let mut function_name: String = "None".to_string();
    let mut parameter_and_value: Vec<BaseTypes> = Vec::new();
    let mut i = 0;

    match expression.get(i).unwrap() {
        ASTNode::FunctionCall(f) => {
            function_name = f.name.clone();
            i += 1;
            while i < expression.len() {
                match &expression[i] {
                    ASTNode::FunctionCallArguments(_) => {
                        parameter_and_value = parse_function_call_arguments(&expression[i + 1..])?;
                    }
                    ASTNode::RightParenthesis => {}
                    ASTNode::LeftParenthesis => {
                        parameter_and_value = parse_function_call_arguments(&expression[i + 1..])?;
                        break;
                    }
                    ASTNode::VariableCall(v) => {
                        // get variable value
                        let var_value = parse_variable_call(&expression[i]);
                        parameter_and_value.push(var_value?.1);
                    }
                    ASTNode::Int(n) => {
                        let arg1 = (String::new(), BaseTypes::Int(n.value.clone()));
                        parameter_and_value.push(n.value.into());
                    }

                    _ => return Err("Unhandled node in function call: ".into()),
                }
                i += 1;
            }
        }
        _ => println!("Unhandled node: {:?}", expression[i]),
    }

    match dot_notation.as_str() {
        "dictionary" => {
            let result = get_function_result(
                function_name,
                &mut parameter_and_value,
                dot_notation,
                None,
                dictionary,
                None,
            );
            match result {
                Ok(result) => return Ok(result),
                Err(e) => return Err(e),
            }
        }
        "array" => {
            let result = get_function_result(
                function_name,
                &mut parameter_and_value,
                dot_notation,
                array,
                None,
                None,
            );
            match result {
                Ok(result) => return Ok(result),
                Err(e) => return Err(e),
            }
        }
        "variable" => {
            let result = get_function_result(
                function_name,
                &mut parameter_and_value,
                dot_notation,
                None,
                None,
                variable,
            );
            match result {
                Ok(result) => return Ok(result),
                Err(e) => return Err(e),
            }
        }
        "None" => {
            let result = get_function_result(
                function_name,
                &mut parameter_and_value,
                dot_notation,
                None,
                None,
                None,
            );
            match result {
                Ok(result) => {
                    return Ok(result);
                }
                Err(e) => return Err(e),
            }
        }
        _ => {
            return Err("Unknown dot notation object type".into());
        }
    }
}

pub fn get_function_result(
    function_name: String,
    parameter_and_value: &mut Vec<BaseTypes>,
    dot_notation: String,
    array: Option<Array>,
    dictionary: Option<Dictionary>,
    variable: Option<Variable>,
) -> Result<BaseTypes, Box<dyn Error>> {
    let std_functions = FUNCTIONS
        .lock()
        .map_err(|_| "Failed to lock FUNCTIONS mutex")?;
    let array_functions = ARRAY_FUNCTIONS
        .lock()
        .map_err(|_| "Failed to lock ARRAY_FUNCTIONS mutex")?;
    let dictionary_functions = DICTIONARY_FUNCTIONS
        .lock()
        .map_err(|_| "Failed to lock DICTIONARY_FUNCTIONS mutex")?;
    // Adjust parameter types
    adjust_parameter_types(parameter_and_value);

    match dot_notation.as_str() {
        "dictionary" => {
            let func: &FunctionTypes = dictionary_functions.get(&function_name.as_str()).unwrap();
            let result =
                call_function_with_params(func, None, dictionary.clone(), parameter_and_value)?;
            return Ok(result);
        }
        "array" => {
            let func: &FunctionTypes = array_functions.get(&function_name.as_str()).unwrap();
            let result = call_function_with_params(func, array.clone(), None, parameter_and_value)?;
            return Ok(result);
        }
        "variable" => {
            // Handle variable logic if needed
        }
        _ => {}
    }

    // Handle standard functions
    if let Some(func) = std_functions.get(&function_name.as_str()) {
        let result = call_standard_function(func, parameter_and_value)?;
        return Ok(result);
    }

    println!("Function call is not in any of the registered functions.");
    Err("Function not found".into())
}

fn adjust_parameter_types(parameter_and_value: &mut Vec<BaseTypes>) {
    for param in parameter_and_value.iter_mut().take(2) {
        if let BaseTypes::Int(x) = *param {
            *param = BaseTypes::Float(x as f64);
        }
    }
}

fn call_function_with_params(
    func: &FunctionTypes,     // Adjust type as needed
    array: Option<Array>,     // Adjust type as needed
    dict: Option<Dictionary>, // Adjust type as needed
    parameter_and_value: &mut Vec<BaseTypes>,
) -> Result<BaseTypes, Box<dyn Error>> {
    let mut params: Vec<Box<dyn Any>> = Vec::new();

    if let Some(collection_param) = array {
        params.push(Box::new(collection_param));
    }

    if let Some(collection_param) = dict {
        params.push(Box::new(collection_param));
    }

    for param in parameter_and_value {
        let boxed_param: Box<dyn Any> = match param {
            BaseTypes::Int(x) => Box::new(*x),
            BaseTypes::Float(x) => Box::new(*x),
            BaseTypes::StringWrapper(x) => Box::new(x.clone()),
            BaseTypes::Bool(x) => Box::new(*x),
            BaseTypes::Char(x) => Box::new(*x),
            _ => {
                let error = format!("Unknown parameter type: {:?}", param);
                return Err(error.into());
            }
        };
        params.push(boxed_param);
    }

    let result = call_function(func, params);

    if let Some(value) = result.downcast_ref::<i32>() {
        Ok(BaseTypes::Int(*value))
    } else if let Some(value) = result.downcast_ref::<f64>() {
        Ok(BaseTypes::Float(*value))
    } else if let Some(value) = result.downcast_ref::<String>() {
        Ok(BaseTypes::StringWrapper(value.clone()))
    } else if let Some(value) = result.downcast_ref::<bool>() {
        Ok(BaseTypes::Bool(*value))
    } else if let Some(value) = result.downcast_ref::<char>() {
        Ok(BaseTypes::Char(*value))
    } else {
        Ok(BaseTypes::Null)
    }
}

fn call_standard_function(
    func: &FunctionTypes,
    parameter_and_value: &mut Vec<BaseTypes>,
) -> Result<BaseTypes, Box<dyn Error>> {
    let mut params: Vec<Box<dyn Any>> = Vec::new();

    for param in parameter_and_value {
        let boxed_param: Box<dyn Any> = match param {
            BaseTypes::Int(x) => Box::new(*x),
            BaseTypes::Float(x) => Box::new(*x),
            BaseTypes::StringWrapper(x) => Box::new(x.clone()),
            BaseTypes::Bool(x) => Box::new(*x),
            BaseTypes::Char(x) => Box::new(*x),
            _ => return Err("Unknown parameter type".into()),
        };

        params.push(boxed_param);
    }

    let result = call_function(func, params);

    if let Some(value) = result.downcast_ref::<i32>() {
        Ok(BaseTypes::Int(*value))
    } else if let Some(value) = result.downcast_ref::<f64>() {
        Ok(BaseTypes::Float(*value))
    } else if let Some(value) = result.downcast_ref::<String>() {
        Ok(BaseTypes::StringWrapper(value.clone()))
    } else if let Some(value) = result.downcast_ref::<bool>() {
        Ok(BaseTypes::Bool(*value))
    } else if let Some(value) = result.downcast_ref::<char>() {
        Ok(BaseTypes::Char(*value))
    } else {
        Ok(BaseTypes::Null)
    }
}

fn parse_function_call_arguments(expression: &[ASTNode]) -> Result<Vec<BaseTypes>, Box<dyn Error>> {
    let mut arguments: Vec<BaseTypes> = Vec::new();
    let mut i = 0;

    while i < expression.len() {
        match &expression[i] {
            //process do notation calls
            ASTNode::Dot(_d) => {
                let mut vec: Vec<ASTNode> = expression[i..].to_vec();
                let result = compile_dot_statement(&mut vec);

                arguments.push(result?);
            }
            ASTNode::VariableCall(v) => {
                // Process variable call, you could push its value from a variable store
                // For now, let's assume variables are stored in VARIABLE_STACK and extract their values
                for var in unsafe { VARIABLE_STACK.iter() } {
                    if var.name == v.name {
                        arguments.push(var.value.clone());
                    }
                }
            }
            ASTNode::Int(n) => {
                // Handle integer argument
                arguments.push(BaseTypes::Int(n.value));
            }
            ASTNode::Float(f) => {
                // Handle float argument
                arguments.push(BaseTypes::Float(f.value.into()));
            }
            ASTNode::String(s) => {
                // Handle string argument
                arguments.push(BaseTypes::StringWrapper(s.value.clone()));
            }
            ASTNode::Bool(b) => {
                // Handle boolean argument
                arguments.push(BaseTypes::Bool(b.value));
            }
            ASTNode::Char(c) => {
                // Handle char argument
                arguments.push(BaseTypes::Char(c.value));
            }
            ASTNode::ArgumentSeparator => {
                // Simply skip over argument separators (commas)
            }
            ASTNode::RightParenthesis => {
                // End of arguments, break out of the loop
                break;
            }
            ASTNode::FunctionArguments(a) => {
                // Process function arguments
                //call tokenizer
                let mut result = tokenize(a.value.clone());
                let mut output = Vec::new();
                for node in result {
                    output.push(match_token_to_node(node));
                }
                let variable = parse_variable_call(&output[0]);
                arguments.push(variable?.1);
            }
            _ => {
                return Err("Unhandled node in arguments".into());
            }
        }
        i += 1;
    }

    // Return the collected arguments
    //println!("@@@@@@@@@@@Arguments: {:?}", arguments);
    return Ok(arguments);
}
