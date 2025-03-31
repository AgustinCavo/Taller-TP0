use crate::operations::operation_trait::Operation;
use std::collections::HashMap;
use std::usize;
const SIMPLE_OPERATION: usize = 2;
//Operaciones del aritmeticas
pub struct Sum {
    pub name: String,
    pub quantity: usize,
    pub operands: Vec<String>,
}
pub struct Sub {
    pub name: String,
    pub quantity: usize,
    pub operands: Vec<String>,
}
pub struct Div {
    pub name: String,
    pub quantity: usize,
    pub operands: Vec<String>,
}
pub struct Mul {
    pub name: String,
    pub quantity: usize,
    pub operands: Vec<String>,
}

impl Operation for Sum {
    fn add_operand(&mut self, element: i16) -> bool {
        if self.operands.len() < SIMPLE_OPERATION {
            self.operands.push(element.to_string());
            return true;
        }
        return false;
    }

    fn make_operation(&mut self) -> i16 {
        match self.operands[0].parse::<i16>() {
            Ok(operand1) => {
                match self.operands[1].parse::<i16>() {
                    Ok(operand2) => operand1 + operand2,
                    Err(_) => { 
                        println!("Error al convertir los numeros de la operacion");
                        0
                    }
                }
            },
            Err(_) => {
                println!("Error al convertir los numeros de la operacion");
                0
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

impl Operation for Sub {
    fn add_operand(&mut self, element: i16) -> bool {
        if self.operands.len() < SIMPLE_OPERATION {
            self.operands.push(element.to_string());
            return true;
        }
        return false;
    }
    fn make_operation(&mut self) -> i16 {
        match self.operands[0].parse::<i16>() {
            Ok(operand1) => {
                match self.operands[1].parse::<i16>() {
                    Ok(operand2) => operand1 - operand2,
                    Err(_) => { 
                        println!("Error al convertir los numeros de la operacion");
                        0
                    }
                }
            },
            Err(_) => {
                println!("Error al convertir los numeros de la operacion");
                0
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

impl Operation for Mul {
    fn add_operand(&mut self, element: i16) -> bool {
        if self.operands.len() < SIMPLE_OPERATION {
            self.operands.push(element.to_string());
            return true;
        }
        return false;
    }
    fn make_operation(&mut self) -> i16 {
        match self.operands[0].parse::<i16>() {
            Ok(operand1) => {
                match self.operands[1].parse::<i16>() {
                    Ok(operand2) => operand1 * operand2,
                    Err(_) => { 
                        println!("Error al convertir los numeros de la operacion");
                        0
                    }
                }
            },
            Err(_) => {
                println!("Error al convertir los numeros de la operacion");
                0
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
impl Operation for Div {
    fn add_operand(&mut self, element: i16) -> bool {
        
        if element == 0 {
            println!("division-by-zero");
            return false;
        } else {
            self.operands.push(element.to_string());
            return true;
        }
    }
    fn make_operation(&mut self) -> i16 {
        match self.operands[0].parse::<i16>() {
            Ok(operand1) => {
                match self.operands[1].parse::<i16>() {
                    Ok(operand2) => {
                        if operand2 == 0 {
                            println!("Error: División por cero.");
                            return 0; 
                        }
                        operand1 / operand2  
                    },
                    Err(_) => {
                        
                        println!("Error al convertir el segundo operando a i16.");
                        0 
                    }
                }
            },
            Err(_) => {
                println!("Error al convertir el primer operando a i16.");
                0
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
