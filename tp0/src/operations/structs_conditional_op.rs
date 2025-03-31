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

impl Operation for Warhouseif {
    fn add_operand(&mut self, element: i16) -> bool {
        if self.operands.len() < SIMPLE_OPERATION {
            self.operands.push(element.to_string());
            return true;
        }
        return false;
    }

    fn make_operation(&mut self) -> i16 {
        if self.operands[0] == "0" {
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
    fn get_operands(&self) -> &Vec<String> {
        if self.status == 0 {
            &self.no
        } else {
            &self.yes
        }
    }
}

pub fn if_struct_creation(data: &mut Vec<String>, i: &mut i32) -> Box<dyn Operation> {
    let mut nested_if_count: i32 = 1;
    let mut residual_count: i32 = 0;
    let mut aux: Vec<String> = Vec::new();
    let mut if_operation = Warhouseif {
        name: "if".to_string(),
        quantity: 1,
        operands: vec![],
        yes: vec![],
        no: vec![],
        status: 0,
    };
    data.pop();
    *i -= 1;
    while nested_if_count != 0 {
        let current_item = &data[*i as usize];

        aux.push(current_item.to_string());

        if current_item == "then" {
            nested_if_count += 1;
        }
        if current_item == "if" {
            if nested_if_count == 1 {
                aux.pop();
                if_operation.yes = aux.clone();
            }
            nested_if_count -= 1;
        }
        if current_item == "else" {
            if nested_if_count == 1 {
                aux.pop();
                if_operation.no = aux.clone();
            }
        }
        residual_count += 1;
        *i -= 1;
    }
    for _ in 0..residual_count {
        data.pop();
    }
    Box::new(if_operation)
}
