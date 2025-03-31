use crate::operations::operation_trait::Operation;
const SIMPLE_OPERATION: usize = 2;

struct Warhouseif {
    pub name: String,
    quantity: usize,
    operands: Vec<String>,
    yes: Vec<String>,
    no: Vec<String>,
    status: i16,
}

impl Operation for Warhouseif{
    fn add_operand(&mut self, element: i16) -> bool {
        if self.operands.len() < SIMPLE_OPERATION {
            self.operands.push(element.to_string());
            return true;
        }
        return false;
    }

    fn make_operation(&mut self) -> i16 {
        if self.operands[0] == "0"{
            self.status = 0;
        } else {
            self.status = 1;
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
    fn get_operands(&self) -> &[String] {
        if self.status==0{
            &self.no
        }else{
            &self.yes
        }
    }
}

pub fn if_struct_creation(data: &mut Vec<String>,i: &mut i32,)->Box<dyn Operation>{
    let mut if_operation = Warhouseif {
        name: "if".to_string(),
        quantity: 1,  
        operands: vec![],  
        yes: vec![],  
        no: vec![],  
        status: 0,   
    };
    data.pop();
    *i-=1;
    while (*i as usize) < data.len() && data[*i as usize] != "else" && data[*i as usize] != "if" {
        if_operation.yes.push(data[*i as usize].to_string()); 
        data.remove(*i as usize); 
        *i-=1;
    }
    if (*i as usize)< data.len() && data[*i as usize] == "else" {
        data.remove(*i as usize);
        *i-=1; 
        while (*i as usize) < data.len() && data[*i as usize] != "if" {
         
            if_operation.no.push(data[*i as usize].to_string()); 
            data.remove(*i as usize);
            *i-=1; 
        }
    }
    println!("data : {:?}", data);
    println!("Instrucciones YES: {:?}", if_operation.yes);
    println!("Instrucciones NO: {:?}", if_operation.no);
    Box::new(if_operation) 
}