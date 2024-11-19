pub mod variables {
    use super::variable::Variable;
    // use super::base_variables::BaseVariables::{Pi, E};

    pub static mut VARIABLE_STACK: Vec<Variable> = Vec::new();
}

pub mod variable {
    use super::base_types::BaseTypes;
    use super::variables::VARIABLE_STACK;
    use crate::base_variable::base_types::GetType;
    use crate::node::nodes::ASTNode;
    use std::fmt;

    #[derive(Debug, Clone)]
    pub struct Variable {
        pub name: String,
        pub value: BaseTypes,
        pub var_type: BaseTypes,
    }

    pub trait GetValue {
        fn get_value(&self) -> BaseTypes;
    }

    pub trait SetValue {
        fn set_value<T>(&mut self, value: T)
        where
            T: Into<BaseTypes>;
    }

    pub trait SetType {
        fn set_type(&mut self, var_type: BaseTypes);
    }

    pub trait SetName {
        fn set_name(&mut self, name: String);
    }

    pub trait Increment {
        fn increment(&mut self);
    }

    pub trait Decrement {
        fn decrement(&mut self);
    }

    impl PartialEq for BaseTypes {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (BaseTypes::Int(x), BaseTypes::Int(y)) => x == y,
                (BaseTypes::Float(x), BaseTypes::Float(y)) => x == y,
                (BaseTypes::StringWrapper(s1), BaseTypes::StringWrapper(s2)) => s1 == s2,
                (BaseTypes::Char(c1), BaseTypes::Char(c2)) => c1 == c2,
                _ => false,
            }
        }
    }

    impl PartialOrd for BaseTypes {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match (self, other) {
                (BaseTypes::Int(x), BaseTypes::Int(y)) => x.partial_cmp(y),
                (BaseTypes::Float(x), BaseTypes::Float(y)) => x.partial_cmp(y),
                (BaseTypes::StringWrapper(s1), BaseTypes::StringWrapper(s2)) => s1.partial_cmp(s2),
                (BaseTypes::Char(c1), BaseTypes::Char(c2)) => c1.partial_cmp(c2),
                _ => {
                    println!("Warning: Cannot compare different types.");
                    None
                }
            }
        }

        fn lt(&self, other: &Self) -> bool {
            match (self, other) {
                (BaseTypes::Int(x), BaseTypes::Int(y)) => x < y,
                (BaseTypes::Float(x), BaseTypes::Float(y)) => x < y,
                (BaseTypes::StringWrapper(s1), BaseTypes::StringWrapper(s2)) => s1 < s2,
                (BaseTypes::Char(c1), BaseTypes::Char(c2)) => c1 < c2,
                _ => {
                    println!("Warning: Cannot compare different types.");
                    false
                }
            }
        }

        fn gt(&self, other: &Self) -> bool {
            match (self, other) {
                (BaseTypes::Int(x), BaseTypes::Int(y)) => x > y,
                (BaseTypes::Float(x), BaseTypes::Float(y)) => x > y,
                (BaseTypes::StringWrapper(s1), BaseTypes::StringWrapper(s2)) => s1 > s2,
                (BaseTypes::Char(c1), BaseTypes::Char(c2)) => c1 > c2,
                _ => {
                    println!("Warning: Cannot compare different types.");
                    false
                }
            }
        }

        fn ge(&self, other: &Self) -> bool {
            match (self, other) {
                (BaseTypes::Int(x), BaseTypes::Int(y)) => x >= y,
                (BaseTypes::Float(x), BaseTypes::Float(y)) => x >= y,
                (BaseTypes::StringWrapper(s1), BaseTypes::StringWrapper(s2)) => s1 >= s2,
                (BaseTypes::Char(c1), BaseTypes::Char(c2)) => c1 >= c2,
                _ => {
                    println!("Warning: Cannot compare different types.");
                    false
                }
            }
        }

        fn le(&self, other: &Self) -> bool {
            match (self, other) {
                (BaseTypes::Int(x), BaseTypes::Int(y)) => x <= y,
                (BaseTypes::Float(x), BaseTypes::Float(y)) => x <= y,
                (BaseTypes::StringWrapper(s1), BaseTypes::StringWrapper(s2)) => s1 <= s2,
                (BaseTypes::Char(c1), BaseTypes::Char(c2)) => c1 <= c2,
                _ => {
                    println!("Warning: Cannot compare different types.");
                    false
                }
            }
        }
    }

    impl std::ops::Add for BaseTypes {
        type Output = BaseTypes;
        fn add(self, other: BaseTypes) -> BaseTypes {
            match (self, other) {
                (BaseTypes::Int(x), BaseTypes::Int(y)) => BaseTypes::Int(x + y),
                (BaseTypes::Float(x), BaseTypes::Float(y)) => BaseTypes::Float(x + y),
                (BaseTypes::StringWrapper(s1), BaseTypes::StringWrapper(s2)) => {
                    BaseTypes::StringWrapper(s1 + &s2)
                }
                _ => {
                    println!("Warning: Cannot add different types as well as char type.");
                    BaseTypes::Null
                }
            }
        }
    }

    impl std::ops::Sub for BaseTypes {
        type Output = BaseTypes;
        fn sub(self, other: BaseTypes) -> BaseTypes {
            match (self, other) {
                (BaseTypes::Int(x), BaseTypes::Int(y)) => BaseTypes::Int(x - y),
                (BaseTypes::Float(x), BaseTypes::Float(y)) => BaseTypes::Float(x - y),
                _ => {
                    println!(
                        "Warning: Cannot subtract different types as well as string and char type."
                    );
                    BaseTypes::Null
                }
            }
        }
    }

    impl std::ops::Mul for BaseTypes {
        type Output = BaseTypes;
        fn mul(self, other: BaseTypes) -> BaseTypes {
            match (self, other) {
                (BaseTypes::Int(x), BaseTypes::Int(y)) => BaseTypes::Int(x * y),
                (BaseTypes::Float(x), BaseTypes::Float(y)) => BaseTypes::Float(x * y),
                _ => {
                    println!(
                        "Warning: Cannot multiply different types as well as string and char type."
                    );
                    BaseTypes::Null
                }
            }
        }
    }

    impl std::ops::Div for BaseTypes {
        type Output = BaseTypes;
        fn div(self, other: BaseTypes) -> BaseTypes {
            match (self, other) {
                (BaseTypes::Int(x), BaseTypes::Int(y)) => BaseTypes::Int(x / y),
                (BaseTypes::Float(x), BaseTypes::Float(y)) => BaseTypes::Float(x / y),
                _ => {
                    println!(
                        "Warning: Cannot divide different types as well as string and char type."
                    );
                    BaseTypes::Null
                }
            }
        }
    }

    impl std::ops::Rem for BaseTypes {
        type Output = BaseTypes;
        fn rem(self, other: BaseTypes) -> BaseTypes {
            match (self, other) {
                (BaseTypes::Int(x), BaseTypes::Int(y)) => BaseTypes::Int(x % y),
                _ => {
                    println!(
                        "Warning: Cannot divide different types as well as string and char type."
                    );
                    BaseTypes::Null
                }
            }
        }
    }

    impl Variable {
        pub fn new(name: String, value: BaseTypes, var_type: BaseTypes) -> Variable {
            //println!("Variable info: {}, {:?}, {:?}", name, value, var_type);

            // Ensure the value type matches the variable type
            let checked_value = match var_type {
                BaseTypes::Int(_) => match value {
                    BaseTypes::Int(_) => value,
                    BaseTypes::Null => BaseTypes::Int(0),
                    BaseTypes::Float(_) => BaseTypes::Int(value.into()),
                    _ => {
                        println!(
                            "Warning: Value type mismatch for '{}'. Setting default Int value.",
                            name
                        );
                        BaseTypes::Int(0)
                    }
                },
                BaseTypes::Float(_) => {
                    match value {
                        BaseTypes::Float(_) => value,
                        BaseTypes::Null => BaseTypes::Float(0.0),
                        BaseTypes::Int(_) => BaseTypes::Float(value.into()),
                        _ => {
                            println!("Warning: Value type mismatch for '{}'. Setting default Float value.", name);
                            BaseTypes::Float(0.0)
                        }
                    }
                }
                BaseTypes::StringWrapper(_) => {
                    match value {
                        BaseTypes::StringWrapper(_) => value,
                        BaseTypes::Char(_) => BaseTypes::StringWrapper(value.into()),

                        BaseTypes::Null => BaseTypes::StringWrapper(String::new()),

                        _ => {
                            println!("Warning: Value type mismatch for '{}'. Setting default String value.", name);
                            BaseTypes::StringWrapper(String::new())
                        }
                    }
                }
                BaseTypes::Bool(_) => {
                    match value {
                        BaseTypes::Bool(_) => value,
                        BaseTypes::Int(1) => BaseTypes::Bool(true),
                        BaseTypes::Int(0) => BaseTypes::Bool(false),
                        BaseTypes::Null => BaseTypes::Bool(false),

                        _ => {
                            println!("Warning: Value type mismatch for '{}'. Setting default Bool value.", name);
                            BaseTypes::Bool(false)
                        }
                    }
                }

                BaseTypes::Char(_) => {
                    match value {
                        BaseTypes::Char(_) => value,
                        BaseTypes::Null => BaseTypes::Char('\0'),

                        _ => {
                            println!("Warning: Value type mismatch for '{}'. Setting default Char value.", name);
                            BaseTypes::Char('\0')
                        }
                    }
                }

                BaseTypes::Null => {
                    match value {
                        BaseTypes::Null => value,
                        _ => {
                            println!("Warning: Value type mismatch for '{}'. Setting default Null value.", name);
                            BaseTypes::Null
                        }
                    }
                }
            };

            Variable {
                name,
                value: checked_value,
                var_type,
            }
        }

        pub fn set_name(&mut self, name: String) {
            self.name = name;
        }

        pub fn set_type(&mut self, var_type: BaseTypes) {
            self.var_type = var_type;
        }

        pub fn set_value<T>(&mut self, value: T)
        where
            T: Into<BaseTypes>,
        {
            self.value = value.into();
            //now update the variable in the stack
            for variable in unsafe { VARIABLE_STACK.iter_mut() } {
                if variable.name == self.name {
                    variable.value = self.value.clone();
                    break; // Exit loop once the variable is found and updated
                }
            }
        }

        pub fn get_value(&self) -> &BaseTypes {
            &self.value
        }

        pub fn increment(&mut self) {
            println!("Incrementing variable: {}", self);
            match self.value {
                BaseTypes::Int(ref mut i) => {
                    // Increment the integer value in place
                    *i += 1;

                    // Now update the variable in the stack
                    for variable in unsafe { VARIABLE_STACK.iter_mut() } {
                        if variable.name == self.name {
                            variable.value = BaseTypes::Int(*i);
                            break; // Exit loop once the variable is found and updated
                        }
                    }
                }
                BaseTypes::Float(ref mut f) => {
                    // Increment the float value in place
                    *f += 1.0;

                    // Now update the variable in the stack
                    for variable in unsafe { VARIABLE_STACK.iter_mut() } {
                        if variable.name == self.name {
                            variable.value = BaseTypes::Float(*f);
                            break; // Exit loop once the variable is found and updated
                        }
                    }
                }
                _ => println!("Warning: Cannot increment non-numeric type."),
            }
        }
        //use crate::node::nodes::ASTNode;

        pub fn decrement(&mut self) {
            match self.value {
                BaseTypes::Int(ref mut i) => {
                    // Decrement the integer value in place
                    *i -= 1;

                    // Now update the variable in the stack
                    for variable in unsafe { VARIABLE_STACK.iter_mut() } {
                        if variable.name == self.name {
                            variable.value = BaseTypes::Int(*i);
                            break; // Exit loop once the variable is found and updated
                        }
                    }
                }
                BaseTypes::Float(ref mut f) => {
                    // Decrement the float value in place
                    *f -= 1.0;

                    // Now update the variable in the stack
                    for variable in unsafe { VARIABLE_STACK.iter_mut() } {
                        if variable.name == self.name {
                            variable.value = BaseTypes::Float(*f);
                            break; // Exit loop once the variable is found and updated
                        }
                    }
                }
                _ => println!("Warning: Cannot decrement non-numeric type."),
            }
        }

        pub fn print(&self) {
            println!("Variable Name: {}", self.name);
            println!("Variable Type: {}", self.var_type.GetType());
            println!("Variable Value: {}", self.value);
        }
    }

    impl fmt::Display for Variable {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.name)
        }
    }

    impl From<i32> for BaseTypes {
        fn from(value: i32) -> Self {
            BaseTypes::Int(value)
        }
    }

    impl From<f64> for BaseTypes {
        fn from(value: f64) -> Self {
            BaseTypes::Float(value.into())
        }
    }

    impl From<String> for BaseTypes {
        fn from(value: String) -> Self {
            BaseTypes::StringWrapper(value)
        }
    }

    impl From<&str> for BaseTypes {
        fn from(value: &str) -> Self {
            BaseTypes::StringWrapper(value.to_string())
        }
    }

    impl From<bool> for BaseTypes {
        fn from(value: bool) -> Self {
            BaseTypes::Bool(value)
        }
    }

    impl From<char> for BaseTypes {
        fn from(value: char) -> Self {
            BaseTypes::Char(value)
        }
    }

    impl From<usize> for BaseTypes {
        fn from(value: usize) -> Self {
            BaseTypes::Int(value as i32)
        }
    }

    impl From<()> for BaseTypes {
        fn from(_: ()) -> Self {
            BaseTypes::Null
        }
    }

    impl From<BaseTypes> for usize {
        fn from(value: BaseTypes) -> Self {
            match value {
                BaseTypes::Int(i) => i as usize,
                BaseTypes::Float(f) => f as usize,
                _ => 0,
            }
        }
    }

    impl From<BaseTypes> for i32 {
        fn from(value: BaseTypes) -> Self {
            match value {
                BaseTypes::Int(i) => i,
                BaseTypes::Float(f) => f as i32,
                _ => 0,
            }
        }
    }

    impl From<BaseTypes> for f64 {
        fn from(value: BaseTypes) -> Self {
            match value {
                BaseTypes::Float(f) => f,
                BaseTypes::Int(i) => i as f64,
                _ => 0.0,
            }
        }
    }

    impl From<&ASTNode> for BaseTypes {
        fn from(value: &ASTNode) -> Self {
            match value {
                ASTNode::Int(n) => BaseTypes::Int(n.value),
                ASTNode::Float(f) => BaseTypes::Float(f.value.into()),
                ASTNode::String(s) => BaseTypes::StringWrapper(s.value.clone()),
                ASTNode::Bool(b) => BaseTypes::Bool(b.value),
                ASTNode::Char(c) => BaseTypes::Char(c.value),
                _ => BaseTypes::Null,
            }
        }
    }

    impl From<ASTNode> for BaseTypes {
        fn from(value: ASTNode) -> Self {
            match value {
                ASTNode::Int(n) => BaseTypes::Int(n.value),
                ASTNode::Float(f) => BaseTypes::Float(f.value.into()),
                ASTNode::String(s) => BaseTypes::StringWrapper(s.value),
                ASTNode::Bool(b) => BaseTypes::Bool(b.value),
                ASTNode::Char(c) => BaseTypes::Char(c.value),
                _ => BaseTypes::Null,
            }
        }
    }

    impl From<BaseTypes> for String {
        fn from(value: BaseTypes) -> Self {
            match value {
                BaseTypes::StringWrapper(s) => s,
                BaseTypes::Int(i) => i.to_string(),
                BaseTypes::Float(f) => f.to_string(),
                BaseTypes::Bool(b) => b.to_string(),
                BaseTypes::Char(c) => c.to_string(),
                _ => {
                    println!(
                        "Warning: Value not able to be returned as string. Returning empty string."
                    );
                    String::new()
                }
            }
        }
    }

    impl From<BaseTypes> for bool {
        fn from(value: BaseTypes) -> Self {
            match value {
                BaseTypes::Bool(b) => b,
                BaseTypes::Int(1) => true,
                BaseTypes::Int(0) => false,
                _ => false,
            }
        }
    }

    impl From<BaseTypes> for char {
        fn from(value: BaseTypes) -> Self {
            match value {
                BaseTypes::Char(c) => c,
                _ => '\0',
            }
        }
    }

    impl From<BaseTypes> for () {
        fn from(_: BaseTypes) -> Self {
            ()
        }
    }

    impl fmt::Display for BaseTypes {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                BaseTypes::Int(i) => write!(f, "{}", i),
                BaseTypes::Float(flt) => write!(f, "{}", flt),
                BaseTypes::StringWrapper(s) => write!(f, "{}", s),
                BaseTypes::Bool(b) => write!(f, "{}", b),
                BaseTypes::Char(c) => write!(f, "{}", c),
                BaseTypes::Null => write!(f, "null"),
            }
        }
    }
}

pub mod base_variables {
    use super::base_types::BaseTypes;
    // Stores some basic variables that most PLs have like pi, e, etc.
    #[derive(Debug, Clone)]
    pub enum BaseVariables {
        Pi,
        E,
    }

    pub struct Pi {
        pub value: f64,
    }

    impl Pi {
        pub fn new() -> Pi {
            Pi {
                value: std::f64::consts::PI,
            }
        }

        pub fn get_value(&self) -> f64 {
            self.value
        }

        pub fn get_type(&self) -> BaseTypes {
            BaseTypes::Float(self.value)
        }

        pub fn print(&self) {
            println!("Pi: {}", self.value);
        }
    }

    pub struct E {
        pub value: f64,
    }

    impl E {
        pub fn new() -> E {
            E {
                value: std::f64::consts::E,
            }
        }

        pub fn get_value(&self) -> f64 {
            self.value
        }

        pub fn get_type(&self) -> BaseTypes {
            BaseTypes::Float(self.value)
        }

        pub fn print(&self) {
            println!("E: {}", self.value);
        }
    }
}

pub mod base_types {
    //use std::fmt;

    #[derive(Debug, Clone)]
    pub enum BaseTypes {
        Int(i32),
        Float(f64),
        StringWrapper(String),
        Bool(bool),
        Char(char),
        Null,
    }

    pub trait GetType {
        fn GetType(&self) -> String;
    }

    impl GetType for BaseTypes {
        fn GetType(&self) -> String {
            match self {
                BaseTypes::Int(_) => "Int".to_string(),
                BaseTypes::Float(_) => "Float".to_string(),
                BaseTypes::StringWrapper(_) => "String".to_string(),
                BaseTypes::Bool(_) => "Bool".to_string(),
                BaseTypes::Char(_) => "Char".to_string(),
                BaseTypes::Null => "Null".to_string(),
            }
        }
    }

    pub struct Int {
        pub value: i32,
    }

    impl Int {
        pub fn new(value: i32) -> Int {
            Int { value: value }
        }
    }

    pub struct Float {
        pub value: f64,
    }

    impl Float {
        pub fn new(value: f64) -> Float {
            Float { value: value }
        }
    }

    pub struct StringWrapper {
        pub value: String,
    }

    impl StringWrapper {
        pub fn new(value: String) -> StringWrapper {
            StringWrapper { value: value }
        }
    }

    pub struct Char {
        pub value: char,
    }

    impl Char {
        pub fn new(value: char) -> Char {
            Char { value: value }
        }
    }

    pub struct Bool {
        pub value: bool,
    }

    impl Bool {
        pub fn new(value: bool) -> Bool {
            Bool { value: value }
        }
    }

    pub struct Null {
        pub value: (),
    }

    impl Null {
        pub fn new() -> Null {
            Null { value: () }
        }
    }
}
