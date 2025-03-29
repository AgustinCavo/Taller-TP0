use crate::operations::structs_arithmetic_op::*;
use std::collections::HashMap;

pub fn calculate(data: &mut Vec<String>) {
    let basic_operations = load_basic_operations_map();
    let stack_operations=load_stack_operations_map();
    let mut ongoing_ops: Vec<Box<dyn Operation>> = Vec::new();
    let mut i = data.len() as i32 -1;

    while i >= 0 {
        println!("{}", i);
        
        if let Some(op_fn) = basic_operations.get(&data[i as usize]) {
            ongoing_ops.push(op_fn());
            println!("Cargué operación: {}", &data[i as usize]);
            data.pop();
        } else if let Some(op_fn)=stack_operations.get(&data[i as usize]){
            ongoing_ops.push(op_fn());
            println!("Cargué operación: {}", &data[i as usize]);
            data.pop();
        }else{
            if let Some(last_op) = ongoing_ops.last_mut() {
                if basic_operations.contains_key(last_op.name()) {
                    if basic_operations_handler(data, &mut i, last_op){
                        ongoing_ops.pop();
                    }
                    
                }else if stack_operations.contains_key(last_op.name()){
                    stack_operations_handler();
                    ongoing_ops.pop();
                }
            }
        }
        i -= 1;
    }

    println!("finish calculations");
}
fn basic_operations_handler(data: &mut Vec<String>,i: &mut i32,last_op: &mut Box<dyn Operation>)->bool{

    if let Ok(operand) = data[*i as usize].parse::<i16>() {
        last_op.add_operand(operand);
        println!("Agregué operando: {}", operand);
        data.pop();

        if last_op.operands() == last_op.quantity() as usize {
            let result = last_op.make_operation();
            data.push(result.to_string());
            println!("Resultado de la operación: {}", result);
            *i+=1;
           return true;
        }else{
            return false;
        }
    }
    return false;
}
fn stack_operations_handler(){

}
fn load_basic_operations_map() -> HashMap<String, Box<dyn Fn() -> Box<dyn Operation>>> {
    let mut basic_operations: HashMap<String, Box<dyn Fn() -> Box<dyn Operation>>> =
        HashMap::new();

    basic_operations.insert(
        "+".to_string(),
        Box::new(|| {
            Box::new(Sum {
                name:{"+"}.to_string(),
                quantity: 2,
                operands: Vec::new(),
            })
        }),
    );
    basic_operations.insert(
        "-".to_string(),
        Box::new(|| {
            Box::new(Sub {
                name:{"-"}.to_string(),
                quantity: 2,
                operands: Vec::new(),
            })
        }),
    );
    basic_operations.insert(
        "/".to_string(),
        Box::new(|| {
            Box::new(Div {
                name:{"/"}.to_string(),
                quantity: 2,
                operands: Vec::new(),
            })
        }),
    );
    basic_operations.insert(
        "*".to_string(),
        Box::new(|| {
            Box::new(Mul {
                name:{"*"}.to_string(),
                quantity: 2,
                operands: Vec::new(),
            })
        }),
    );

       basic_operations
}

fn load_stack_operations_map()->HashMap<String, Box<dyn Fn() -> Box<dyn Operation>>> {
    let mut stack_operations: HashMap<String, Box<dyn Fn() -> Box<dyn Operation>>> =HashMap::new();

    stack_operations.insert(
        "drop".to_string(),
        Box::new(|| {
            Box::new(Drop {
                name:{"drop"}.to_string(),
                quantity: 1,
                operands: Vec::new(),
            })
        }),
    );

    stack_operations.insert(
        "dup".to_string(),
        Box::new(|| {
            Box::new(Dup {
                name:{"dup"}.to_string(),
                quantity: 1,
                operands: Vec::new(),
            })
        }),
    );

    stack_operations.insert(
        "swap".to_string(),
        Box::new(|| {
            Box::new(Swap {
                name:{"swap"}.to_string(),
                quantity: 1,
                operands: Vec::new(),
            })
        }),
    );
    stack_operations.insert(
        "over".to_string(),
        Box::new(|| {
            Box::new(Over {
                name:{"over"}.to_string(),
                quantity: 1,
                operands: Vec::new(),
            })
        }),
    );
    stack_operations.insert(
        "rot".to_string(),
        Box::new(|| {
            Box::new(Rot {
                name:{"rot"}.to_string(),
                quantity: 1,
                operands: Vec::new(),
            })
        }),
    );
    stack_operations
}