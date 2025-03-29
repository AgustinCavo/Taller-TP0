use std::{path::StripPrefixError, usize};

const SIMPLE_OPERATION: usize = 2;


//Trait para las operaciones
pub trait Operation{
    fn add_operand(&mut self, element: i16) -> bool;
    fn make_operation(&self) -> i16;
    fn operands(&self) -> usize;
    fn quantity(&self) -> usize;
    fn name(&self)-> &str;
}


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
    fn name(&self)-> &str {
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
    fn name(&self)-> &str {
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
    fn name(&self)-> &str {
        return &self.name;
    }
}

impl Operation for Div {
    fn add_operand(&mut self, element: i16) -> bool {
        if element == 0 {
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
    fn name(&self)-> &str {
        return &self.name;
    }
}
//Operaciones del stack
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
    fn make_operation(&self) -> i16 {
        println!("{}",&self.operands[0]);
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
    fn name(&self)-> &str {
        return &self.name;
    }
}

impl Operation for Drop {
    fn add_operand(&mut self, element: i16) -> bool {
            self.operands.push(element.to_string());
            return true;
    }
    fn make_operation(&self) -> i16 {
        println!("{}",&self.operands[0]);
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
    fn name(&self)-> &str {
        return &self.name;
    }
}

impl Operation for Swap {
    fn add_operand(&mut self, element: i16) -> bool {
            self.operands.push(element.to_string());
            return true;
    }
    fn make_operation(&self) -> i16 {
        println!("{}",&self.operands[0]);
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
    fn name(&self)-> &str {
        return &self.name;
    }
}

impl Operation for Rot {
    fn add_operand(&mut self, element: i16) -> bool {
            self.operands.push(element.to_string());
            return true;
    }
    fn make_operation(&self) -> i16 {
        println!("{}",&self.operands[0]);
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
    fn name(&self)-> &str {
        return &self.name;
    }
}

impl Operation for Over {
    fn add_operand(&mut self, element: i16) -> bool {
            self.operands.push(element.to_string());
            return true;
    }
    fn make_operation(&self) -> i16 {
        println!("{}",&self.operands[0]);
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
    fn name(&self)-> &str {
        return &self.name;
    }
}
