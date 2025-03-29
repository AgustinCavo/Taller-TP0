use crate::operations::structs_arithmetic_op::*;
use std::collections::HashMap;

pub fn calculate(data: &mut Vec<String>) {
    let basic_operations = load_basic_operations_map();
    let mut ongoing_ops: Vec<Box<dyn ArithmeticOp>> = Vec::new();
    let mut i = data.len() as i32 - 1;

    while i >= 0 {
        println!("{}", i);
        if let Some(op_fn) = basic_operations.get(&data[i as usize]) {
            ongoing_ops.push(op_fn());
            println!("Cargué operación: {}", &data[i as usize]);
            data.pop();
        } else {
            if let Some(last_op) = ongoing_ops.last_mut() {
                if let Ok(operand) = data[i as usize].parse::<i16>() {
                    last_op.add_operand(operand);
                    println!("Agregué operando: {}", operand);
                    data.pop();

                    if last_op.operands() == last_op.quantity() as usize {
                        let result = last_op.make_operation();
                        data.push(result.to_string());
                        println!("Resultado de la operación: {}", result);
                    }
                }
            }
        }
        i -= 1;
    }

    println!("finish calculations");
}

fn load_basic_operations_map() -> HashMap<String, Box<dyn Fn() -> Box<dyn ArithmeticOp>>> {
    let mut basic_operations: HashMap<String, Box<dyn Fn() -> Box<dyn ArithmeticOp>>> =
        HashMap::new();

    basic_operations.insert(
        "+".to_string(),
        Box::new(|| {
            Box::new(Sum {
                quantity: 2,
                operands: Vec::new(),
            })
        }),
    );
    basic_operations.insert(
        "-".to_string(),
        Box::new(|| {
            Box::new(Sub {
                quantity: 2,
                operands: Vec::new(),
            })
        }),
    );
    basic_operations.insert(
        "/".to_string(),
        Box::new(|| {
            Box::new(Div {
                quantity: 2,
                operands: Vec::new(),
            })
        }),
    );
    basic_operations.insert(
        "*".to_string(),
        Box::new(|| {
            Box::new(Mul {
                quantity: 2,
                operands: Vec::new(),
            })
        }),
    );

    basic_operations
}
