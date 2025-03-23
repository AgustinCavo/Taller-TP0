use std::usize;

const SIMPLE_OPERATION: usize=2;

pub trait ArithmeticOp{
    fn make_operation(&self)->i16;
    fn add_operand(&mut self,element: i16)->bool;
}

pub struct Sum{
    pub quantity:usize,
    pub operands: Vec<i16>,
}
pub struct Sub{
    pub quantity:usize,
    pub operands: Vec<i16>,
}
pub struct Div{
    pub quantity:usize,
    pub operands: Vec<i16>,
}
pub struct Mul{
    pub quantity:usize,
    pub operands: Vec<i16>,
}

impl Sum{
    pub fn new()->Sum{
        Sum{
            quantity:SIMPLE_OPERATION,
            operands:Vec::new(),
        }
    }
}
impl ArithmeticOp for Sum{
   

    fn add_operand(&mut self,element:i16)->bool{
        if self.operands.len()<SIMPLE_OPERATION{    
            self.operands.push(element);
            return true;
        }
        return false;
    }

    fn make_operation(&self)->i16{
        return self.operands[0]+self.operands[1];
    }
}
impl Sub{
    pub fn new()->Sub{
        Sub{
            quantity:SIMPLE_OPERATION,
            operands:Vec::new(),
        }
    }
}
impl ArithmeticOp for Sub{
   
    fn add_operand(&mut self,element:i16)->bool{
        if self.operands.len()<SIMPLE_OPERATION{
            self.operands.push(element);
            return true;
        }
        return false;
    }
    fn make_operation(&self)->i16{
        return self.operands[0]-self.operands[1];
    }
}
impl Mul{
    pub fn new()->Mul{
        Mul{
            quantity:SIMPLE_OPERATION,
            operands:Vec::new(),
        }
    }
}
impl ArithmeticOp for Mul{

    fn add_operand(&mut self,element:i16)->bool{
        if self.operands.len()<SIMPLE_OPERATION{
            self.operands.push(element);
            return true;
        }
        return false;
        
    }
    fn make_operation(&self)->i16{
        return self.operands[0]*self.operands[1];
    }
}

impl Div{
    pub fn new()->Div{
        Div{
            quantity:SIMPLE_OPERATION,
            operands:Vec::new(),
        }
    }
}

impl ArithmeticOp for Div{
 
    fn add_operand(&mut self,element:i16)->bool{
        if element==0{
            return false;
        }else{
            self.operands.push(element);
            return true;
        }
        
    }
    fn make_operation(&self)->i16{
        return self.operands[0]/self.operands[1];
    }
}