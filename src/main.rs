mod ast;
pub mod base_variable;
mod collection;
pub mod compiler;
mod function_map;
mod node;
pub mod token_type;
//mod tokenizer;

mod compilers {
    pub mod collection;
    pub mod function;
    pub mod operation;
    pub mod variable;
}

mod statement_tokenizer {
    pub mod basic_tokenizer;
    pub mod collection_tokenizer;
    pub mod function_tokenizer;
    pub mod tests;
    pub mod tokenizer;
    pub mod variable_tokenizer;
}

use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::exit;

//use crate::collection::collections::{Array, Dictionary};
use base_variable::variables::VARIABLE_STACK;
use compiler::compilers::route_to_parser;
use node::nodes::match_token_to_node;
use node::nodes::ASTNode;
use statement_tokenizer::tokenizer::tokenizers::tokenize;

use crate::collection::{ARRAY_STACK, DICTIONARY_STACK};
//use lazy_static::lazy_static;
//use std::sync::Mutex;

///
/// This function checks if the file extension is valid. IE: .jist
///
fn check_file_extension(file_path: String) -> Result<bool, Box<dyn Error>> {
    let ext = Path::new(&file_path).extension().and_then(OsStr::to_str);
    let valid_ext = "jist";
    if ext == Some(valid_ext) {
        Ok(true)
    } else {
        Err("Invalid file extension".into())
    }
}

fn print_array_stack() {
    let array_stack = ARRAY_STACK.lock().unwrap(); // Lock the mutex
    for array in array_stack.iter() {
        println!("{}", array); // Now we can iterate over the Vec
    }
}

fn print_dictionary_stack() {
    let dict_stack = DICTIONARY_STACK.lock().unwrap(); // Lock the mutex
    for dict in dict_stack.iter() {
        println!("{}", dict); // Now we can iterate over the Vec
    }
}

fn parse_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    let mut brace_count = 0;
    let mut bracket_count = 0;
    let mut current_line = String::new();
    let mut number_of_lines = 0;
    let mut finished_lines: Vec<String> = Vec::new();

    for line in &lines {
        // No need to clone `lines`, just borrow
        for ch in line.chars() {
            match ch {
                '{' => {
                    brace_count += 1;
                    current_line.push(ch);
                }
                '}' => {
                    brace_count -= 1;
                    current_line.push(ch);
                    if brace_count < 0 {
                        return Err("Unmatched closing curly brace".into());
                    }
                }
                '[' => {
                    bracket_count += 1;
                    current_line.push(ch);
                }
                ']' => {
                    bracket_count -= 1;
                    current_line.push(ch);
                    if bracket_count < 0 {
                        return Err("Unmatched closing square bracket".into());
                    }
                }
                ';' => {
                    if brace_count == 0 && bracket_count == 0 {
                        current_line.push(ch);
                        println!("Finished statement: {}", current_line.trim());
                        finished_lines.push(current_line.clone());
                        current_line.clear();
                        number_of_lines += 1;
                    } else {
                        current_line.push(ch);
                    }
                }
                _ => current_line.push(ch),
            }
        }
    }

    // Tokenization and AST generation
    let _ast_nodes: Vec<ASTNode> = Vec::new();

    //println!("Number of lines: {}", finished_lines.len());

    for line in finished_lines {
        let tokens = tokenize(line);
        //println!("Tokens: {:?}", tokens);

        let mut hasroot = false;
        let mut tokenized_expression = Vec::new();
        for parsed_info in tokens {
            let node = match_token_to_node(parsed_info);
            match node {
                ASTNode::SemiColon => {
                    if !hasroot {
                        println!("Syntax error: expression must be more than a semicolon");
                        exit(1);
                    } else {
                        route_to_parser(&mut tokenized_expression);
                    }
                }
                _ => {
                    hasroot = true;
                    tokenized_expression.push(node);
                }
            }
        }
    }

    // Check for unmatched braces and brackets
    if brace_count > 0 {
        return Err("Unmatched opening curly brace".into());
    }
    if bracket_count > 0 {
        return Err("Unmatched opening square bracket".into());
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("No file path provided".into());
    }
    let file_path = &args[1];
    match check_file_extension(file_path.to_owned()) {
        Ok(true) => {
            //println!("File path is valid");
        }
        Err(_) => {
            return Err("File path not valid: Does not have extension .jist".into());
        }
        _ => {
            return Err("Some error occurred".into());
        }
    }

    // Read the file contents
    //let contents = fs::read_to_string(file_path)?;
    //println!("{}", contents);
    //
    //curly braces take priority, then square braces then ;
    //keep track of braces and make sure all are closed before finishing line and check for ; as
    //next,
    //if none go until ;
    if let Err(e) = parse_file(file_path) {
        eprintln!("Failed to parse file: {}", e);
    }

    // Tokenize each line and collect AST nodes

    // Display the collected AST nodes
    /*
    for node in &ast_nodes {
        let indent = " ".repeat(4);
        match node {
            ASTNode::Bool(b) => println!("{}BoolNode: Value: {}", indent, b.value),
            ASTNode::Variable(v) => println!("{}VariableNode: Type: {}, Value: {}", indent, v.var_type, v.value),
            ASTNode::Int(n) => println!("{}IntNode: Value: {}", indent, n.value),
            ASTNode::Operator(o) => println!("{}OperatorNode: Operator: {}", indent, o.operator),
            ASTNode::Function(f) => println!("{}FunctionNode: Name: {}", indent, f.name),
            ASTNode::String(s) => println!("{}StringNode: Value: {}", indent, s.value),
            ASTNode::Char(c) => println!("{}CharNode: Value: {}", indent, c.value),
            ASTNode::Assignment(a) => println!("{}AssignmentNode: Value: {}", indent, a.value),
            ASTNode::VarTypeAssignment(v) => println!("{}VarTypeAssignmentNode: Value: {}", indent, v.value),
            ASTNode::FunctionCall(f) => println!("{}FunctionCallNode: Value: {}", indent, f.name),
            ASTNode::VariableCall(v) => println!("{}VariableCallNode: Value: {}", indent, v.name),
            ASTNode::VariableType(v) => println!("{}VariableTypeNode: Value: {}", indent, v.value),
            ASTNode::VariableValue(v) => println!("{}VariableValueNode: Value: {}", indent, v.value),
            ASTNode::FunctionArguments(f) => println!("{}FunctionArgumentsNode: Value: {}", indent, f.value),
            ASTNode::AssignmentOperator(a) => println!("{}AssignmentOperatorNode: Value: {}", indent, a.operator),
            ASTNode::ReturnTypeAssignment(r) => println!("{}ReturnTypeAssignmentNode: Value: {}", indent, r.value),
            ASTNode::Comment(c) => println!("{}CommentNode: Value: {}", indent, c.value),
            ASTNode::SemiColon => println!("{}SemicolonNode", indent),
            ASTNode::LeftParenthesis => println!("{}LeftParenthesisNode", indent),
            ASTNode::RightParenthesis => println!("{}RightParenthesisNode", indent),
            ASTNode::ArgumentSeparator => println!("{}ArgumentSeparatorNode", indent),
            ASTNode::LeftCurly => println!("{}LeftCurlyNode", indent),
            ASTNode::RightCurly => println!("{}RightCurlyNode", indent),
            ASTNode::None => println!("{}NoneNode", indent),
        }
    }*/
    //print variable stack
    println!("\nVariable stack:");
    for variable in unsafe { VARIABLE_STACK.iter() } {
        variable.print();
    }

    print_array_stack();
    print_dictionary_stack();
    Ok(())
}

#[cfg(test)]
mod main_test {

    #[test]
    fn test_check_file_extension() {
        let file_path = "test.jist";
        let result = super::check_file_extension(file_path.to_string());
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_check_file_extension_invalid() {
        let file_path = "test.txt";
        let result = super::check_file_extension(file_path.to_string());
        assert_eq!(result.is_err(), true);
    }
}

#[cfg(test)]
mod test_input_output {
    use assert_cmd::Command;
    use predicates::prelude::*;

    // Pass in a file path and check if the output is correct
    #[test]
    fn test_int_variable_declarations() {
        let file_path = "test_files/int_variable_declaration.jist";

        // Run the program and check the output
        let mut cmd = Command::cargo_bin("jist").unwrap();

        cmd.arg(file_path)
            .assert()
            .success() // Asserting that the command was successful
            .stdout(predicate::str::contains(
                "Variable Name: a\nVariable Type: Int\nVariable Value: 1",
            ));
    }

    #[test]
    fn test_string_variable_declaration() {
        let file_path = "test_files/string_variable_declaration.jist";

        let mut cmd = Command::cargo_bin("jist").unwrap();

        cmd.arg(file_path)
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "Variable Name: a\nVariable Type: String\nVariable Value: Hello World",
            ));
    }

    #[test]
    fn test_bool_variable_declaration() {
        let file_path = "test_files/boolean_variable_declaration.jist";
        let mut cmd = Command::cargo_bin("jist").unwrap();
        cmd.arg(file_path)
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "Variable Name: a\nVariable Type: Bool\nVariable Value: true",
            ));
    }

    #[test]
    fn test_char_variable_declaration() {
        let file_path = "test_files/char_variable_declaration.jist";
        let mut cmd = Command::cargo_bin("jist").unwrap();
        cmd.arg(file_path)
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "Variable Name: a\nVariable Type: Char\nVariable Value: a",
            ));
    }

    #[test]
    fn test_float_variable_declaration() {
        let file_path = "test_files/float_variable_declartion.jist";

        let mut cmd = Command::cargo_bin("jist").unwrap();

        cmd.arg(file_path)
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "Variable Name: a\nVariable Type: Float\nVariable Value: 3.141590118408203",
            ));
    }

    #[test]
    fn test_dict_boolean_string_collection_declaration() {
        let file_path = "test_files/dict_boolean_string_collection_declaration.jist";
        let mut cmd = Command::cargo_bin("jist").unwrap();
        cmd.arg(file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            r#"a: Dict<bool, string> = {"true" => true, "false" => false, "true" => not false}"#,
        ));
    }

    #[test]
    fn test_array_boolean_collection_declaration() {
        let file_path = "test_files/array_boolean_collection_declaration.jist";
        let mut cmd = Command::cargo_bin("jist").unwrap();
        cmd.arg(file_path)
            .assert()
            .success()
            .stdout(predicate::str::contains(
                r#"a: Array<bool> = [true, false, true]"#,
            ));
    }

    #[test]
    fn test_array_char_collection_declaration() {
        let file_path = "test_files/array_char_collection_declaration.jist";
        let mut cmd = Command::cargo_bin("jist").unwrap();
        cmd.arg(file_path)
            .assert()
            .success()
            .stdout(predicate::str::contains(r#"a: Array<char> = [a, b, c]"#));
    }

    #[test]
    fn test_array_float_collection_declaration() {
        let file_path = "test_files/array_float_collection_declaration.jist";
        let mut cmd = Command::cargo_bin("jist").unwrap();
        cmd.arg(file_path)
            .assert()
            .success()
            .stdout(predicate::str::contains(
                r#"a: Array<float> = [1.2300000190734863, 2.2300000190734863, 3.2300000190734863]"#,
            ));
    }

    #[test]
    fn test_array_int_collection_declaration() {
        let file_path = "test_files/array_int_collection_declaration.jist";
        let mut cmd = Command::cargo_bin("jist").unwrap();
        cmd.arg(file_path)
            .assert()
            .success()
            .stdout(predicate::str::contains(r#"a: Array<int> = [1, 2, 3]"#));
    }

    #[test]
    fn test_boolean_variable_declaration() {
        let file_path = "test_files/boolean_variable_declaration.jist";
        let mut cmd = Command::cargo_bin("jist").unwrap();
        cmd.arg(file_path)
            .assert()
            .success()
            .stdout(predicate::str::contains(r#"true"#));
    }

    #[test]
    fn test_dict_float_int_collection_declaration() {
        let file_path = "test_files/dict_float_int_collection_declaration.jist";
        let mut cmd = Command::cargo_bin("jist").unwrap();
        cmd.arg(file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            r#"a: Dict<float, int> = {"1.100000023841858" => 1, "2.0999999046325684" => 2, "3.9000000953674316" => 4}"#,
        ));
    }

    #[test]
    fn test_dict_int_char_collection_declaration() {
        let file_path = "test_files/dict_int_char_collection_declaration.jist";
        let mut cmd = Command::cargo_bin("jist").unwrap();
        cmd.arg(file_path)
            .assert()
            .success()
            .stdout(predicate::str::contains(
                r#"a: Dict<int, char> = {"1" => a, "2" => b, "3" => c}"#,
            ));
    }

    #[test]
    fn test_dict_int_string_collection_declaration() {
        let file_path = "test_files/dict_int_string_collection_declaration.jist";
        let mut cmd = Command::cargo_bin("jist").unwrap();
        cmd.arg(file_path)
            .assert()
            .success()
            .stdout(predicate::str::contains(
                r#"a: Dict<int, string> = {"1" => one, "2" => two, "3" => three}"#,
            ));
    }

    #[test]
    fn test_dict_string_float_collection_declaration() {
        let file_path = "test_files/dict_string_float_collection_declaration.jist";
        let mut cmd = Command::cargo_bin("jist").unwrap();
        cmd.arg(file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            r#"a: Dict<string, float> = {"one" => 1.100000023841858, "two" => 2.0999999046325684, "three" => 3.0999999046325684}"#,
        ));
    }
}
