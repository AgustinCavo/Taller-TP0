use crate::operations::operation_trait::Operation;
use std::collections::HashMap;
use std::usize;
const SIMPLE_OPERATION: usize = 2;
//Operaciones del aritmeticas
pub struct Sum {
    pub name: String,
    pub quantity: usize,
    pub operands: Vec<i16>,
}
pub struct Sub {
    pub name: String,
    pub quantity: usize,
    pub operands: Vec<i16>,
}
pub struct Div {
    pub name: String,
    pub quantity: usize,
    pub operands: Vec<i16>,
}
pub struct Mul {
    pub name: String,
    pub quantity: usize,
    pub operands: Vec<i16>,
}

impl Operation for Sum {
    fn add_operand(&mut self, element: i16) -> bool {
        if self.operands.len() < SIMPLE_OPERATION {
            self.operands.push(element);
            return true;
        }
        return false;
    }

    fn make_operation(&self) -> i16 {
        return self.operands[0] + self.operands[1];
    }
    fn quantity(&self) -> usize {
        return self.quantity;
    }
    fn operands(&self) -> usize {
        return self.operands.len();
    }
    fn name(&self) -> &str {
        return &self.name;
    }
}

impl Operation for Sub {
    fn add_operand(&mut self, element: i16) -> bool {
        if self.operands.len() < SIMPLE_OPERATION {
            self.operands.push(element);
            return true;
        }
        return false;
    }
    fn make_operation(&self) -> i16 {
        return self.operands[1] - self.operands[0];
    }

    fn quantity(&self) -> usize {
        return self.quantity;
    }
    fn operands(&self) -> usize {
        return self.operands.len();
    }
    fn name(&self) -> &str {
        return &self.name;
    }
}

impl Operation for Mul {
    fn add_operand(&mut self, element: i16) -> bool {
        if self.operands.len() < SIMPLE_OPERATION {
            self.operands.push(element);
            return true;
        }
        return false;
    }
    fn make_operation(&self) -> i16 {
        return self.operands[1] * self.operands[0];
    }
    fn quantity(&self) -> usize {
        return self.quantity;
    }
    fn operands(&self) -> usize {
        return self.operands.len();
    }
    fn name(&self) -> &str {
        return &self.name;
    }
}

impl Operation for Div {
    fn add_operand(&mut self, element: i16) -> bool {
        if element == 0 {
            println!("division-by-zero");
            return false;
        } else {
            self.operands.push(element);
            return true;
        }
    }
    fn make_operation(&self) -> i16 {
        return self.operands[1] / self.operands[0];
    }
    fn quantity(&self) -> usize {
        return self.quantity;
    }
    fn operands(&self) -> usize {
        return self.operands.len();
    }
    fn name(&self) -> &str {
        return &self.name;
    }
}

pub fn load_basic_operations_map() -> HashMap<String, Box<dyn Fn() -> Box<dyn Operation>>> {
    let mut basic_operations: HashMap<String, Box<dyn Fn() -> Box<dyn Operation>>> = HashMap::new();

    basic_operations.insert(
        "+".to_string(),
        Box::new(|| {
            Box::new(Sum {
                name: { "+" }.to_string(),
                quantity: 2,
                operands: Vec::new(),
            })
        }),
    );
    basic_operations.insert(
        "-".to_string(),
        Box::new(|| {
            Box::new(Sub {
                name: { "-" }.to_string(),
                quantity: 2,
                operands: Vec::new(),
            })
        }),
    );
    basic_operations.insert(
        "/".to_string(),
        Box::new(|| {
            Box::new(Div {
                name: { "/" }.to_string(),
                quantity: 2,
                operands: Vec::new(),
            })
        }),
    );
    basic_operations.insert(
        "*".to_string(),
        Box::new(|| {
            Box::new(Mul {
                name: { "*" }.to_string(),
                quantity: 2,
                operands: Vec::new(),
            })
        }),
    );

    basic_operations
}
