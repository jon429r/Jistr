use crate::base_variable::base_types::BaseTypes;
use crate::collection::collections::Array;
use crate::collection::collections::Dictionary;
use lazy_static::lazy_static;

use crate::function::functions::FunctionTypes;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    pub static ref ARRAY_STACK: Mutex<Vec<Array>> = Mutex::new(Vec::new());
    pub static ref DICTIONARY_STACK: Mutex<Vec<Dictionary>> = Mutex::new(Vec::new());
}

lazy_static! {
    pub static ref ARRAY_FUNCTIONS: Mutex<HashMap<&'static str, FunctionTypes>> = {
        let mut map = HashMap::new();

        map.insert("push", FunctionTypes::ArrayPushFn(array_push));
        map.insert("pop", FunctionTypes::ArrayPopFn(array_pop));
        map.insert("remove", FunctionTypes::ArrayRemoveFn(array_remove));
        map.insert("get", FunctionTypes::ArrayGetFn(array_get));
        map.insert("set", FunctionTypes::ArraySetFn(array_set));
        map.insert("append", FunctionTypes::ArrayAppendFn(array_append));
        map.insert("print", FunctionTypes::ArrayPrint(array_print));

        map.into()
    };
}
lazy_static! {
    pub static ref DICTIONARY_FUNCTIONS: Mutex<HashMap<&'static str, FunctionTypes>> = {
        let mut map = HashMap::new();

        map.insert("add", FunctionTypes::DictionaryAddFn(dictionary_add));
        map.insert(
            "remove",
            FunctionTypes::DictionaryRemoveFn(dictionary_remove),
        );
        map.insert("get", FunctionTypes::DictionaryGetFn(dictionary_get));
        map.insert("set", FunctionTypes::DictionarySetFn(dictionary_set));
        map.insert("keys", FunctionTypes::DictionaryKeysFn(dictionary_keys));
        map.insert(
            "values",
            FunctionTypes::DictionaryValuesFn(dictionary_values),
        );
        map.insert("print", FunctionTypes::DictionaryPrint(dictionary_print));
        map.into()
    };
}

pub fn update_array_stack(array: Array) {
    //remove array from stack then add it back
    let mut array_stack = ARRAY_STACK.lock().unwrap();
    let mut index = None;
    for (i, a) in array_stack.iter().enumerate() {
        if a.name == array.name {
            index = Some(i);
            break;
        }
    }
    if let Some(i) = index {
        array_stack.remove(i);
    }
    array_stack.push(array);
}

// Function to push an element into the array
fn array_push(array: &mut Array, value: BaseTypes) {
    let mut array_functions = ArrayFunctions::Push;
    array_functions.push(array, value);
}

fn array_append(array: &mut Array, value: BaseTypes) {
    let mut array_functions = ArrayFunctions::Append;
    array_functions.append(array, value);
}

// Function to pop an element from the array
fn array_pop(array: &mut Array) -> Option<BaseTypes> {
    let mut array_functions = ArrayFunctions::Pop;
    array_functions.pop(array)
}

// Function to remove an element from the array by index
fn array_remove(array: &mut Array, index: BaseTypes) {
    let mut array_functions = ArrayFunctions::Remove;
    array_functions.remove(array, index.into());
}

// Function to get an element from the array by index
fn array_get(array: &Array, index: BaseTypes) -> Option<BaseTypes> {
    let index: usize = index.into();
    let mut array_functions = ArrayFunctions::Get;
    array_functions.get(array, index)
}

// Function to set an element in the array at the specified index
fn array_set(array: &mut Array, index: BaseTypes, value: BaseTypes) -> Option<BaseTypes> {
    let index: usize = index.into();
    println!("Index: {}", index);
    let mut array_functions = ArrayFunctions::Set;
    array_functions.set(array, index, value)
}

fn array_print(array: &Array) {
    println!("{}", array);
}

pub enum ArrayFunctions {
    Push,
    Pop,
    Append,
    Remove,
    Get,
    Set,
    Print,
}

impl ArrayFunctions {
    pub fn push(&mut self, array: &mut Array, value: BaseTypes) {
        array.push(value.clone());
        update_array_stack(array.clone());
    }
    pub fn pop(&mut self, array: &mut Array) -> Option<BaseTypes> {
        let result = array.pop();
        update_array_stack(array.clone());
        result
    }
    pub fn append(&mut self, array: &mut Array, value: BaseTypes) {
        array.append(value);
        update_array_stack(array.clone());
    }
    pub fn remove(&mut self, array: &mut Array, index: usize) {
        array.remove(index);
        update_array_stack(array.clone());
    }
    pub fn get(&mut self, array: &Array, index: usize) -> Option<BaseTypes> {
        array.get(index)
    }
    pub fn set(&mut self, array: &mut Array, index: usize, value: BaseTypes) -> Option<BaseTypes> {
        println!("Index: {}", index);
        array.set(index, value.clone());
        update_array_stack(array.clone());
        Some(value)
    }
    pub fn print(&mut self, array: &Array) {
        println!("{}", array);
    }
}

pub enum DictionaryFunctions {
    Add,
    Remove,
    Get,
    Set,
    Keys,
    Values,
    Print,
}

pub fn update_dictionary_stack(dictionary: Dictionary) {
    //remove dictionary from stack then add it back
    let mut dictionary_stack = DICTIONARY_STACK.lock().unwrap();
    let mut index = None;
    for (i, d) in dictionary_stack.iter().enumerate() {
        if d.name == dictionary.name {
            index = Some(i);
            break;
        }
    }
    if let Some(i) = index {
        dictionary_stack.remove(i);
    }
    dictionary_stack.push(dictionary);
}

pub fn dictionary_add(dictionary: &mut Dictionary, key: BaseTypes, value: BaseTypes) {
    let mut dictionary_functions = DictionaryFunctions::Add;
    dictionary_functions.add(dictionary, key, value);
}

pub fn dictionary_remove(dictionary: &mut Dictionary, key: BaseTypes) {
    let mut dictionary_functions = DictionaryFunctions::Remove;
    dictionary_functions.remove(dictionary, key);
}

pub fn dictionary_get(dictionary: &Dictionary, key: BaseTypes) -> Option<(BaseTypes, BaseTypes)> {
    let mut dictionary_functions = DictionaryFunctions::Get;
    dictionary_functions.get(dictionary, key)
}

pub fn dictionary_set(dictionary: &mut Dictionary, key: BaseTypes, value: BaseTypes) {
    let mut dictionary_functions = DictionaryFunctions::Set;
    dictionary_functions.set(dictionary, key, value);
}

pub fn dictionary_keys(dictionary: &Dictionary) -> Vec<BaseTypes> {
    let mut dictionary_functions = DictionaryFunctions::Keys;
    dictionary_functions.keys(dictionary)
}

pub fn dictionary_values(dictionary: &Dictionary) -> Vec<BaseTypes> {
    let mut dictionary_functions = DictionaryFunctions::Values;
    dictionary_functions.values(dictionary)
}

pub fn dictionary_print(dictionary: &Dictionary) {
    println!("{}", dictionary);
}

impl DictionaryFunctions {
    pub fn add(&mut self, dictionary: &mut Dictionary, key: BaseTypes, value: BaseTypes) {
        println!("Add : {:?}", dictionary.add(key.clone(), value.clone()));
        dictionary.add(key, value);
        update_dictionary_stack(dictionary.clone());
    }

    pub fn remove(&mut self, dictionary: &mut Dictionary, key: BaseTypes) {
        dictionary.remove(key);
        update_dictionary_stack(dictionary.clone());
    }

    pub fn get(
        &mut self,
        dictionary: &Dictionary,
        key: BaseTypes,
    ) -> Option<(BaseTypes, BaseTypes)> {
        println!("Get : {:?}", dictionary.get(key.clone()));
        dictionary.get(key).cloned()
    }

    pub fn set(&mut self, dictionary: &mut Dictionary, key: BaseTypes, value: BaseTypes) {
        dictionary.set(key, value);
        update_dictionary_stack(dictionary.clone());
    }

    pub fn keys(&mut self, dictionary: &Dictionary) -> Vec<BaseTypes> {
        println!("Keys : {:?}", dictionary.keys());
        dictionary.keys().into_iter().cloned().collect()
    }

    pub fn values(&mut self, dictionary: &Dictionary) -> Vec<BaseTypes> {
        println!("Values : {:?}", dictionary.values());
        dictionary.values().into_iter().cloned().collect()
    }

    pub fn print(&mut self, dictionary: &Dictionary) {
        println!("{}", dictionary);
    }
}

pub mod collections {
    use crate::base_variable::base_types::BaseTypes;
    use std::fmt;

    #[derive(Clone, Debug)]
    pub struct Array {
        pub name: String,
        pub data: Vec<BaseTypes>,
        pub value_type: BaseTypes,
    }

    impl fmt::Display for Array {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}: Array<{}> = [", self.name, self.value_type)?;
            for (i, value) in self.data.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", value)?;
            }
            write!(f, "]")
        }
    }

    // functions for arrays: new, push, pop, remove, get(i), set(i), to_string
    impl Array {
        pub fn new(name: String, value_type: BaseTypes, data: Vec<BaseTypes>) -> Array {
            Array {
                name,
                data,
                value_type,
            }
        }

        pub fn push(&mut self, value: BaseTypes) {
            self.data.push(value);
        }

        pub fn pop(&mut self) -> Option<BaseTypes> {
            self.data.pop()
        }

        pub fn append(&mut self, value: BaseTypes) {
            // Push the value to the existing data vector
            self.data.push(value);
        }

        pub fn remove(&mut self, index: usize) {
            if index < self.data.len() {
                Some(self.data.remove(index));
            } else {
                println!("Syntax Error, during removal")
            }
        }

        pub fn get(&self, index: usize) -> Option<BaseTypes> {
            return self.data.get(index).clone().cloned();
        }

        pub fn set(&mut self, index: usize, value: BaseTypes) -> Option<BaseTypes> {
            if index < self.data.len() {
                Some(std::mem::replace(&mut self.data[index], value))
            } else {
                None
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Dictionary {
        pub name: String,
        pub values: Vec<(BaseTypes, BaseTypes)>,
        pub types: (BaseTypes, BaseTypes),
    }

    impl fmt::Display for Dictionary {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{}: Dict<{}, {}> = {{",
                self.name, self.types.0, self.types.1
            )?;
            for (i, (key, value)) in self.values.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "\"{}\" => {}", key, value)?;
            }
            write!(f, "}}")
        }
    }

    // functions for dictionaries: new, add, remove, get, set(key), keys, values, to_string

    impl Dictionary {
        pub fn new(
            name: String,
            key_type: BaseTypes,
            value_type: BaseTypes,
            values: Vec<(BaseTypes, BaseTypes)>,
        ) -> Dictionary {
            Dictionary {
                name,
                values,
                types: (key_type, value_type),
            }
        }

        pub fn add(&mut self, key: BaseTypes, value: BaseTypes) {
            self.values.push((key, value));
        }

        /*pub fn append(&mut self, key: ASTNode, value: ASTNode) {
            // add to end of dict
            self.values.add((key, value));
        } */

        pub fn remove(&mut self, key: BaseTypes) -> Option<(BaseTypes, BaseTypes)> {
            let mut index = None;
            for (i, (k, _)) in self.values.iter().enumerate() {
                if k == &key {
                    index = Some(i);
                    break;
                }
            }
            if let Some(i) = index {
                Some(self.values.remove(i))
            } else {
                None
            }
        }

        pub fn get(&self, key: BaseTypes) -> Option<&(BaseTypes, BaseTypes)> {
            self.values.iter().find(|(k, _)| k == &key)
        }

        pub fn set(&mut self, key: BaseTypes, value: BaseTypes) -> Option<BaseTypes> {
            let mut index = None;
            for (i, (k, _)) in self.values.iter().enumerate() {
                if k == &key {
                    index = Some(i);
                    break;
                }
            }
            if let Some(i) = index {
                Some(std::mem::replace(&mut self.values[i].1, value))
            } else {
                None
            }
        }

        pub fn keys(&self) -> Vec<&BaseTypes> {
            self.values.iter().map(|(k, _)| k).collect()
        }

        pub fn values(&self) -> Vec<&BaseTypes> {
            self.values.iter().map(|(_, v)| v).collect()
        }

        /*pub fn to_string(&self) -> String {
                    let mut output = String::new();
                    output.push_str(&self.name);
                    output.push_str(" = {");
                    for (i, (key, value)) in self.values.iter().enumerate() {
                        if i > 0 {
                            output.push_str(", ");
                        }
                        output.push_str(&key.to_string());
                        output.push_str("=>");
                        output.push_str(&value.to_string());
                    }
                    output.push_str("}");
                    output
                }
        */
    }
}
