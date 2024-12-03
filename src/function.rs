use crate::function::functions::Function;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref FUNCTION_STACK: Mutex<Vec<Function>> = Mutex::new(Vec::new());
    pub static ref USER_FUNCTION_STACK: Mutex<Vec<Function>> = Mutex::new(Vec::new());
}

pub mod functions {

    use crate::base_variable::base_types::BaseTypes;
    use crate::base_variable::variable::Variable;
    use crate::collection::collections::Array;
    use crate::collection::collections::Dictionary;
    use crate::node::nodes::ASTNode;
    use std::fmt;

    pub enum FunctionTypes {
        // std array functions
        ArrayPopFn(fn(&mut Array) -> Option<BaseTypes>),
        ArrayPushFn(fn(&mut Array, BaseTypes)),
        ArrayRemoveFn(fn(&mut Array, BaseTypes)),
        ArrayInsertFn(fn(&mut Array, BaseTypes, BaseTypes)),
        ArrayGetFn(fn(&Array, BaseTypes) -> Option<BaseTypes>),
        ArraySetFn(fn(&mut Array, BaseTypes, BaseTypes) -> Option<BaseTypes>),
        ArrayAppendFn(fn(&mut Array, BaseTypes)),
        ArrayPrint(fn(&Array)),

        // std dictionary functions
        DictionaryAddFn(fn(&mut Dictionary, BaseTypes, BaseTypes)),
        DictionaryRemoveFn(fn(&mut Dictionary, BaseTypes)),

        DictionaryGetFn(fn(&Dictionary, BaseTypes) -> Option<(BaseTypes, BaseTypes)>),
        DictionarySetFn(fn(&mut Dictionary, BaseTypes, BaseTypes)),
        DictionaryKeysFn(fn(&Dictionary) -> Vec<BaseTypes>),
        DictionaryValuesFn(fn(&Dictionary) -> Vec<BaseTypes>),
        DictionaryPrint(fn(&Dictionary)),

        // std functions
        DoubleFloatArrayFn(fn(f64, f64) -> Vec<f64>),
        DoubleStringNoOutFn(fn(String, String)),
        FloatFn(fn(f64)),
        DoubleFloatFn(fn(f64, f64) -> f64),
        SingleFloatFn(fn(f64) -> f64),
        NoArgFloatFn(fn() -> f64),
        StringFn(fn(String)),
        DoubleStringFn(fn(String, String) -> String),
        SingleStringFn(fn(String) -> String),
        EchoFn(fn(String)),
    }

    use std::any::Any;
    impl PartialEq for FunctionTypes {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (FunctionTypes::FloatFn(f1), FunctionTypes::FloatFn(f2)) => {
                    f1 as *const _ == f2 as *const _
                }
                (FunctionTypes::DoubleFloatFn(f1), FunctionTypes::DoubleFloatFn(f2)) => {
                    f1 as *const _ == f2 as *const _
                }
                (FunctionTypes::SingleFloatFn(f1), FunctionTypes::SingleFloatFn(f2)) => {
                    f1 as *const _ == f2 as *const _
                }
                (FunctionTypes::NoArgFloatFn(f1), FunctionTypes::NoArgFloatFn(f2)) => {
                    f1 as *const _ == f2 as *const _
                }
                (FunctionTypes::StringFn(f1), FunctionTypes::StringFn(f2)) => {
                    f1 as *const _ == f2 as *const _
                }
                (FunctionTypes::DoubleStringFn(f1), FunctionTypes::DoubleStringFn(f2)) => {
                    f1 as *const _ == f2 as *const _
                }
                (FunctionTypes::SingleStringFn(f1), FunctionTypes::SingleStringFn(f2)) => {
                    f1 as *const _ == f2 as *const _
                }
                (FunctionTypes::EchoFn(f1), FunctionTypes::EchoFn(f2)) => {
                    f1 as *const _ == f2 as *const _
                }
                _ => false, // Different types cannot be equal
            }
        }
    }

    pub fn call_function(func: &FunctionTypes, arguments: Vec<Box<dyn Any>>) -> Box<dyn Any> {
        match func {
            FunctionTypes::DoubleFloatFn(f) => {
                if arguments.len() == 2 {
                    let arg1 = arguments[0].downcast_ref::<f64>().expect("Expected f64");
                    let arg2 = arguments[1].downcast_ref::<f64>().expect("Expected f64");
                    let result = f(*arg1, *arg2);
                    //println!("DoubleFn result: {}", result);
                    return Box::new(result);
                } else {
                    panic!("Expected exactly two arguments for DoubleFn");
                }
            }
            FunctionTypes::SingleFloatFn(f) => {
                if arguments.len() == 1 {
                    let arg = arguments[0].downcast_ref::<f64>().expect("Expected f64");
                    let result = f(*arg);
                    //println!("SingleFn result: {}", result);
                    return Box::new(result);
                } else {
                    panic!("Expected exactly one argument for SingleFn");
                }
            }
            FunctionTypes::NoArgFloatFn(f) => {
                if arguments.is_empty() {
                    let result = f();
                    //println!("NoArgFn result: {}", result);
                    return Box::new(result);
                } else {
                    panic!("Expected no arguments for NoArgFn");
                }
            }
            FunctionTypes::StringFn(f) => {
                if arguments.len() == 1 {
                    let arg = arguments[0]
                        .downcast_ref::<String>()
                        .expect("Expected String");
                    f(arg.clone());
                    //println!("StringFn called with: {}", arg);
                    return Box::new(());
                } else {
                    panic!("Expected exactly one argument for StringFn");
                }
            }
            FunctionTypes::DoubleStringFn(f) => {
                if arguments.len() == 2 {
                    let arg1 = arguments[0]
                        .downcast_ref::<String>()
                        .expect("Expected String");
                    let arg2 = arguments[1]
                        .downcast_ref::<String>()
                        .expect("Expected String");
                    let result = f(arg1.clone(), arg2.clone());
                    //println!("DoubleStringFn result: {}", result);
                    return Box::new(result);
                } else {
                    panic!("Expected exactly two arguments for DoubleStringFn");
                }
            }
            FunctionTypes::SingleStringFn(f) => {
                if arguments.len() == 1 {
                    let arg = arguments[0]
                        .downcast_ref::<String>()
                        .expect("Expected String");
                    let result = f(arg.clone());
                    //print!("SingleStringFn result: {}", result);
                    Box::new(result)
                } else {
                    panic!("Expected exactly one argument for SingleStringFn");
                }
            }

            FunctionTypes::DoubleFloatArrayFn(f) => {
                if arguments.len() == 2 {
                    let arg1 = arguments[0].downcast_ref::<f64>().expect("Expected f64");
                    let arg2 = arguments[1].downcast_ref::<f64>().expect("Expected f64");
                    let result = f(*arg1, *arg2);
                    //println!("DoubleFloatArrayFn result: {:?}", result);
                    return Box::new(result);
                } else {
                    panic!("Expected exactly two arguments for DoubleFloatArrayFn");
                }
            }

            FunctionTypes::ArrayAppendFn(f) => {
                if arguments.len() == 2 {
                    let arg1 = arguments[0]
                        .downcast_ref::<Array>()
                        .expect("Expected Array");
                    let arg2 = arguments[1]
                        .downcast_ref::<BaseTypes>()
                        .expect("Expected BaseTypes");
                    f(&mut arg1.clone(), arg2.clone());
                    //println!("ArrayAppendFn called with: {:?}", arg2);
                    return Box::new(());
                } else {
                    panic!("Expected exactly two arguments for ArrayAppendFn");
                }
            }

            FunctionTypes::ArrayPopFn(f) => {
                //println!("ArrayPopFn called arguments: {:?}", arguments);
                if arguments.len() == 1 {
                    // Extract the argument and ensure it's of type Array
                    let arg = arguments[0]
                        .downcast_ref::<Array>()
                        .expect("Expected Array");

                    // Call the function with the array
                    let result = f(&mut arg.clone()); // Array passed here

                    return Box::new(result);
                } else {
                    panic!("Expected exactly one argument for ArrayPopFn (the array itself)");
                }
            }

            FunctionTypes::ArrayPushFn(f) => {
                //print arguments
                if arguments.len() == 2 {
                    let arg1 = arguments[0]
                        .downcast_ref::<Array>()
                        .expect("Expected Array");
                    let arg2 = arguments[1]
                        .downcast_ref::<BaseTypes>()
                        .expect("Expected BaseTypes");

                    f(&mut arg1.clone(), arg2.clone());
                    //println!("ArrayPushFn called with: {:?}", arg2);
                    return Box::new(());
                } else {
                    panic!("Expected exactly two arguments for ArrayPushFn");
                }
            }

            FunctionTypes::DoubleStringNoOutFn(f) => {
                if arguments.len() == 2 {
                    let arg1 = arguments[0]
                        .downcast_ref::<String>()
                        .expect("Expected String");
                    let arg2 = arguments[1]
                        .downcast_ref::<String>()
                        .expect("Expected String");
                    f(arg1.clone(), arg2.clone());
                    //println!("DoubleStringNoOutFn called with: {:?}, {:?}", arg1, arg2);
                    return Box::new(());
                } else {
                    panic!("Expected exactly two arguments for DoubleStringNoOutFn");
                }
            }

            FunctionTypes::ArrayRemoveFn(f) => {
                if arguments.len() == 2 {
                    let arg1 = arguments[0]
                        .downcast_ref::<Array>()
                        .expect("Expected Array");
                    let arg2 = arguments[1]
                        .downcast_ref::<BaseTypes>()
                        .expect("Expected BaseTypes");

                    f(&mut arg1.clone(), (*arg2).clone().into());
                    //println!("ArrayRemoveFn called with: {:?}", arg2);
                    return Box::new(());
                } else {
                    panic!("Expected exactly two arguments for ArrayRemoveFn");
                }
            }

            FunctionTypes::ArrayInsertFn(f) => {
                if arguments.len() == 3 {
                    let arg1 = arguments[0]
                        .downcast_ref::<Array>()
                        .expect("Expected Array");
                    let arg2 = arguments[1]
                        .downcast_ref::<usize>()
                        .expect("Expected usize");
                    let arg3 = arguments[2]
                        .downcast_ref::<BaseTypes>()
                        .expect("Expected BaseTypes");
                    f(&mut arg1.clone(), (*arg2).clone().into(), arg3.clone());
                    //println!("ArrayInsertFn called with: {:?}", arg2);
                    return Box::new(());
                } else {
                    panic!("Expected exactly three arguments for ArrayInsertFn");
                }
            }

            FunctionTypes::ArrayGetFn(f) => {
                if arguments.len() == 2 {
                    let mut arg1 = arguments[0]
                        .downcast_ref::<Array>()
                        .expect("Expected Array");
                    let arg2 = arguments[1]
                        .downcast_ref::<BaseTypes>()
                        .expect("Expected BaseTypes");

                    let result = f(&mut arg1, arg2.clone().into());
                    let result = result.unwrap();

                    return Box::new(result);
                } else {
                    panic!("Expected exactly two arguments for ArrayGetFn");
                }
            }

            FunctionTypes::ArraySetFn(f) => {
                if arguments.len() == 3 {
                    let arg1 = arguments[0]
                        .downcast_ref::<Array>()
                        .expect("Expected Array");
                    let arg2 = arguments[1]
                        .downcast_ref::<BaseTypes>()
                        .expect("Expected BaseTypes");

                    let arg3 = arguments[2]
                        .downcast_ref::<BaseTypes>()
                        .expect("Expected BaseTypes");
                    let result = f(&mut arg1.clone(), (*arg2).clone().into(), arg3.clone());
                    //println!("ArraySetFn result: {:?}", result);
                    return Box::new(result);
                } else {
                    panic!("Expected exactly three arguments for ArraySetFn");
                }
            }

            FunctionTypes::ArrayPrint(f) => {
                if arguments.len() == 1 {
                    let arg = arguments[0]
                        .downcast_ref::<Array>()
                        .expect("Expected Array");
                    f(&arg.clone());
                    //println!("ArrayPrint called with: {:?}", arg);
                    return Box::new(());
                } else {
                    panic!("Expected exactly one argument for ArrayPrint");
                }
            }

            FunctionTypes::DictionaryPrint(f) => {
                if arguments.len() == 1 {
                    let arg = arguments[0]
                        .downcast_ref::<Dictionary>()
                        .expect("Expected Dictionary");
                    f(&arg.clone());
                    //println!("DictionaryPrint called with: {:?}", arg);
                    return Box::new(());
                } else {
                    panic!("Expected exactly one argument for DictionaryPrint");
                }
            }

            FunctionTypes::DictionaryAddFn(f) => {
                if arguments.len() == 3 {
                    let arg1 = arguments[0]
                        .downcast_ref::<Dictionary>()
                        .expect("Expected Dictionary");
                    let arg2 = arguments[1]
                        .downcast_ref::<BaseTypes>()
                        .expect("Expected BaseTypes");
                    let arg3 = arguments[2]
                        .downcast_ref::<BaseTypes>()
                        .expect("Expected BaseTypes");
                    f(&mut arg1.clone(), arg2.clone(), arg3.clone());
                    //println!("DictionaryAddFn called with: {:?}", arg2);
                    return Box::new(());
                } else {
                    panic!("Expected exactly three arguments for DictionaryAddFn");
                }
            }

            FunctionTypes::DictionaryRemoveFn(f) => {
                if arguments.len() == 2 {
                    let arg1 = arguments[0]
                        .downcast_ref::<Dictionary>()
                        .expect("Expected Dictionary");
                    let arg2 = arguments[1]
                        .downcast_ref::<BaseTypes>()
                        .expect("Expected BaseTypes");
                    f(&mut arg1.clone(), arg2.clone());
                    //println!("DictionaryRemoveFn called with: {:?}", arg2);
                    return Box::new(());
                } else {
                    panic!("Expected exactly two arguments for DictionaryRemoveFn");
                }
            }

            FunctionTypes::DictionaryGetFn(f) => {
                if arguments.len() == 1 {
                    let arg = arguments[0]
                        .downcast_ref::<Dictionary>()
                        .expect("Expected Dictionary");
                    let arg2 = arguments[1]
                        .downcast_ref::<BaseTypes>()
                        .expect("Expected BaseTypes");
                    let result: Option<(BaseTypes, BaseTypes)> = f(arg, arg2.clone());

                    // result is now an Option<(BaseTypes, BaseTypes)>
                    return Box::new(result);
                } else {
                    panic!("Expected exactly one argument for DictionaryGetFn");
                }
            }

            FunctionTypes::DictionarySetFn(f) => {
                if arguments.len() == 3 {
                    let arg1 = arguments[0]
                        .downcast_ref::<Dictionary>()
                        .expect("Expected Dictionary");
                    let arg2 = arguments[1]
                        .downcast_ref::<BaseTypes>()
                        .expect("Expected BaseTypes");
                    let arg3 = arguments[2]
                        .downcast_ref::<BaseTypes>()
                        .expect("Expected BaseTypes");
                    let result = f(&mut arg1.clone(), arg2.clone(), arg3.clone());
                    //println!("DictionarySetFn result: {:?}", result);
                    return Box::new(());
                } else {
                    panic!("Expected exactly three arguments for DictionarySetFn");
                }
            }

            FunctionTypes::DictionaryKeysFn(f) => {
                if arguments.len() == 1 {
                    let arg = arguments[0]
                        .downcast_ref::<Dictionary>()
                        .expect("Expected Dictionary");
                    let result = f(arg); // No need to clone `arg` here
                    return Box::new(result); // Ensure `result` is of the right type
                } else {
                    panic!("Expected exactly one argument for DictionaryKeysFn");
                }
            }

            FunctionTypes::DictionaryValuesFn(f) => {
                if arguments.len() == 1 {
                    let arg = arguments[0]
                        .downcast_ref::<Dictionary>()
                        .expect("Expected Dictionary");
                    let result = f(arg); // No need to clone `arg` here
                    return Box::new(result); // Ensure `result` is of the right type
                } else {
                    panic!("Expected exactly one argument for DictionaryValuesFn");
                }
            }

            FunctionTypes::EchoFn(f) => {
                if arguments.len() == 1 {
                    let arg = if let Some(s) = arguments[0].downcast_ref::<String>() {
                        // Argument is already a String
                        s.clone()
                    } else if let Some(i) = arguments[0].downcast_ref::<i32>() {
                        // Argument is an i32, convert to String
                        i.to_string()
                    } else if let Some(flt) = arguments[0].downcast_ref::<f64>() {
                        // Argument is a f64, convert to String
                        flt.to_string()
                    } else if let Some(b) = arguments[0].downcast_ref::<bool>() {
                        // Argument is a bool, convert to String
                        b.to_string()
                    } else if let Some(b) = arguments[0].downcast_ref::<char>() {
                        // Argument is a char, convert to String
                        b.to_string()
                    } else {
                        panic!(
                            "Expected String or convertible type, but found {:?}",
                            arguments[0].type_id()
                        );
                    };

                    f(arg); // Call the function with the converted argument.
                    return Box::new(());
                } else {
                    println!("Arguments: {:?}", arguments);
                    panic!(
                        "Expected exactly one argument for EchoFn, but got {}",
                        arguments.len()
                    );
                }
            }
            _ => {
                panic!("Function not implemented");
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Function {
        pub name: String,
        pub return_type: BaseTypes,
        pub arguments: Vec<Variable>,
        pub body: Vec<ASTNode>,
    }

    // Constructor for creating a new function
    impl Function {
        pub fn new(
            name: String,
            return_type: BaseTypes,
            arguments: Vec<Variable>,
            body: Vec<ASTNode>,
        ) -> Function {
            Function {
                name,
                return_type,
                arguments,
                body,
            }
        }
    }

    // Implementing Display trait for Function
    impl fmt::Display for Function {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Formatting the arguments and body for proper display
            let arguments = self
                .arguments
                .iter()
                .map(|arg| format!("{}", arg)) // Assuming Variable implements Display
                .collect::<Vec<String>>()
                .join(", ");

            let body = self
                .body
                .iter()
                .map(|token| format!("{:?}", token)) // Assuming TokenTypes implements Display
                .collect::<Vec<String>>()
                .join("\n");

            write!(
                f,
                "func {}({}) -> {}\n{}",
                self.name, arguments, self.return_type, body
            )
        }
    }

    impl Function {
        // Function to call the function
        pub fn call(&mut self) {
            // Logic for calling the function can be implemented here
        }
    }
}
