use crate::operations::operation_trait::Operation;
use std::collections::HashMap;
use std::usize;

pub struct Dup {
    pub name: String,
    pub quantity: usize,
    pub operands: Vec<String>,
}
pub struct Drop {
    pub name: String,
    pub quantity: usize,
    pub operands: Vec<String>,
}
pub struct Swap {
    pub name: String,
    pub quantity: usize,
    pub operands: Vec<String>,
}
pub struct Over {
    pub name: String,
    pub quantity: usize,
    pub operands: Vec<String>,
}
pub struct Rot {
    pub name: String,
    pub quantity: usize,
    pub operands: Vec<String>,
}

impl Operation for Dup {
    fn add_operand(&mut self, element: i16) -> bool {
        self.operands.push(element.to_string());
        return true;
    }
    fn make_operation(&mut self) -> i16 {
        println!("{}", &self.operands[0]);
        match self.operands[0].parse::<i16>() {
            Ok(resuslt) => {
                return resuslt;
            }
            Err(_) => {
                return 0;
            }
        }
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
    fn get_operands(&self) -> &[String] {
        &self.operands
    }
}

impl Operation for Drop {
    fn add_operand(&mut self, element: i16) -> bool {
        self.operands.push(element.to_string());
        return true;
    }
    fn make_operation(&mut self) -> i16 {
        println!("{}", &self.operands[0]);
        match self.operands[0].parse::<i16>() {
            Ok(resuslt) => {
                return resuslt;
            }
            Err(_) => {
                return 0;
            }
        }
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
    fn get_operands(&self) -> &[String] {
        &self.operands
    }
}

impl Operation for Swap {
    fn add_operand(&mut self, element: i16) -> bool {
        self.operands.push(element.to_string());
        return true;
    }
    fn make_operation(&mut self) -> i16 {
        println!("{}", &self.operands[0]);
        match self.operands[0].parse::<i16>() {
            Ok(resuslt) => {
                return resuslt;
            }
            Err(_) => {
                return 0;
            }
        }
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
    fn get_operands(&self) -> &[String] {
        &self.operands
    }
}

impl Operation for Rot {
    fn add_operand(&mut self, element: i16) -> bool {
        self.operands.push(element.to_string());
        return true;
    }
    fn make_operation(&mut self) -> i16 {
        println!("{}", &self.operands[0]);
        match self.operands[0].parse::<i16>() {
            Ok(resuslt) => {
                return resuslt;
            }
            Err(_) => {
                return 0;
            }
        }
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
    fn get_operands(&self) -> &[String] {
        &self.operands
    }
}

impl Operation for Over {
    fn add_operand(&mut self, element: i16) -> bool {
        self.operands.push(element.to_string());
        return true;
    }
    fn make_operation(&mut self) -> i16 {
        println!("{}", &self.operands[0]);
        match self.operands[0].parse::<i16>() {
            Ok(resuslt) => {
                return resuslt;
            }
            Err(_) => {
                return 0;
            }
        }
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
    fn get_operands(&self) -> &[String] {
        &self.operands
    }
}

pub fn load_stack_operations_map() -> HashMap<String, Box<dyn Fn() -> Box<dyn Operation>>> {
    let mut stack_operations: HashMap<String, Box<dyn Fn() -> Box<dyn Operation>>> = HashMap::new();

    stack_operations.insert(
        "drop".to_string(),
        Box::new(|| {
            Box::new(Drop {
                name: { "drop" }.to_string(),
                quantity: 1,
                operands: Vec::new(),
            })
        }),
    );

    stack_operations.insert(
        "dup".to_string(),
        Box::new(|| {
            Box::new(Dup {
                name: { "dup" }.to_string(),
                quantity: 1,
                operands: Vec::new(),
            })
        }),
    );

    stack_operations.insert(
        "swap".to_string(),
        Box::new(|| {
            Box::new(Swap {
                name: { "swap" }.to_string(),
                quantity: 2,
                operands: Vec::new(),
            })
        }),
    );
    stack_operations.insert(
        "over".to_string(),
        Box::new(|| {
            Box::new(Over {
                name: { "over" }.to_string(),
                quantity: 2,
                operands: Vec::new(),
            })
        }),
    );
    stack_operations.insert(
        "rot".to_string(),
        Box::new(|| {
            Box::new(Rot {
                name: { "rot" }.to_string(),
                quantity: 3,
                operands: Vec::new(),
            })
        }),
    );
    stack_operations
}
