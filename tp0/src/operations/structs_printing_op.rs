use crate::operations::operation_trait::Operation;
use std::collections::HashMap;
const SIMPLE_OPERATION: usize = 2;

struct Dot {
    pub name: String,
    quantity: usize,
    operands: Vec<String>,
}
struct Emit {
    pub name: String,
    quantity: usize,
    operands: Vec<String>,
}
struct Quotes {
    pub name: String,
    quantity: usize,
    operands: Vec<String>,
}
impl Operation for Dot {
    fn add_operand(&mut self, element: i16) -> bool {
        if self.operands.len() < SIMPLE_OPERATION {
            self.operands.push(element.to_string());
            return true;
        }
        return false;
    }

    fn make_operation(&self) -> i16 {
        print!("{} ", self.operands[0]);
        return 0;
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
impl Operation for Emit {
    fn add_operand(&mut self, element: i16) -> bool {
        if self.operands.len() < SIMPLE_OPERATION {
            self.operands.push(element.to_string());
            return true;
        }
        return false;
    }

    fn make_operation(&self) -> i16 {
        return 0;
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

impl Operation for Quotes {
    fn add_operand(&mut self, element: i16) -> bool {
        if self.operands.len() < SIMPLE_OPERATION {
            self.operands.push(element.to_string());
            return true;
        }
        return false;
    }

    fn make_operation(&self) -> i16 {
        if self.operands[0] < self.operands[1] {
            return -1;
        }
        return 0;
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

pub fn load_printing_operations_map() -> HashMap<String, Box<dyn Fn() -> Box<dyn Operation>>> {
    let mut printing_operations: HashMap<String, Box<dyn Fn() -> Box<dyn Operation>>> =
        HashMap::new();

    printing_operations.insert(
        ".".to_string(),
        Box::new(|| {
            Box::new(Dot {
                name: { "." }.to_string(),
                quantity: 1,
                operands: Vec::new(),
            })
        }),
    );

    printing_operations.insert(
        "emit".to_string(),
        Box::new(|| {
            Box::new(Emit {
                name: { "emit" }.to_string(),
                quantity: 1,
                operands: Vec::new(),
            })
        }),
    );

    printing_operations.insert(
        "Quotes".to_string(),
        Box::new(|| {
            Box::new(Quotes {
                name: { "Quotes" }.to_string(),
                quantity: 0,
                operands: Vec::new(),
            })
        }),
    );
    printing_operations
}
