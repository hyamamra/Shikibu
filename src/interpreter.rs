use self::{
    ast::{node::Node, Ast},
    error::RuntimeError,
};
use std::collections::{HashMap, VecDeque};

pub mod ast;
pub mod error;
pub mod tokens;

#[derive(Debug, Clone)]
enum Value {
    Number(f64),
    String(String),
    True,
    False,
    Null,
}

#[derive(Debug)]
pub struct Interpreter {
    ast: Ast,
    functions: HashMap<String, Node>,
    variables: HashMap<String, Value>,
}

impl Interpreter {
    pub fn run(ast: Ast) -> Result<(), RuntimeError> {
        let mut interpreter = Self {
            ast,
            functions: HashMap::new(),
            variables: HashMap::new(),
        };
        interpreter.drain_functions().unwrap();
        interpreter.run_children().unwrap();
        Ok(())
    }

    fn run_children(&mut self) -> Result<(), RuntimeError> {
        while !self.ast.is_empty() {
            let node = self.ast.pop_front().unwrap();
            match node {
                Node::Assignment { name, value } => self.assign_variable(name, *value).unwrap(),
                Node::Call { name, args } => todo!(),
                Node::If {
                    condition,
                    then_part,
                    else_part,
                } => todo!(),
                Node::Loop { body } => todo!(),
                Node::Print(value) => {
                    let value = self.calculate(*value).unwrap();
                    self.print(&value);
                }
                _ => return Err(RuntimeError::unexpected_node(node)),
            }
        }
        Ok(())
    }

    fn run_function(&mut self) -> Result<Value, RuntimeError> {
        while !self.ast.is_empty() {
            let node = self.ast.pop_front().unwrap();
            match node {
                Node::Assignment { name, value } => self.assign_variable(name, *value).unwrap(),
                Node::Call { name, args } => todo!(),
                Node::If {
                    condition,
                    then_part,
                    else_part,
                } => todo!(),
                Node::Loop { body } => todo!(),
                Node::Return(value) => return self.calculate(*value),
                Node::Print(value) => {
                    let value = self.calculate(*value).unwrap();
                    self.print(&value);
                }
                _ => return Err(RuntimeError::unexpected_node(node)),
            }
        }
        Ok(Value::Null)
    }

    fn print(&self, value: &Value) {
        match value {
            Value::Number(number) => println!("{}", number),
            Value::String(string) => println!("{}", string),
            Value::True => println!("真"),
            Value::False => println!("偽"),
            Value::Null => println!("無"),
        }
    }

    fn assign_variable(&mut self, name: String, value: Node) -> Result<(), RuntimeError> {
        let value = self.calculate(value);
        self.variables.insert(name, value.unwrap());
        Ok(())
    }

    fn calculate(&self, value: Node) -> Result<Value, RuntimeError> {
        Ok(match value {
            Node::Number(number) => Value::Number(string_to_number(&number)),
            Node::String(string) => Value::String(string),
            Node::True => Value::True,
            Node::False => Value::False,
            Node::Null => Value::Null,
            Node::Variable(ref variable) => {
                if let Some(value) = self.variables.get(variable) {
                    value.clone()
                } else {
                    return Err(RuntimeError::undefined_variable(value));
                }
            }
            Node::Call { ref name, ref args } => {
                let Some(function) = self.functions.get(name) else {
                    return Err(RuntimeError::undefined_function(value));
                };

                let Node::Function {
                    name: _,
                    params,
                    body,
                } = function.clone()
                else {
                    panic!()
                };

                if params.len() != args.len() {
                    return Err(RuntimeError::wrong_number_of_arguments(value));
                }

                let mut variables = self.variables.clone();
                for (param, arg) in params.iter().zip(args.iter()) {
                    variables.insert(param.clone(), self.calculate(arg.clone()).unwrap());
                }

                let mut interpreter = Self {
                    ast: Ast::new(body),
                    functions: self.functions.clone(),
                    variables,
                };

                interpreter.run_function().unwrap()
            }
            Node::Or {
                ref left,
                ref right,
            } => {
                let left = self.calculate(*left.clone()).unwrap();
                let right = self.calculate(*right.clone()).unwrap();
                match (left, right) {
                    (Value::True, _) => Value::True,
                    (_, Value::True) => Value::True,
                    _ => Value::False,
                }
            }
            Node::And {
                ref left,
                ref right,
            } => {
                let left = self.calculate(*left.clone()).unwrap();
                let right = self.calculate(*right.clone()).unwrap();
                match (left, right) {
                    (Value::False, _) => Value::False,
                    (_, Value::False) => Value::False,
                    _ => Value::True,
                }
            }
            Node::Equal {
                ref left,
                ref right,
            } => {
                let left = self.calculate(*left.clone()).unwrap();
                let right = self.calculate(*right.clone()).unwrap();
                match (left, right) {
                    (Value::Number(left), Value::Number(right)) => {
                        if left == right {
                            Value::True
                        } else {
                            Value::False
                        }
                    }
                    (Value::String(left), Value::String(right)) => {
                        if left == right {
                            Value::True
                        } else {
                            Value::False
                        }
                    }
                    _ => return Err(RuntimeError::comparing_different_types(value)),
                }
            }
            Node::NotEqual {
                ref left,
                ref right,
            } => {
                let left = self.calculate(*left.clone()).unwrap();
                let right = self.calculate(*right.clone()).unwrap();
                match (left, right) {
                    (Value::Number(left), Value::Number(right)) => {
                        if left != right {
                            Value::True
                        } else {
                            Value::False
                        }
                    }
                    (Value::String(left), Value::String(right)) => {
                        if left != right {
                            Value::True
                        } else {
                            Value::False
                        }
                    }
                    _ => return Err(RuntimeError::comparing_different_types(value)),
                }
            }
            Node::LessThan {
                ref left,
                ref right,
            } => {
                let left = self.calculate(*left.clone()).unwrap();
                let right = self.calculate(*right.clone()).unwrap();
                match (left, right) {
                    (Value::Number(left), Value::Number(right)) => {
                        if left < right {
                            Value::True
                        } else {
                            Value::False
                        }
                    }
                    _ => return Err(RuntimeError::comparing_different_types(value)),
                }
            }
            Node::LessThanOrEqual {
                ref left,
                ref right,
            } => {
                let left = self.calculate(*left.clone()).unwrap();
                let right = self.calculate(*right.clone()).unwrap();
                match (left, right) {
                    (Value::Number(left), Value::Number(right)) => {
                        if left <= right {
                            Value::True
                        } else {
                            Value::False
                        }
                    }
                    _ => return Err(RuntimeError::comparing_different_types(value)),
                }
            }
            Node::GreaterThan {
                ref left,
                ref right,
            } => {
                let left = self.calculate(*left.clone()).unwrap();
                let right = self.calculate(*right.clone()).unwrap();
                match (left, right) {
                    (Value::Number(left), Value::Number(right)) => {
                        if left > right {
                            Value::True
                        } else {
                            Value::False
                        }
                    }
                    _ => return Err(RuntimeError::comparing_different_types(value)),
                }
            }
            Node::GreaterThanOrEqual {
                ref left,
                ref right,
            } => {
                let left = self.calculate(*left.clone()).unwrap();
                let right = self.calculate(*right.clone()).unwrap();
                match (left, right) {
                    (Value::Number(left), Value::Number(right)) => {
                        if left >= right {
                            Value::True
                        } else {
                            Value::False
                        }
                    }
                    _ => return Err(RuntimeError::comparing_different_types(value)),
                }
            }
            Node::Add {
                ref left,
                ref right,
            } => {
                let left = self.calculate(*left.clone()).unwrap();
                let right = self.calculate(*right.clone()).unwrap();
                match (left, right) {
                    (Value::Number(left), Value::Number(right)) => Value::Number(left + right),
                    (Value::String(left), Value::String(right)) => Value::String(left + &right),
                    _ => return Err(RuntimeError::string_addition(value)),
                }
            }
            Node::Subtract {
                ref left,
                ref right,
            } => {
                let left = self.calculate(*left.clone()).unwrap();
                let right = self.calculate(*right.clone()).unwrap();
                match (left, right) {
                    (Value::Number(left), Value::Number(right)) => Value::Number(left - right),
                    _ => return Err(RuntimeError::string_addition(value)),
                }
            }
            Node::Multiply {
                ref left,
                ref right,
            } => {
                let left = self.calculate(*left.clone()).unwrap();
                let right = self.calculate(*right.clone()).unwrap();
                match (left, right) {
                    (Value::Number(left), Value::Number(right)) => Value::Number(left * right),
                    _ => return Err(RuntimeError::string_addition(value)),
                }
            }
            Node::Divide {
                ref left,
                ref right,
            } => {
                let left = self.calculate(*left.clone()).unwrap();
                let right = self.calculate(*right.clone()).unwrap();
                match (left, right) {
                    (Value::Number(left), Value::Number(right)) => Value::Number(left / right),
                    _ => return Err(RuntimeError::string_addition(value)),
                }
            }
            _ => return Err(RuntimeError::unexpected_node(value)),
        })
    }

    fn drain_functions(&mut self) -> Result<(), RuntimeError> {
        let mut functions = VecDeque::from(self.ast.drain_functions());
        while !functions.is_empty() {
            let function = functions.pop_front().unwrap();
            if let Node::Function {
                ref name,
                params: _,
                body: _,
            } = function
            {
                self.functions.insert(name.clone(), function);
            } else {
                return Err(RuntimeError::unexpected_node(function));
            }
        }
        Ok(())
    }
}

fn string_to_number(string: &str) -> f64 {
    string
        .chars()
        .map(|c| match c {
            '０'..='９' => (c as u8 - '０' as u8 + b'0') as char,
            '．' => '.',
            _ => c,
        })
        .collect::<String>()
        .parse()
        .unwrap()
}
