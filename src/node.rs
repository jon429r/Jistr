/*
* This file contains the different AST node types used to create the AST tree and functions to go
* with them
*/

pub mod nodes {
    use crate::base_variable::base_types::BaseTypes;
    use crate::statement_tokenizer::tokenizer::tokenizers::ParseInfo;
    use crate::token_type::token_types::*;
    use std::fmt::{self, Display};
    use std::process::exit;

    pub fn to_base_type(node: &ASTNode) -> Option<BaseTypes> {
        match node {
            ASTNode::SemiColon => Some(BaseTypes::Null),
            ASTNode::Operator(_) => Some(BaseTypes::Null),
            ASTNode::Int(int_node) => Some(BaseTypes::Int(int_node.value)),
            ASTNode::String(string_node) => {
                Some(BaseTypes::StringWrapper(string_node.value.clone()))
            }
            ASTNode::Char(char_node) => Some(BaseTypes::Char(char_node.value)),
            ASTNode::Bool(bool_node) => Some(BaseTypes::Bool(bool_node.value)),
            ASTNode::Float(float_node) => Some(BaseTypes::Float(float_node.value as f64)),
            _ => {
                println!(
                    "This type {:?} cannot be converted to base type since it is not a value",
                    node
                );
                exit(1)
            } // cannot convert to base type since it is not a value, error
        }
    }

    pub fn from_base_type(base_type: BaseTypes) -> ASTNode {
        match base_type {
            BaseTypes::Int(value) => ASTNode::Int(IntNode::new(value)),
            BaseTypes::StringWrapper(value) => ASTNode::String(StringNode::new(value)),
            BaseTypes::Char(value) => ASTNode::Char(CharNode::new(value)),
            BaseTypes::Bool(value) => ASTNode::Bool(BoolNode::new(value)),
            BaseTypes::Float(value) => ASTNode::Float(FloatNode::new(value as f32)),
            _ => ASTNode::None,
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum ASTNode {
        ObjectCall(ObjectCallNode),
        CollectionCall(CollectionCallNode),
        Dot(DotNode),
        While(WhileNode),
        For(ForNode),
        If(IfNode),
        Elif(ElifNode),
        Else,
        Try,
        Catch,
        Finally,
        SemiColon,
        Operator(OperatorNode),
        Int(IntNode),
        String(StringNode),
        Char(CharNode),
        Bool(BoolNode),
        Float(FloatNode),
        Assignment(AssignmentNode),
        VarTypeAssignment(VarTypeAssignmentNode),
        Variable(VariableNode),
        Function(FunctionNode),
        FunctionCall(FunctionCallNode),
        VariableCall(VariableCallNode),
        VariableType(VariableTypeNode),
        VariableValue(VariableValueNode),
        FunctionArguments(FunctionArgumentsNode),
        AssignmentOperator(AssignmentOperatorNode),
        ReturnTypeAssignment(ReturnTypeAssignmentNode),
        Comment(CommentNode),
        FunctionCallArguments(FunctionArgumentsNode),
        Collection(CollectionNode),
        LeftBracket,
        RightBracket,
        LeftParenthesis,
        RightParenthesis,
        ArgumentSeparator,
        LeftCurly,
        RightCurly,
        FatArrow,
        None,
    }

    impl fmt::Display for ASTNode {
        ///
        ///Formats the ASTNode for printing
        ///
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ASTNode::CollectionCall(c) => write!(f, "{}", c),
                ASTNode::ObjectCall(c) => write!(f, "{}", c),
                ASTNode::Dot(d) => write!(f, "{}", d),
                ASTNode::While(w) => write!(f, "{}", w),
                ASTNode::For(fr) => write!(f, "{}", fr),
                ASTNode::If(i) => write!(f, "{}", i),
                ASTNode::Elif(e) => write!(f, "{}", e),
                ASTNode::Else => write!(f, "Else"),
                ASTNode::Try => write!(f, "Try"),
                ASTNode::Catch => write!(f, "Catch"),
                ASTNode::Finally => write!(f, "Finally"),
                ASTNode::Collection(c) => write!(f, "{:?}", c),
                ASTNode::SemiColon => write!(f, "SemiColon"),
                ASTNode::Operator(o) => write!(f, "{}", o),
                ASTNode::Int(i) => write!(f, "{}", i),
                ASTNode::String(s) => write!(f, "{}", s),
                ASTNode::Char(c) => write!(f, "{}", c),
                ASTNode::Bool(b) => write!(f, "{}", b),
                ASTNode::Float(fl) => write!(f, "{}", fl),
                ASTNode::Assignment(a) => write!(f, "{}", a),
                ASTNode::VarTypeAssignment(v) => write!(f, "{}", v),
                ASTNode::Variable(v) => write!(f, "{}", v),
                ASTNode::Function(fun) => write!(f, "{}", fun),
                ASTNode::FunctionCall(fun) => write!(f, "{}", fun),
                ASTNode::VariableCall(v) => write!(f, "{}", v),
                ASTNode::VariableType(v) => write!(f, "{}", v),
                ASTNode::VariableValue(v) => write!(f, "{}", v),
                ASTNode::AssignmentOperator(a) => write!(f, "{}", a),
                ASTNode::ReturnTypeAssignment(r) => write!(f, "{}", r),
                ASTNode::Comment(c) => write!(f, "{}", c),
                ASTNode::LeftParenthesis => write!(f, "LeftParenthesis"),
                ASTNode::RightParenthesis => write!(f, "RightParenthesis"),
                ASTNode::ArgumentSeparator => write!(f, "ArgumentSeparator"),
                ASTNode::LeftCurly => write!(f, "LeftCurly"),
                ASTNode::RightCurly => write!(f, "RightCurly"),
                ASTNode::RightBracket => write!(f, "RightBracket"),
                ASTNode::LeftBracket => write!(f, "LeftBracket"),
                ASTNode::FunctionCallArguments(call_args) => write!(f, "{}", call_args), // Call Display
                ASTNode::FunctionArguments(args) => write!(f, "{}", args), // Call Display
                ASTNode::FatArrow => write!(f, "FatArrow"),
                ASTNode::None => write!(f, "None"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CollectionCallNode {
        pub name: String,
    }

    impl CollectionCallNode {
        pub fn new(name: String) -> Self {
            CollectionCallNode { name }
        }
    }

    impl Display for CollectionCallNode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Collection Call Node, name {}", self.name)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ObjectCallNode {
        pub name: String,
    }

    impl ObjectCallNode {
        pub fn new(name: String) -> Self {
            ObjectCallNode { name }
        }
    }

    impl Display for ObjectCallNode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Object Call Node, name: {}", self.name)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct DotNode {
        pub object: String,
        pub function: String,
    }

    impl DotNode {
        pub fn new(object: String, function: String) -> Self {
            DotNode { object, function }
        }
    }

    impl fmt::Display for DotNode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Dot: object: {}, function: {}",
                self.object, self.function
            )
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WhileNode {
        pub condition: String,
        pub block: Vec<String>,
    }

    impl WhileNode {
        pub fn new(condition: String, block: Vec<String>) -> Self {
            WhileNode { condition, block }
        }
    }

    impl fmt::Display for WhileNode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "While: {}", self.condition)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ForNode {
        pub variable: String,
        pub iterable: (i32, i32),
        pub block: Vec<String>,
    }

    impl ForNode {
        pub fn new(variable: String, iterable: (i32, i32), block: Vec<String>) -> Self {
            ForNode {
                variable,
                iterable,
                block,
            }
        }
    }

    impl fmt::Display for ForNode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "For: {} {:?} ", self.variable, self.iterable)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct IfNode {
        pub condition: String,
    }

    impl IfNode {
        pub fn new(condition: String) -> Self {
            IfNode { condition }
        }
    }

    impl fmt::Display for IfNode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "If: {}", self.condition)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ElifNode {
        pub condition: String,
    }

    impl ElifNode {
        pub fn new(condition: String) -> Self {
            ElifNode { condition }
        }
    }

    impl fmt::Display for ElifNode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Elif: {}", self.condition)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ElseNode;

    impl ElseNode {
        pub fn new() -> Self {
            ElseNode
        }
    }

    impl fmt::Display for ElseNode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Else")
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct TryNode;

    impl TryNode {
        pub fn new() -> Self {
            TryNode
        }
    }

    impl fmt::Display for TryNode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Try")
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CatchNode;

    impl CatchNode {
        pub fn new() -> Self {
            CatchNode
        }
    }

    impl fmt::Display for CatchNode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Catch")
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FinallyNode;

    impl FinallyNode {
        pub fn new() -> Self {
            FinallyNode
        }
    }

    impl fmt::Display for FinallyNode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Finally")
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct FatArrowNode;

    #[derive(Debug, Clone, PartialEq)]
    pub struct CollectionNode {
        pub name: String,
        pub collection_type: String,
        pub value_type_tuple: Option<(String, String)>,
        pub value_type_single: Option<String>,
    }

    impl CollectionNode {
        pub fn new(
            name: String,
            collection_type: String,
            value_type_tuple: Option<(String, String)>,
            value_type_single: Option<String>,
        ) -> Self {
            CollectionNode {
                name,
                collection_type,
                value_type_tuple,
                value_type_single,
            }
        }

        pub fn display_info(&self) {
            println!("Collection: {}", self.name);
            println!("Collection Type: {}", self.collection_type);
            if let Some((value, value_type)) = &self.value_type_tuple {
                println!("Value Type Tuple: {} {}", value, value_type);
            }
            if let Some(value) = &self.value_type_single {
                println!("Value Type Single: {}", value);
            }
        }
    }

    impl fmt::Display for CollectionNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Collection: {}", self.name)?;
            write!(f, "Collection Type: {}", self.collection_type)?;
            if let Some((value, value_type)) = &self.value_type_tuple {
                write!(f, "Value Type Tuple: {} {}", value, value_type)?;
            }
            if let Some(value) = &self.value_type_single {
                write!(f, "Value Type Single: {}", value)?;
            }
            Ok(())
        }
    }

    // BoolNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct BoolNode {
        pub value: bool,
    }

    impl BoolNode {
        pub fn new(value: bool) -> Self {
            BoolNode { value }
        }
        pub fn display_info(&self) {
            println!("Bool: {}", self.value);
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FloatNode {
        pub value: f32,
    }

    impl FloatNode {
        pub fn new(value: f32) -> Self {
            FloatNode { value }
        }
        pub fn display_info(&self) {
            println!("Float: {}", self.value);
        }
    }

    impl fmt::Display for FloatNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Float: {}", self.value)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FunctionArgumentsNode {
        pub value: String,
    }

    impl FunctionArgumentsNode {
        pub fn new(value: String) -> Self {
            FunctionArgumentsNode { value }
        }

        pub fn display_info(&self) {
            println!("Function Arguments: {}", self.value);
        }
    }

    impl fmt::Display for FunctionArgumentsNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Function Arguments: {}", self.value)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ArgumentCallNode {
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FunctionCallArgumentsNode {
        pub value: String,
    }

    impl FunctionCallArgumentsNode {
        pub fn new(value: String) -> Self {
            FunctionCallArgumentsNode { value }
        }
        pub fn display_info(&self) {
            println!("Function Call Arguments: {}", self.value);
        }
    }

    impl fmt::Display for FunctionCallArgumentsNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Function Call Arguments: {}", self.value)
        }
    }

    // CommentNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct CommentNode {
        pub value: String,
    }

    impl CommentNode {
        pub fn new(value: String) -> Self {
            CommentNode { value }
        }
        pub fn display_info(&self) {
            println!("Comment: {}", self.value);
        }
    }

    impl fmt::Display for CommentNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Comment: {}", self.value)
        }
    }

    impl fmt::Display for BoolNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Bool: {}", self.value)
        }
    }

    // SemiColonNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct SemiColonNode;

    impl SemiColonNode {
        pub fn new() -> Self {
            SemiColonNode
        }
        pub fn display_info(&self) {
            println!("SemiColon");
        }
    }

    impl fmt::Display for SemiColonNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "SemiColon")
        }
    }

    // AssignmentOperatorNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct AssignmentOperatorNode {
        pub operator: String,
    }

    impl AssignmentOperatorNode {
        pub fn new(operator: String) -> Self {
            AssignmentOperatorNode { operator }
        }
        pub fn display_info(&self) {
            println!("Assignment Operator: {}", self.operator);
        }
    }

    impl fmt::Display for AssignmentOperatorNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Assignment Operator: {}", self.operator)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ReturnTypeAssignmentNode {
        pub value: String,
    }

    impl ReturnTypeAssignmentNode {
        pub fn new(value: String) -> Self {
            ReturnTypeAssignmentNode { value }
        }
        pub fn display_info(&self) {
            println!("Return Type Assignment: {}", self.value);
        }
    }

    impl fmt::Display for ReturnTypeAssignmentNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Return Type Assignment: {}", self.value)
        }
    }

    // OperatorNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct OperatorNode {
        pub operator: String,
    }

    impl OperatorNode {
        pub fn new(operator: String) -> Self {
            OperatorNode { operator }
        }
        pub fn display_info(&self) {
            println!("Operator: {}", self.operator);
        }
    }

    impl fmt::Display for OperatorNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Operator: {}", self.operator)
        }
    }

    // IntNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct IntNode {
        pub value: i32,
    }

    impl IntNode {
        pub fn new(value: i32) -> Self {
            IntNode { value }
        }
        pub fn display_info(&self) {
            println!("Int: {}", self.value);
        }
    }

    impl fmt::Display for IntNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Int: {}", self.value)
        }
    }

    // StringNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct StringNode {
        pub value: String,
    }

    impl StringNode {
        pub fn new(value: String) -> Self {
            StringNode { value }
        }
        pub fn display_info(&self) {
            println!("String: {}", self.value);
        }
    }

    impl fmt::Display for StringNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "String: {}", self.value)
        }
    }

    // CharNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct CharNode {
        pub value: char,
    }

    impl CharNode {
        pub fn new(value: char) -> Self {
            CharNode { value }
        }
        pub fn display_info(&self) {
            println!("Char: {}", self.value);
        }
    }

    impl fmt::Display for CharNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Char: {}", self.value)
        }
    }

    // AssignmentNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct AssignmentNode {
        pub value: String,
    }

    impl AssignmentNode {
        pub fn new(value: String) -> Self {
            AssignmentNode { value }
        }
        pub fn display_info(&self) {
            println!("Assignment: {}", self.value);
        }
    }

    impl fmt::Display for AssignmentNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Assignment: {}", self.value)
        }
    }

    // VarTypeAssignmentNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct VarTypeAssignmentNode {
        pub value: String,
    }

    impl VarTypeAssignmentNode {
        pub fn new(value: String) -> Self {
            VarTypeAssignmentNode { value }
        }
        pub fn display_info(&self) {
            println!("VarTypeAssignment: {}", self.value);
        }
    }

    impl fmt::Display for VarTypeAssignmentNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "VarTypeAssignment: {}", self.value)
        }
    }

    // VariableNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct VariableNode {
        pub var_type: String,
        pub value: String,
    }

    impl VariableNode {
        pub fn new(var_type: String, value: String) -> Self {
            VariableNode { var_type, value }
        }
        pub fn display_info(&self) {
            println!("Variable Type: {}, Value: {}", self.var_type, self.value);
        }
    }

    impl fmt::Display for VariableNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Variable Type: {}, Value: {}", self.var_type, self.value)
        }
    }

    // VariableValueNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct VariableValueNode {
        pub value: String,
    }

    impl VariableValueNode {
        pub fn new(value: String) -> Self {
            VariableValueNode { value }
        }
        pub fn display_info(&self) {
            println!("Variable Value: {}", self.value);
        }
    }

    impl fmt::Display for VariableValueNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Variable Value: {}", self.value)
        }
    }

    // VariableTypeNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct VariableTypeNode {
        pub value: String,
    }

    impl VariableTypeNode {
        pub fn new(value: String) -> Self {
            VariableTypeNode { value }
        }
        pub fn display_info(&self) {
            println!("Variable Type: {}", self.value);
        }
    }

    impl fmt::Display for VariableTypeNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Variable Type: {}", self.value)
        }
    }
    // VariableCall    // VariableCallNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct VariableCallNode {
        pub name: String,
    }

    impl VariableCallNode {
        pub fn new(name: String) -> Self {
            VariableCallNode { name }
        }
        pub fn display_info(&self) {
            println!("Variable Call: {}", self.name);
        }
    }

    impl fmt::Display for VariableCallNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Variable Call: {}", self.name)
        }
    }

    // FunctionNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct FunctionNode {
        pub name: String,
        pub return_type: String,
        pub arguments: Vec<(String, String, String)>,
        pub block: Vec<String>,
    }

    impl FunctionNode {
        pub fn new(
            name: String,
            return_type: String,
            arguments: Vec<(String, String, String)>,
            block: Vec<String>,
        ) -> Self {
            FunctionNode {
                name,
                return_type,
                arguments,
                block,
            }
        }
        pub fn display_info(&self) {
            let mut args_str = String::new();
            for (i, (arg_name, arg_type, arg_value)) in self.arguments.iter().enumerate() {
                if i > 0 {
                    args_str.push_str(", ");
                }
                args_str.push_str(&format!("{}: {} = {}", arg_name, arg_type, arg_value));
            }
            println!(
                "Function: {} Return Type: {}, Arguments: {}",
                self.name, self.return_type, args_str
            );
        }
    }

    impl fmt::Display for FunctionNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Function: {}", self.name)
        }
    }

    // FunctionCallNode implementation
    #[derive(Debug, Clone, PartialEq)]
    pub struct FunctionCallNode {
        pub name: String,
    }

    impl FunctionCallNode {
        pub fn new(name: String) -> Self {
            FunctionCallNode { name }
        }
        pub fn display_info(&self) {
            println!("Function Call: {}", self.name);
        }
    }

    impl fmt::Display for FunctionCallNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Function Call: {}", self.name)
        }
    }

    ///
    ///Checks to see if the parse info token is a valid ASTNode
    ///
    ///args: parse_info: ParseInfo, the tokenized info to be checked
    ///
    ///returns: ASTNode, the ASTNode that corresponds to the token
    ///
    pub fn match_token_to_node(parse_info: ParseInfo) -> ASTNode {
        match parse_info.token {
            TokenTypes::Int => {
                if let Ok(value) = parse_info.value.parse::<i32>() {
                    ASTNode::Int(IntNode::new(value))
                } else if let Ok(value) = parse_info.value.parse::<i32>() {
                    ASTNode::Int(IntNode::new(value))
                } else {
                    panic!("Failed to parse Int: {}", parse_info.value);
                }
            }
            TokenTypes::String => {
                let value = &parse_info.value[1..parse_info.value.len() - 1]; // Removes the first and last characters (quotes)
                ASTNode::String(StringNode::new(value.to_string()))
            }
            TokenTypes::Bool => ASTNode::Bool(BoolNode::new(
                parse_info.value.parse::<bool>().expect("Invalid bool"),
            )),
            TokenTypes::Float => ASTNode::Float(FloatNode::new(
                parse_info.value.parse::<f32>().expect("Invalid float"),
            )),
            TokenTypes::Char => {
                // cut out ' and ' from the string to get the value
                let char_value = parse_info.value.chars().nth(1).unwrap();

                ASTNode::Char(CharNode::new(char_value))
            }
            TokenTypes::Operator => ASTNode::Operator(OperatorNode::new(parse_info.value)),
            TokenTypes::AssignmentOperator => {
                ASTNode::AssignmentOperator(AssignmentOperatorNode::new(parse_info.value))
            }
            TokenTypes::LeftParenthesis => ASTNode::LeftParenthesis,
            TokenTypes::RightParenthesis => ASTNode::RightParenthesis,

            TokenTypes::Dot { object, method } => ASTNode::Dot(DotNode::new(object, method)),

            TokenTypes::Function {
                name,
                return_type,
                arguments,
                block,
            } => ASTNode::Function(FunctionNode::new(name, return_type, arguments, block)),
            TokenTypes::FunctionArguments => {
                ASTNode::FunctionArguments(FunctionArgumentsNode::new(parse_info.value))
            }
            TokenTypes::FunctionCall => {
                ASTNode::FunctionCall(FunctionCallNode::new(parse_info.value))
            }
            TokenTypes::Variable => {
                ASTNode::Variable(VariableNode::new("".to_string(), parse_info.value))
            }
            TokenTypes::VarTypeAssignment => {
                ASTNode::VariableType(VariableTypeNode::new(parse_info.value))
            }
            TokenTypes::VariableCall => {
                ASTNode::VariableCall(VariableCallNode::new(parse_info.value))
            }
            TokenTypes::ArgumentSeparator => ASTNode::ArgumentSeparator,
            TokenTypes::Assignment => ASTNode::Assignment(AssignmentNode::new(parse_info.value)),
            TokenTypes::RightCurly => ASTNode::RightCurly,
            TokenTypes::LeftCurly => ASTNode::LeftCurly,
            TokenTypes::ReturnTypeAssignment => {
                ASTNode::ReturnTypeAssignment(ReturnTypeAssignmentNode::new(parse_info.value))
            }
            TokenTypes::Comment => ASTNode::Comment(CommentNode::new(parse_info.value)),
            TokenTypes::SemiColon => ASTNode::SemiColon,
            TokenTypes::None => ASTNode::None,
            TokenTypes::Collection {
                name,
                collection_type,
                stored_value_type_single,
                stored_value_type_tuple,
            } => ASTNode::Collection(CollectionNode::new(
                name,
                collection_type,
                Some(stored_value_type_tuple),
                Some(stored_value_type_single),
            )),
            TokenTypes::LeftBracket => ASTNode::LeftBracket,
            TokenTypes::RightBracket => ASTNode::RightBracket,
            TokenTypes::FatArrow => ASTNode::FatArrow,
            TokenTypes::While { statement, block } => {
                ASTNode::While(WhileNode::new(statement, block))
            }
            TokenTypes::For {
                variable,
                iterable: (start, end),
                block,
            } => ASTNode::For(ForNode::new(variable, (start, end), block)),
            TokenTypes::If { statement } => ASTNode::If(IfNode::new(statement)),
            TokenTypes::Elif { statement } => ASTNode::Elif(ElifNode::new(statement)),
            TokenTypes::Else => ASTNode::Else,
            TokenTypes::Try => ASTNode::Try,
            TokenTypes::Catch => ASTNode::Catch,
            TokenTypes::Finally => ASTNode::Finally,
            TokenTypes::ObjectCall { name } => ASTNode::ObjectCall(ObjectCallNode::new(name)),

            _ => {
                panic!("Unrecognized token: {:?}", parse_info.token);
            }
        }
    }
}
