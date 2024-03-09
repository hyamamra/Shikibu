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
        let mut runner = Self {
            ast,
            functions: HashMap::new(),
            variables: HashMap::new(),
        };
        runner.drain_functions();
        runner.run_ast().unwrap();
        Ok(())
    }

    fn run_ast(&mut self) -> Result<(), RuntimeError> {
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
            Node::Variable(variable) => self.variables.get(&variable).unwrap().clone(),
            Node::Call { name, args } => todo!(),
            Node::Or { left, right } => todo!(),
            Node::And { left, right } => todo!(),
            Node::Equal { left, right } => todo!(),
            Node::NotEqual { left, right } => todo!(),
            Node::LessThan { left, right } => todo!(),
            Node::LessThanOrEqual { left, right } => todo!(),
            Node::GreaterThan { left, right } => todo!(),
            Node::GreaterThanOrEqual { left, right } => todo!(),
            Node::Add { left, right } => todo!(),
            Node::Subtract { left, right } => todo!(),
            Node::Multiply { left, right } => todo!(),
            Node::Divide { left, right } => todo!(),
            _ => return Err(RuntimeError::unexpected_node(value)),
        })
    }

    fn drain_functions(&mut self) {
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
                panic!("Expected function, found {:?}", function);
            }
        }
    }
}

fn string_to_number(string: &str) -> f64 {
    to_half_size_number(string).parse().unwrap()
}

fn to_half_size_number(zenkaku_num: &str) -> String {
    zenkaku_num
        .chars()
        .map(|c| match c {
            '０'..='９' => (c as u8 - '０' as u8 + b'0') as char,
            '．' => '.',
            _ => c,
        })
        .collect()
}
