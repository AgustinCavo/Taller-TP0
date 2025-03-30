use crate::operations::operation_trait::Operation;
use std::collections::HashMap;
const SIMPLE_OPERATION: usize = 2;
pub struct Grt {
    pub name: String,
    pub quantity: usize,
    pub operands: Vec<String>,
}
pub struct Lst {
    pub name: String,
    pub quantity: usize,
    pub operands: Vec<String>,
}
pub struct Equ {
    pub name: String,
    pub quantity: usize,
    pub operands: Vec<String>,
}
pub struct Or {
    pub name: String,
    pub quantity: usize,
    pub operands: Vec<String>,
}
pub struct And {
    pub name: String,
    pub quantity: usize,
    pub operands: Vec<String>,
}
pub struct Not {
    pub name: String,
    pub quantity: usize,
    pub operands: Vec<String>,
}

impl Operation for Equ {
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
impl Operation for Grt {
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
impl Operation for Lst {
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
impl Operation for And {
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
impl Operation for Or {
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
impl Operation for Not {
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

pub fn load_conditional_operations_map() -> HashMap<String, Box<dyn Fn() -> Box<dyn Operation>>> {
    let mut stack_operations: HashMap<String, Box<dyn Fn() -> Box<dyn Operation>>> = HashMap::new();

    stack_operations.insert(
        "=".to_string(),
        Box::new(|| {
            Box::new(Equ {
                name: { "=" }.to_string(),
                quantity: 2,
                operands: Vec::new(),
            })
        }),
    );

    stack_operations.insert(
        "<".to_string(),
        Box::new(|| {
            Box::new(Grt {
                name: { "<" }.to_string(),
                quantity: 2,
                operands: Vec::new(),
            })
        }),
    );

    stack_operations.insert(
        ">".to_string(),
        Box::new(|| {
            Box::new(Lst {
                name: { ">" }.to_string(),
                quantity: 2,
                operands: Vec::new(),
            })
        }),
    );
    stack_operations.insert(
        "and".to_string(),
        Box::new(|| {
            Box::new(And {
                name: { "and" }.to_string(),
                quantity: 2,
                operands: Vec::new(),
            })
        }),
    );
    stack_operations.insert(
        "or".to_string(),
        Box::new(|| {
            Box::new(Or {
                name: { "or" }.to_string(),
                quantity: 2,
                operands: Vec::new(),
            })
        }),
    );
    stack_operations.insert(
        "not".to_string(),
        Box::new(|| {
            Box::new(Not {
                name: { "not" }.to_string(),
                quantity: 1,
                operands: Vec::new(),
            })
        }),
    );
    stack_operations
}
