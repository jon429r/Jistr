use crate::base_variable::base_types::StringWrapper;
use crate::base_variable::variable::{self, Variable};
use crate::collection::collections::{Array, Dictionary};
use crate::collection::{ARRAY_FUNCTIONS, DICTIONARY_FUNCTIONS};
use crate::compiler::compilers::route_to_parser;
use crate::compilers::conditional::conditional_compilers::string_to_ast;
use crate::node::nodes::{match_token_to_node, ASTNode};
use std::process::exit;

use crate::compilers::variable::compile_dot_statement;
use crate::function::functions::FunctionTypes;

use crate::base_variable::base_types::BaseTypes;
use crate::base_variable::variables::VARIABLE_STACK;
use crate::compilers::variable::parse_variable_call;
use crate::function::functions::call_function;
use crate::function::functions::Function;
use crate::function::{FUNCTION_STACK, USER_FUNCTION_STACK};
use crate::function_map::{FUNCTIONS, USER_FUNCTIONS};
use std::any::Any;
use std::error::Error;

use crate::statement_tokenizer::tokenizer::tokenizers::tokenize;

/// add the function in the function stack
///
/// params: function_name: &str -> The name of the function to be added
///
/// Returns: None
fn add_to_function_stack(func: Function) {
    USER_FUNCTION_STACK.lock().unwrap().push(func.clone());
}

/// Find the function in the function stack
///
/// params: function_name: &str -> The name of the function to be found
///
/// Returns: Function: Fucntion -> The function found
fn find_function_in_stack(function_name: &str) -> Option<Function> {
    let function_stack = USER_FUNCTION_STACK.lock().unwrap(); // Lock the Mutex, unwrap if the lock is successful

    for function in function_stack.iter() {
        if function_name == function.name {
            return Some(function.clone());
        }
    }
    None
}

fn is_function_in_stack(function_name: &str) -> bool {
    let function_stack = USER_FUNCTION_STACK.lock().unwrap();
    for function in function_stack.iter() {
        if function_name == function.name {
            return true;
        }
    }
    false
}

fn add_to_variable_stack(var: Variable) {
    unsafe { VARIABLE_STACK.clone() }.push(var);
}

fn remove_from_variable_stack(var: Variable) {
    let mut index = 0;
    for arg in unsafe { VARIABLE_STACK.clone() } {
        if arg.name == var.name {
            unsafe { VARIABLE_STACK.clone() }.remove(index);
        }
        index += 1;
    }
}

/// Parse the function declaration
///
/// params: expression: &[ASTNode] -> The expression to be parsed
///
/// Returns: Result<bool, Box<dyn Error>> -> The result of the parsing
pub fn parse_function_declaration(expression: &[ASTNode]) -> Result<bool, Box<dyn Error>> {
    let function_name: String;
    let mut parameters: Vec<(String, String, String)>;
    let mut function_return_type: String;
    let mut function_body: Vec<String>;

    match expression[0].clone() {
        ASTNode::Function(func) => {
            function_name = func.name.clone();
            function_return_type = func.return_type.clone();
            parameters = func.arguments.clone();
            function_body = func.block.clone();
        }
        _ => {
            return Err("Syntax Error: Function declaration must start with a function".into());
        }
    }

    // turn function body into vec of ast nodes
    let mut function_body_nodes: Vec<ASTNode> = Vec::new();
    for line in function_body {
        let result = tokenize(line);
        for node in result {
            function_body_nodes.push(match_token_to_node(node));
        }
    }

    // make arguments into vec of vars
    let mut args: Vec<Variable> = Vec::new();
    for arg in parameters.iter() {
        let var = Variable {
            name: arg.0.clone(),
            value: BaseTypes::Null,
            var_type: arg.1.clone().into(),
        };
        args.push(var);
    }

    use crate::function::functions::Function;
    let function = Function {
        name: function_name.clone(),
        return_type: function_return_type.clone().into(),
        arguments: args.clone(),
        body: function_body_nodes.clone(),
    };

    // add function to user function stack
    add_to_function_stack(function);

    Ok(true)
}

/// Parse the function call includes dot notation
///
/// Params: expression: &Vec<ASTNode> -> The expression to be parsed
/// Params: dot_notation: String -> The dot notation
/// Params: array: Option<Array> -> The array
///
/// Returns: Result<BaseTypes, Box<dyn Error>> -> The result of the parsing
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
                    ASTNode::VariableCall(_) => {
                        // get variable value
                        let var_value = parse_variable_call(&expression[i]);
                        parameter_and_value.push(var_value?.1);
                    }
                    ASTNode::Int(n) => {
                        let _arg1 = (String::new(), BaseTypes::Int(n.value));
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
                Ok(result) => Ok(result),
                Err(e) => Err(e),
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
                Ok(result) => Ok(result),
                Err(e) => Err(e),
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
                Ok(result) => Ok(result),
                Err(e) => Err(e),
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

/// Get the function result
///
/// params: function_name: String -> The name of the function
/// params: parameter_and_value: &mut Vec<BaseTypes> -> The parameters and values
/// params: dot_notation: String -> The dot notation -> either dictionary, array or variable else
/// no dot notation
/// params: array: Option<Array> -> The array if dot call
/// params: dictionary: Option<Dictionary> -> The dictionary if dot call
/// params: variable: Option<Variable> -> The variable if dot call
///
/// Returns: Result<BaseTypes, Box<dyn Error>> -> The result of the function
pub fn get_function_result(
    function_name: String,
    parameter_and_value: &mut Vec<BaseTypes>,
    dot_notation: String,
    array: Option<Array>,
    dictionary: Option<Dictionary>,
    _variable: Option<Variable>,
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

    if let Some(func) = find_function_in_stack(function_name.as_str()) {
        // initalize params as vars
        for arg in func.arguments.clone() {
            add_to_variable_stack(arg)
        }
        let result: BaseTypes;

        for line in func.body {
            match line {
                ASTNode::Return(r) => {
                    println!("value {}", r.value);
                    let mut line = string_to_ast(r.value);
                    let result = route_to_parser(&mut line, 0.into());
                    println!("result {:?}", result);
                }
                _ => println!("line {}", line),
            }
        }

        // clean up vars after running
        for arg in func.arguments {
            remove_from_variable_stack(arg)
        }

        let result = 0.into();
        return Ok(result);
    }

    // Handle standard functions
    if let Some(func) = std_functions.get(&function_name.as_str()) {
        let result = call_standard_function(func, parameter_and_value)?;
        return Ok(result);
    }

    println!(
        "Function call {} is not in any of the registered functions.",
        function_name
    );
    Err("Function not found".into())
}

/// Adjust parameter types to floats if needed to match function signatures
///
/// params: parameter_and_value: &mut Vec<BaseTypes> -> The parameters and values
///
/// Returns: None
fn adjust_parameter_types(parameter_and_value: &mut Vec<BaseTypes>) {
    for param in parameter_and_value.iter_mut().take(2) {
        if let BaseTypes::Int(x) = *param {
            *param = BaseTypes::Float(x as f64);
        }
    }
}

/// Call the function with parameters includes dot calls
///
/// params: func: &FunctionTypes -> The function to be called
/// params: array: Option<Array> -> The array if dot call
/// params: dict: Option<Dictionary> -> The dictionary if dot call
/// params: parameter_and_value: &mut Vec<BaseTypes> -> The parameters and values
///
/// returns: Result<BaseTypes, Box<dyn Error>> -> The result of the function
fn call_function_with_params(
    func: &FunctionTypes,
    array: Option<Array>,
    dict: Option<Dictionary>,
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

/// Parse the function call arguments
///
/// params: expression: &[ASTNode] -> The expression to be parsed
///
/// returns: Result<Vec<BaseTypes>, Box<dyn Error>> -> The result of the parsing
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
                let result = tokenize(a.value.clone());
                let mut output = Vec::new();
                for node in result {
                    output.push(match_token_to_node(node));
                }
                let variable = parse_variable_call(&output[0]);
                arguments.push(variable?.1);
            }
            _ => {
                let error = format!("Unhandled node in arguments: {:?}", expression[i]);
                return Err(error.into());
            }
        }
        i += 1;
    }

    // Return the collected arguments
    //println!("@@@@@@@@@@@Arguments: {:?}", arguments);
    Ok(arguments)
}
