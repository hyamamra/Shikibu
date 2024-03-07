#[derive(Debug)]
pub enum Node {
    Number(String),
    String(String),
    True,
    False,
    Null,
    Variable(String),
    Assignment {
        name: String,
        value: Box<Node>,
    },
    /// syntax is `しごと` (name) `（` (args) `）` (body)
    Function {
        name: String,
        args: Vec<Node>,
    },
    Call {
        function: Box<Node>,
        args: Vec<Node>,
    },
    Addition {
        left: Box<Node>,
        right: Box<Node>,
    },
    Subtraction {
        left: Box<Node>,
        right: Box<Node>,
    },
    Multiplication {
        left: Box<Node>,
        right: Box<Node>,
    },
    Division {
        left: Box<Node>,
        right: Box<Node>,
    },
    /// syntax is `もし` (condition) `なら` (then_part) `ちがえば` (else_part)
    If {
        condition: Box<Node>,
        then_part: Box<Node>,
        else_part: Box<Node>,
    },
    /// syntax is `くりかえす` (body)
    Loop {
        body: Vec<Node>,
    },
    /// syntax is `かえす` (value)
    Return(Box<Node>),
    /// syntax is `ぬける`
    Break,
    /// syntax is `つぎへ`
    Continue,
}
