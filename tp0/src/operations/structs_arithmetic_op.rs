use std::usize;

const SIMPLE_OPERATION: usize = 2;

pub trait ArithmeticOp {
    fn make_operation(&self) -> i16;
    fn add_operand(&mut self, element: i16) -> bool;
    fn quantity(&self) -> usize;
    fn operands(&self) -> usize;
}

pub struct Sum {
    pub quantity: usize,
    pub operands: Vec<i16>,
}
pub struct Sub {
    pub quantity: usize,
    pub operands: Vec<i16>,
}
pub struct Div {
    pub quantity: usize,
    pub operands: Vec<i16>,
}
pub struct Mul {
    pub quantity: usize,
    pub operands: Vec<i16>,
}

impl ArithmeticOp for Sum {
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
}

impl ArithmeticOp for Sub {
    fn add_operand(&mut self, element: i16) -> bool {
        if self.operands.len() < SIMPLE_OPERATION {
            self.operands.push(element);
            return true;
        }
        return false;
    }
    fn make_operation(&self) -> i16 {
        return self.operands[0] - self.operands[1];
    }

    fn quantity(&self) -> usize {
        return self.quantity;
    }
    fn operands(&self) -> usize {
        return self.operands.len();
    }
}

impl ArithmeticOp for Mul {
    fn add_operand(&mut self, element: i16) -> bool {
        if self.operands.len() < SIMPLE_OPERATION {
            self.operands.push(element);
            return true;
        }
        return false;
    }
    fn make_operation(&self) -> i16 {
        return self.operands[0] * self.operands[1];
    }
    fn quantity(&self) -> usize {
        return self.quantity;
    }
    fn operands(&self) -> usize {
        return self.operands.len();
    }
}

impl ArithmeticOp for Div {
    fn add_operand(&mut self, element: i16) -> bool {
        if element == 0 {
            return false;
        } else {
            self.operands.push(element);
            return true;
        }
    }
    fn make_operation(&self) -> i16 {
        return self.operands[0] / self.operands[1];
    }
    fn quantity(&self) -> usize {
        return self.quantity;
    }
    fn operands(&self) -> usize {
        return self.operands.len();
    }
}
