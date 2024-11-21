use std::process::exit;

use crate::base_variable::base_types::BaseTypes;
use crate::base_variable::variable::Variable;
use crate::base_variable::variables::VARIABLE_STACK;
use crate::collection::collections::{Array, Dictionary};
use crate::compilers::collection::parse_collection_call;
use crate::compilers::function::parse_function_call;
use crate::node::nodes::match_token_to_node;
use crate::node::nodes::ASTNode;
use crate::node::nodes::VariableCallNode;
use crate::node::nodes::{IntNode, OperatorNode};
use crate::statement_tokenizer::tokenizer::tokenizers::tokenize;
use std::error::Error;

use crate::collection::{ARRAY_STACK, DICTIONARY_STACK};

use super::conditional::conditional_compilers::compile_conditional_statement;

pub fn search_for_dict_name(name: String) -> bool {
    // if name is in DICTIONARY_STACK reutrn true
    let dict_stack = DICTIONARY_STACK.lock().unwrap(); // Lock the mutex
    for dict in dict_stack.iter() {
        if dict.name == name {
            return true;
        }
    }
    false
}

pub fn search_for_array_name(name: String) -> bool {
    let array_stack = ARRAY_STACK.lock().unwrap();
    for array in array_stack.iter() {
        if array.name == name {
            return true;
        }
    }
    false
}

pub fn search_for_var_name(name: String) -> bool {
    let var_stack = unsafe { VARIABLE_STACK.clone() };
    for var in var_stack.iter() {
        if var.name == name {
            return true;
        }
    }
    false
}

pub fn get_dict(name: String) -> Result<Dictionary, Box<dyn Error>> {
    let dict_stack = DICTIONARY_STACK.lock().unwrap();
    for dict in dict_stack.iter() {
        if dict.name == name {
            return Ok(dict.clone());
        }
    }
    return Err("Dictionary not found".into());
}

pub fn get_array(name: String) -> Result<Array, Box<dyn Error>> {
    let array_stack = ARRAY_STACK.lock().unwrap();
    for array in array_stack.iter() {
        if array.name == name {
            return Ok(array.clone());
        }
    }
    return Err("Array not found".into());
}

pub fn parse_object_call(node: &ASTNode) -> Result<(String, String), Box<dyn Error>> {
    // will look at object call name and see if it is in variable stack or collection stack then
    // return the approiate ast node, eg variable call or collection call

    match node.clone() {
        ASTNode::Dot(c) => {
            let object_name = c.object.clone();
            let dict: bool = search_for_dict_name(object_name.clone());
            let array: bool = search_for_array_name(object_name.clone());
            let variable: bool = false;

            if dict {
                return Ok((object_name, "dictionary".to_string()));
            }
            if array {
                return Ok((object_name, "array".to_string()));
            }
            if variable {
                return Ok((object_name, "variable".to_string()));
            } else {
                return Err("Object not found".into());
            }
        }
        _ => {
            return Err("Error expected an object call node".into());
        }
    }
}

pub fn compile_dot_statement(exp_stack: &mut Vec<ASTNode>) -> Result<BaseTypes, Box<dyn Error>> {
    //println!("compiling dot statement");

    let mut result: BaseTypes = BaseTypes::Null;
    // in order to compile we must, tokenize variable or collection call, tokenize function call
    // check what type object is
    // then make an object type specific function call

    let node: ASTNode = exp_stack.get(0).unwrap().clone();

    match node.clone() {
        ASTNode::Dot(d) => {
            //println!("Dot call\nobject: {}, function: {}", d.object, d.function);
            let variable: (String, BaseTypes);
            let collection: (String, Vec<BaseTypes>);
            let object_name_type: (String, String);

            // first tokenize object
            let objects = tokenize(d.object);
            let mut object_nodes: Vec<ASTNode> = Vec::new();
            //now match to node
            for object in objects.clone() {
                object_nodes.push(match_token_to_node(object));
            }

            // check type of object, var, collection? then comiple
            match object_nodes.get(0).unwrap() {
                ASTNode::ObjectCall(c) => {
                    match parse_object_call(&node) {
                        Ok(result) => {
                            object_name_type = result;
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }

                    //determine object type
                    //get that from stack, then call approiate function

                    match object_name_type.1.to_string().as_str() {
                        "dictionary" => {
                            //println!("Object call is a dictionary");
                            let dict: Option<Dictionary> = get_dict(object_name_type.0)?.into();
                            let tokenized_function = tokenize(d.function.clone());
                            let mut function_nodes: Vec<ASTNode> = Vec::new();
                            for function in tokenized_function {
                                function_nodes.push(match_token_to_node(function));
                            }
                            //println!("Function nodes: {:?}", function_nodes);
                            let func_result = parse_function_call(
                                &function_nodes,
                                "dictionary".to_string(),
                                None,
                                dict,
                                None,
                            );

                            result = func_result.unwrap();
                        }
                        "array" => {
                            //println!("Object call is an array");
                            let array: Option<Array> = get_array(object_name_type.0)?.into();
                            let tokenized_function = tokenize(d.function.clone());
                            let mut function_nodes: Vec<ASTNode> = Vec::new();
                            for function in tokenized_function {
                                function_nodes.push(match_token_to_node(function));
                            }
                            //println!("Function nodes: {:?}", function_nodes);

                            //make sure array is not None

                            match array.clone() {
                                Some(Array {
                                    name,
                                    data,
                                    value_type,
                                }) => {
                                    //println!(
                                    //    "Array is not None with name: {}, and data type: {}",
                                    //    name, value_type
                                    //);
                                    // You can use name, data, value_type as needed
                                }
                                None => {
                                    return Err("Array not found".into());
                                }
                            }

                            let func_result = parse_function_call(
                                &function_nodes,
                                "array".to_string(),
                                array,
                                None,
                                None,
                            );

                            result = func_result.unwrap();
                        }
                        "variable" => {
                            println!("Object call is a variable");
                            let tokenized_function = tokenize(d.function.clone());
                            let mut function_nodes: Vec<ASTNode> = Vec::new();
                            for function in tokenized_function {
                                function_nodes.push(match_token_to_node(function));
                            }
                            //println!("Function nodes: {:?}", function_nodes);
                        }
                        _ => {
                            println!("Object call is not a variable, dictionary or array, therefore cannot compile dot call");
                            exit(1);
                        }
                    }
                }
                ASTNode::Collection(_c) => {
                    collection = parse_collection_call(&object_nodes).unwrap()
                }
                ASTNode::VariableCall(_c) => {
                    variable = parse_variable_call(object_nodes.get(0).unwrap())?
                }
                _ => {
                    println!("Unexpected node found within comple dot statement function");
                    exit(1);
                }
            }
        }
        _ => {
            return Err("Syntax Error: Expected a dot statement".into());
        }
    }

    Ok(result)
}

///
///This Function takes in an ASTNode and returns a tuple of the variable name and its value
///
pub fn parse_variable_call(node: &ASTNode) -> Result<(String, BaseTypes), Box<dyn Error>> {
    match node {
        ASTNode::VariableCall(v) => {
            //println!("Function argument: {}", v.name);
            let mut _arg_name = ASTNode::VariableCall(VariableCallNode {
                name: v.name.clone(),
            });
            //var stack for var with this name
            let mut arg1_value = BaseTypes::StringWrapper(String::new()); // Initialize with default value
            let mut arg1_name = String::new(); // Initialize with default value
            for var in unsafe { VARIABLE_STACK.iter() } {
                if var.name == v.name {
                    arg1_value = var.value.clone();
                    //print!("Value: {:?}", arg1_value);
                    arg1_name = var.name.clone();
                }
            }
            let arg1 = (arg1_name, arg1_value);
            //parameter_and_value.push(arg1);
            Ok(arg1)
        }
        _ => Err("Syntax Error: Expected a variable call.".into()),
    }
}

///
/// This function is called when the first token in the expression is a variable
/// can be used to set the value of the variable to something else
///

pub fn compile_variable_call(exp_stack: &mut Vec<ASTNode>) -> Result<bool, Box<dyn Error>> {
    // Ensure there is at least one node in the stack
    if exp_stack.is_empty() {
        return Err("Empty expression stack.".into());
    }

    // Find the first variable call node
    let index = exp_stack
        .iter()
        .position(|node| matches!(node, ASTNode::VariableCall(_)))
        .unwrap_or_else(|| {
            println!("Syntax Error: Expected a variable call.");
            exit(1);
        });

    let first_node = &exp_stack[index];

    // Match against the first node to see if it is a variable call
    if let ASTNode::VariableCall(v) = first_node {
        let mut variable = Variable::new(String::new(), BaseTypes::Null, BaseTypes::Null);

        // Search for the variable in the global VARIABLE_STACK
        for var in unsafe { &VARIABLE_STACK.clone() } {
            if var.name == v.name {
                variable = var.clone();
                /*println!(
                    "Variable found: Name = {}, Value = {:?}, Type = {:?}",
                    variable.name, variable.value, variable.var_type
                );*/
                break; // Break after finding the variable
            }
        }

        // If variable wasn't found, return false
        if variable.name.is_empty() {
            println!("Syntax Error: Variable '{}' not found in stack.", v.name);
            return Err("Variable not found.".into());
        }

        // Process further if there are additional nodes in exp_stack
        for node in &exp_stack[index + 1..] {
            match node {
                ASTNode::AssignmentOperator(_) => {
                    let mut operation_stack = exp_stack[index + 2..].to_vec();
                    println!("Operation stack: {:?}", operation_stack);

                    if operation_stack.is_empty() {
                        return Err(
                            "Syntax Error: Expected an expression after the assignment operator."
                                .into(),
                        );
                    } else if operation_stack.len() == 1 {
                        let value: BaseTypes = operation_stack[0].clone().into();
                        variable.set_value(value.clone());
                        return Ok(true);
                    }

                    match operation_stack.first() {
                        Some(ASTNode::FunctionCall(_)) => {
                            let result = parse_function_call(
                                &operation_stack,
                                "None".to_string(),
                                None,
                                None,
                                None,
                            );
                            let value = result.unwrap();
                            variable.set_value(value.clone());
                            return Ok(true);
                        }
                        Some(ASTNode::Dot(_)) => {
                            let result = compile_dot_statement(&mut operation_stack);
                            let value = result.unwrap();
                            variable.set_value(value.clone());
                            return Ok(true);
                        }
                        Some(ASTNode::VariableCall(_)) => {
                            let result = parse_variable_call(&operation_stack[0]);
                            let value = result.unwrap().1;
                            variable.set_value(value.clone());
                            return Ok(true);
                        }
                        _ => {}
                    }

                    let result = operation(&mut operation_stack)?;
                    // Set the value to the variable based on the operation result
                    println!("result: {:?}", result);
                    variable.set_value(result.clone());
                    if let Some(next_node) = operation_stack.first() {
                        let _value: BaseTypes = next_node.into();
                        print!("result.unwrap(): {:?}", result);
                        variable.set_value(result.clone());
                    }
                    return Ok(true);
                }
                ASTNode::Operator(o) => match o.operator.as_str() {
                    "++" => {
                        variable.increment();
                        return Ok(true);
                    }
                    "--" => {
                        variable.decrement();
                        return Ok(true);
                    }
                    "<=" => {
                        let result = compile_conditional_statement(exp_stack);
                        return Ok(result.is_ok());
                    }
                    _ => {
                        let error: String =
                            format!("Syntax Error: Unrecognized operator '{}'", o.operator);
                        return Err(error.into());
                    }
                },
                _ => {
                    // Handle additional argument processing here
                    let value: BaseTypes = node.into();
                    variable.set_value(value.clone());
                    println!("Set variable value to: {:?}", value);
                }
            }
        }

        Ok(true) // Successfully processed the variable call
    } else {
        let error: String = format!(
            "Syntax Error: Expected a variable call but found {:?}",
            first_node
        );
        Err(error.into())
    }
}

///
///This function takes in a mutable reference to a vector of ASTNodes and parses the variable
///declaration returning end after parsing the variable declaration
///
pub fn parse_variable_declaration(exp_stack: &mut Vec<ASTNode>) -> Result<bool, Box<dyn Error>> {
    let mut var_name: Option<String> = None;
    let mut var_type: Option<BaseTypes> = None;
    let mut assignment_operator: Option<String> = None;
    let mut inside_assignment = false;
    let mut value: BaseTypes = BaseTypes::Null;
    let mut var_value = ASTNode::Int(IntNode { value: 0 });
    let mut first: Option<ASTNode> = Option::None;
    let mut parenthesis: bool = false;
    let mut variable_call_values: Vec<(String, BaseTypes)> = Vec::new();

    let mut index = 0;
    while index < exp_stack.len() {
        let node = &exp_stack[index];
        match node {
            ASTNode::Variable(v) => {
                var_name = Some(v.value.clone());
            }
            ASTNode::VariableType(v) => {
                var_type = match v.value.as_str() {
                    "int" => Some(BaseTypes::Int(0)),
                    "float" => Some(BaseTypes::Float(0.0)),
                    "string" => Some(BaseTypes::StringWrapper(String::new())),
                    "boolean" => Some(BaseTypes::Bool(false)),
                    "char" => Some(BaseTypes::Char('\0')),
                    _ => {
                        let error: String =
                            format!("Syntax Error: Unrecognized type '{}'", v.value);
                        return Err(error.into());
                    }
                };
            }
            ASTNode::AssignmentOperator(a) => {
                assignment_operator = Some(a.operator.clone());
                inside_assignment = true;
            }
            ASTNode::Dot(_) => {
                if inside_assignment {
                    let result = compile_dot_statement(exp_stack);
                    value = result.unwrap();
                    break;
                }
            }
            ASTNode::FunctionCall(_c) => {
                if inside_assignment {
                    /*println!(
                                            "Function call found within variable declaration. Expression: {:?}",
                                            exp_stack
                                        );
                    */
                    let mut function_call_stack = exp_stack.clone();
                    function_call_stack.reverse();
                    function_call_stack.pop();
                    function_call_stack.pop();
                    function_call_stack.pop();
                    function_call_stack.reverse();

                    //println!("Function call stack: {:?}", function_call_stack);
                    //

                    let result = parse_function_call(
                        &function_call_stack,
                        "None".to_string(),
                        None,
                        None,
                        None,
                    );

                    // check with match then set value to result
                    value = result.unwrap();

                    /*println!(
                                            "New variable = name: {}, value: {:?}, type: {:?}",
                                            var_name.clone().unwrap(),
                                            value,
                                            var_type.clone().unwrap()
                                        );
                    */

                    let variable = Variable::new(var_name.unwrap(), value, var_type.unwrap());
                    // add to VARIABLE_STACK
                    unsafe {
                        VARIABLE_STACK.push(variable.clone());
                    }
                    return Ok(true);
                }
            }
            ASTNode::VariableCall(_c) => {
                if inside_assignment {
                    let mut result = parse_variable_call(node);
                    result = match result {
                        Ok(result) => Ok(result),
                        Err(e) => Err(e),
                    };
                    //add the value back into the epression at same index

                    let var_result = result?;

                    exp_stack.insert(
                        index,
                        ASTNode::VariableCall(VariableCallNode {
                            name: var_result.clone().0,
                        }),
                    );
                    variable_call_values.push(var_result);
                } else {
                    let error: String =
                        "Syntax Error: Variable call outside of assignment.".to_string();
                    return Err(error.into());
                }
            }
            ASTNode::Int(_n) => {
                if inside_assignment {
                    var_value = operation(exp_stack)?;
                    if let ASTNode::Int(n) = var_value {
                        value = BaseTypes::Int(n.value);
                    } else {
                        let result: String = "Expected an integer after the operator.".to_string();
                        return Err(result.into());
                    }
                    break;
                }
            }
            //include other types: float, string, bool, char
            ASTNode::Float(f) => {
                if inside_assignment {
                    first = Some(ASTNode::Float(f.clone()));
                    var_value = operation(exp_stack)?;
                    if let ASTNode::Float(f) = var_value {
                        value = BaseTypes::Float(f.value.into());
                    } else {
                        return Err("Expected a float after the operator.".into());
                    }
                    break;
                }
            }
            ASTNode::String(s) => {
                if inside_assignment {
                    //print!("String: {}", s.value);
                    first = Some(ASTNode::String(s.clone()));
                    var_value = operation(exp_stack)?;
                    if let ASTNode::String(s) = var_value {
                        value = BaseTypes::StringWrapper(s.value.clone());
                    } else {
                        return Err("Expected a string after the assignment_operator.".into());
                    }
                    break;
                }
            }
            ASTNode::Bool(b) => {
                if inside_assignment {
                    value = BaseTypes::Bool(b.value);
                } else {
                    return Err("Syntax Error: Bool outside of assignment.".into());
                }
            }
            ASTNode::Char(c) => {
                if inside_assignment {
                    value = BaseTypes::Char(c.value);
                } else {
                    return Err("Syntax Error: Char outside of assignment.".into());
                }
            }
            ASTNode::LeftParenthesis => {
                parenthesis = true;
            }
            ASTNode::RightParenthesis => {}

            _ => {
                let error: String = format!(
                    "Syntax Error: Unhandled node in variable declaration: {:?}",
                    node
                );
                return Err(error.into());
            }
        }
        index += 1;
    }

    if var_name.is_none() || var_type.is_none() || assignment_operator.is_none() {
        return Err("Missing variable components.".into());
    }

    let variable = Variable::new(var_name.unwrap(), value, var_type.unwrap());
    // Add to VARIABLE_STACK
    unsafe {
        VARIABLE_STACK.push(variable.clone());
    }
    //println!("New variable: {:?}", variable.clone());
    Ok(true)
}

///
///This Function is in the variable module and takes in a mut ref of a vector of ASTNodes
///It returns a BaseType result of the parsed expression
///
pub fn operation(exp_stack: &mut Vec<ASTNode>) -> Result<ASTNode, Box<dyn Error>> {
    //cloned_exp_stack.reverse();
    //println!("Cloned stack after reversed: {:?}", cloned_exp_stack);

    //pop 2 elements from the stack
    let mut first_node: Option<ASTNode> = None;
    let mut parenthesis: bool = false;

    let mut first: bool = false;
    let mut operator: ASTNode = ASTNode::Operator(OperatorNode {
        operator: String::new(),
    });
    let mut left: ASTNode = ASTNode::Int(IntNode { value: 0 });
    let _right: ASTNode = ASTNode::Int(IntNode { value: 0 });

    while let Some(top) = exp_stack.pop() {
        match top {
            ASTNode::AssignmentOperator(o) => {
                operator = ASTNode::AssignmentOperator(o);
            }

            ASTNode::Operator(o) => {
                operator = ASTNode::Operator(o);
            }
            ASTNode::Int(n) => {
                if !first {
                    first = true;
                    left = ASTNode::Int(n);
                } else {
                    let right = ASTNode::Int(n);
                    first_node = Some(parse_operator(&left, &operator, &right)?);
                }
            }
            ASTNode::Float(f) => {
                if !first {
                    first = true;
                    left = ASTNode::Float(f);
                } else {
                    let right = ASTNode::Float(f);
                    first_node = Some(parse_operator(&left, &operator, &right)?);
                }
            }
            ASTNode::String(s) => {
                if !first {
                    first = true;
                    left = ASTNode::String(s);
                } else {
                    let right = ASTNode::String(s);
                    first_node = Some(parse_operator(&left, &operator, &right)?);
                }
            }
            ASTNode::Bool(b) => {
                if !first {
                    first = true;
                    left = ASTNode::Bool(b);
                } else {
                    let right = ASTNode::Bool(b);
                    first_node = Some(parse_operator(&left, &operator, &right)?);
                }
            }
            ASTNode::Char(c) => {
                if !first {
                    first = true;
                    left = ASTNode::Char(c);
                } else {
                    let right = ASTNode::Char(c);
                    first_node = Some(parse_operator(&left, &operator, &right)?);
                }
            }

            ASTNode::RightParenthesis => {
                if parenthesis {
                    break;
                }
            }
            ASTNode::LeftParenthesis => {
                parenthesis = true;
            }
            _ => {
                exp_stack.reverse();
                exp_stack.pop();
                exp_stack.reverse();
            }
        }
    }

    if first_node.is_none() {
        Ok(left)
    } else {
        Ok(first_node.unwrap())
    }
}

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
                    let result = IntNode { value: result };
                    return Ok(ASTNode::Int(result));
                }
            }
            "=" => {
                if let (ASTNode::Int(_left_val), ASTNode::Int(right_val)) = (left, right) {
                    let result = right_val.value;
                    let result = IntNode { value: result };
                    return Ok(ASTNode::Int(result));
                }
            }
            "-" => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    let result = right_val.value - left_val.value;
                    let result = IntNode { value: result };
                    return Ok(ASTNode::Int(result));
                }
            }
            "*" => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    let result = left_val.value * right_val.value;
                    let result = IntNode { value: result };
                    return Ok(ASTNode::Int(result));
                }
            }
            "/" => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    if right_val.value != 0 {
                        let result = right_val.value / left_val.value;
                        let result = IntNode { value: result };
                        return Ok(ASTNode::Int(result));
                    } else {
                        return Err("Division by zero.".into());
                    }
                }
            }
            ".." => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    let result = left_val.value..right_val.value;
                    let result = IntNode {
                        value: result.start,
                    };
                    return Ok(ASTNode::Int(result));
                }
            }

            "==" => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    let result = left_val.value == right_val.value;
                    let result = IntNode {
                        value: result as i32,
                    };
                    return Ok(ASTNode::Int(result));
                }
            }
            "!=" => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    let result = left_val.value != right_val.value;
                    let result = IntNode {
                        value: result as i32,
                    };
                    return Ok(ASTNode::Int(result));
                }
            }
            ">" => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    let result = left_val.value > right_val.value;
                    let result = IntNode {
                        value: result as i32,
                    };
                    return Ok(ASTNode::Int(result));
                }
            }
            "<" => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    let result = left_val.value < right_val.value;
                    let result = IntNode {
                        value: result as i32,
                    };
                    return Ok(ASTNode::Int(result));
                }
            }
            ">=" => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    let result = left_val.value >= right_val.value;
                    let result = IntNode {
                        value: result as i32,
                    };
                    return Ok(ASTNode::Int(result));
                }
            }
            "<=" => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    let result = left_val.value <= right_val.value;
                    let result = IntNode {
                        value: result as i32,
                    };
                    return Ok(ASTNode::Int(result));
                }
            }
            "&&" => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    let result = left_val.value != 0 && right_val.value != 0;
                    let result = IntNode {
                        value: result as i32,
                    };
                    return Ok(ASTNode::Int(result));
                }
            }
            "||" => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    let result = left_val.value != 0 || right_val.value != 0;
                    let result = IntNode {
                        value: result as i32,
                    };
                    return Ok(ASTNode::Int(result));
                }
            }
            "!" => {
                if let ASTNode::Int(left_val) = left {
                    let result = left_val.value == 0;
                    let result = IntNode {
                        value: result as i32,
                    };
                    return Ok(ASTNode::Int(result));
                }
            }

            _ => {
                let error: String = format!("Syntax Error: Unrecognized operator '{}'", o.operator);
                return Err(error.into());
            }
        },
        _ => {
            println!("Warning: Expected an operator found None returning false.");
            return Ok(ASTNode::Int(IntNode { value: 0 }));
            //let error: String = format!("Syntax Error: Expected an operator found {}.", operator);
            //return Err(error.into());
        }
    }
    Err("Syntax Error: Invalid operation.".into())
}
