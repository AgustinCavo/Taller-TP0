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

    fn make_operation(&mut self) -> i16 {
        if self.operands[0] == self.operands[1] {
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
    fn get_operands(&self) -> &Vec<String> {
        &self.operands
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

    fn make_operation(&mut self) -> i16 {
        if self.operands[0] > self.operands[1] {
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
    fn get_operands(&self) -> &Vec<String> {
        &self.operands
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

    fn make_operation(&mut self) -> i16 {
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
    fn get_operands(&self) -> &Vec<String> {
        &self.operands
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

    fn make_operation(&mut self) -> i16 {
        let operand1 = match self.operands[0].parse::<i16>() {
            Ok(num) => num,
            Err(_) => 0,
        };
        let operand2 = match self.operands[1].parse::<i16>() {
            Ok(num) => num,
            Err(_) => 0,
        };
        let result = operand1 & operand2;
        if result != 0 {
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
    fn get_operands(&self) -> &Vec<String> {
        &self.operands
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

    fn make_operation(&mut self) -> i16 {
        if self.operands[0] == "-1" || self.operands[1] == "-1" {
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
    fn get_operands(&self) -> &Vec<String> {
        &self.operands
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

    fn make_operation(&mut self) -> i16 {
        let operand1 = match self.operands[0].parse::<i16>() {
            Ok(num) => num,
            Err(_) => 0,
        };

        let operand2 = match self.operands[1].parse::<i16>() {
            Ok(num) => num,
            Err(_) => 0,
        };

        let result = operand1 | operand2;

        if result != 0 {
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
    fn get_operands(&self) -> &Vec<String> {
        &self.operands
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
