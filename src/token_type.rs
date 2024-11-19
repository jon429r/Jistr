/*
* This file contains the token types used by the tokenizer, more can be added as needed to process
* a variety of syntax expressions
*/

pub mod token_types {

    #[derive(Debug, Clone)]
    pub enum TokenTypes {
        /*
         * A simple numeric value
         */
        Int,
        /*
         * A simple string value
         */
        String,
        /*
         * A simple character value
         */
        Char,
        /*
         * the = operator
         */
        AssignmentOperator,

        /*
         * either true or false
         */
        Bool,
        /*
         * A function argument
         */
        FunctionArguments,
        /*
         * An operator (+,-,/,*)
         */
        Operator,
        /*
         * (
         */
        LeftParenthesis,
        /*
         * )
         */
        RightParenthesis,
        /*
         * func
         */
        Function {
            name: String,
            return_type: String,
            arguments: Vec<(String, String, String)>,
        },
        /*
         * funcname()
         */
        FunctionCall,
        /*
         * ',' Comma used to separate function arguments
         */
        ArgumentSeparator,
        /*
         * a = 2
         */
        VariableCall,
        /*
         * values within () in a function call
         */
        FunctionCallArguments,
        /*
         * 'let' used to declare a variable
         */
        Assignment,
        /*
         * '}'
         */
        RightCurly,
        /*
         * '{'
         */
        LeftCurly,
        /*
         *let
         */
        Variable,
        /*
         * ':'
         */
        VarTypeAssignment,
        /*
         * '->'
         */
        ReturnTypeAssignment,
        /*
         * ; semicolon
         */
        SemiColon,
        /*
         * // or /* */
         */
        Comment,
        /*
         * 1.102
         */
        Float,
        /*
         *   Collection
         */
        Collection {
            name: String,
            collection_type: String,
            stored_value_type_single: String,
            stored_value_type_tuple: (String, String),
        },
        /*
         * if statement is simply a object name e.g. foo it is recognized as a call to an object,
         * var, collection, etc
         */
        ObjectCall {
            name: String,
        },
        /*
         * [
         */
        LeftBracket,
        /*
         * ]
         */
        RightBracket,
        /*
         * =>
         */
        FatArrow,
        /*
        Used as a bad return value
        */
        None,

        /*
         * used for dot notation eg obj.method()
         */
        Dot {
            object: String,
            method: String,
        },

        /*
         * If statement
         */
        If {
            statement: String,
        },
        /*
         * Else statement
         */
        Else,
        /*
         * Elif statement
         */
        Elif {
            statement: String,
        },
        /*
         * While statement
         */
        While {
            statement: String,
            block: Vec<String>,
        },
        /*
         * For statement
         */
        For {
            statement: String,
            block: Vec<String>,
        },
        /*
         * Break statement
         */
        Break,
        /*
         * Continue statement
         */
        Continue,
        /*
         * Try statement
         */
        Try,
        /*
         * Catch statement
         */
        Catch,
        /*
         * Finally statement
         */
        Finally,
        /*
         * !
         * */
        Not,
    }
    impl PartialEq for TokenTypes {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (
                    TokenTypes::Dot {
                        object: ref a,
                        method: ref b,
                    },
                    TokenTypes::Dot {
                        object: ref c,
                        method: ref d,
                    },
                ) => a == c && b == d,
                (TokenTypes::FunctionCallArguments, TokenTypes::FunctionCallArguments) => true,
                (
                    TokenTypes::ObjectCall { name: ref name1 },
                    TokenTypes::ObjectCall { name: ref name2 },
                ) => name1 == name2,
                (TokenTypes::SemiColon, TokenTypes::SemiColon) => true,
                (TokenTypes::Int, TokenTypes::Int) => true,
                (TokenTypes::Float, TokenTypes::Float) => true,
                (TokenTypes::String, TokenTypes::String) => true,
                (TokenTypes::Char, TokenTypes::Char) => true,
                (TokenTypes::Operator, TokenTypes::Operator) => true,
                (TokenTypes::AssignmentOperator, TokenTypes::AssignmentOperator) => true,
                (TokenTypes::LeftParenthesis, TokenTypes::LeftParenthesis) => true,
                (TokenTypes::RightParenthesis, TokenTypes::RightParenthesis) => true,
                (TokenTypes::FunctionCall, TokenTypes::FunctionCall) => true,
                (TokenTypes::VariableCall, TokenTypes::VariableCall) => true,
                (TokenTypes::ArgumentSeparator, TokenTypes::ArgumentSeparator) => true,
                (TokenTypes::Assignment, TokenTypes::Assignment) => true,
                (TokenTypes::VarTypeAssignment, TokenTypes::VarTypeAssignment) => true,
                (TokenTypes::RightCurly, TokenTypes::RightCurly) => true,
                (TokenTypes::LeftCurly, TokenTypes::LeftCurly) => true,
                (TokenTypes::ReturnTypeAssignment, TokenTypes::ReturnTypeAssignment) => true,
                (TokenTypes::Variable, TokenTypes::Variable) => true,

                (
                    TokenTypes::Function {
                        name: ref name_a,
                        return_type: ref return_a,
                        arguments: ref args_a,
                    },
                    TokenTypes::Function {
                        name: ref name_b,
                        return_type: ref return_b,
                        arguments: ref args_b,
                    },
                ) => name_a == name_b && return_a == return_b,

                (
                    TokenTypes::Collection {
                        name: ref name_a,
                        collection_type: ref type_a,
                        stored_value_type_single: ref stored_a,
                        stored_value_type_tuple: ref _tuple_a,
                    },
                    TokenTypes::Collection {
                        name: ref name_b,
                        collection_type: ref type_b,
                        stored_value_type_single: ref stored_b,
                        stored_value_type_tuple: ref _tuple_b,
                    },
                ) => name_a == name_b && type_a == type_b && stored_a == stored_b,

                (TokenTypes::Comment, TokenTypes::Comment) => true,
                (TokenTypes::Bool, TokenTypes::Bool) => true,
                (TokenTypes::LeftBracket, TokenTypes::LeftBracket) => true,
                (TokenTypes::RightBracket, TokenTypes::RightBracket) => true,
                (TokenTypes::FatArrow, TokenTypes::FatArrow) => true,
                (TokenTypes::None, TokenTypes::None) => true,
                (
                    TokenTypes::If {
                        statement: ref statement_a,
                    },
                    TokenTypes::If {
                        statement: ref statement_b,
                    },
                ) => statement_a == statement_b,
                (TokenTypes::Else, TokenTypes::Else) => true,
                (
                    TokenTypes::Elif {
                        statement: ref statement_a,
                    },
                    TokenTypes::Elif {
                        statement: ref statement_b,
                    },
                ) => statement_a == statement_b,
                (TokenTypes::Break, TokenTypes::Break) => true,
                (TokenTypes::Continue, TokenTypes::Continue) => true,
                (TokenTypes::Try, TokenTypes::Try) => true,
                (TokenTypes::Catch, TokenTypes::Catch) => true,
                (TokenTypes::Finally, TokenTypes::Finally) => true,
                (TokenTypes::Not, TokenTypes::Not) => true,
                (
                    TokenTypes::While {
                        statement: ref a, ..
                    },
                    TokenTypes::While {
                        statement: ref b, ..
                    },
                ) => a == b,
                (
                    TokenTypes::For {
                        statement: ref a, ..
                    },
                    TokenTypes::For {
                        statement: ref b, ..
                    },
                ) => a == b,
                _ => false,
            }
        }
    }

    impl Eq for TokenTypes {}

    impl TokenTypes {
        pub fn to_string(&self) -> String {
            match self {
                TokenTypes::ObjectCall { name } => format!("Object Call: name: {}", name),
                TokenTypes::Dot { object, method } => format!("Dot: {}.{}", object, method),
                TokenTypes::Function {
                    name,
                    return_type,
                    arguments,
                } => {
                    let mut arguments_str = String::new();
                    for arg in arguments {
                        arguments_str.push_str(&format!("{:?} ", arg));
                    }

                    format!("Function: {} {} {:?}", name, return_type, arguments_str)
                }
                TokenTypes::Not => "Not".to_string(),
                TokenTypes::Else => "Else".to_string(),
                TokenTypes::Elif { statement } => format!("Elif: {}", statement),
                TokenTypes::If { statement } => format!("If: {}", statement),
                TokenTypes::While { statement, block } => {
                    format!("While: {}, Block: {:?}", statement, block)
                }
                TokenTypes::For { statement, block } => {
                    format!("For: {}, Block: {:?}", statement, block)
                }
                TokenTypes::Break => "Break".to_string(),
                TokenTypes::Continue => "Continue".to_string(),
                TokenTypes::Try => "Try".to_string(),
                TokenTypes::Catch => "Catch".to_string(),
                TokenTypes::Finally => "Finally".to_string(),
                TokenTypes::FatArrow => "FatArrow".to_string(),
                TokenTypes::FunctionCallArguments => "FunctionCallArguments".to_string(),
                TokenTypes::Float => "Float".to_string(),
                TokenTypes::SemiColon => "SemiColon".to_string(),
                TokenTypes::FunctionArguments => "FunctionArguments".to_string(),
                TokenTypes::Int => "Int".to_string(),
                TokenTypes::String => "String".to_string(),
                TokenTypes::Char => "Char".to_string(),
                TokenTypes::Operator => "Operator".to_string(),
                TokenTypes::AssignmentOperator => "AssignmentOperator".to_string(),
                TokenTypes::Bool => "Bool".to_string(),
                TokenTypes::LeftParenthesis => "LeftParenthesis".to_string(),
                TokenTypes::RightParenthesis => "RightParenthesis".to_string(),
                TokenTypes::FunctionCall => "FunctionCall".to_string(),
                TokenTypes::Variable => "Variable".to_string(),
                TokenTypes::VariableCall => "VariableCall".to_string(),
                TokenTypes::ArgumentSeparator => "ArgumentSeparator".to_string(),
                TokenTypes::Assignment => "Assignment".to_string(),
                TokenTypes::VarTypeAssignment => "VarTypeAssignment".to_string(),
                TokenTypes::RightCurly => "RightCurly".to_string(),
                TokenTypes::Collection {
                    name,
                    collection_type,
                    stored_value_type_single,
                    stored_value_type_tuple,
                } => {
                    format!(
                        "Collection: {} {} {} {:?}",
                        name, collection_type, stored_value_type_single, stored_value_type_tuple
                    )
                }
                TokenTypes::LeftCurly => "LeftCurly".to_string(),
                TokenTypes::ReturnTypeAssignment => "ReturnTypeAssignment".to_string(),
                TokenTypes::Comment => "Comment".to_string(),
                TokenTypes::RightBracket => "RightBracket".to_string(),
                TokenTypes::LeftBracket => "LeftBracket".to_string(),
                TokenTypes::None => "None".to_string(),
            }
        }
    }
}
