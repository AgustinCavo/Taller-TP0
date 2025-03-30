use crate::calculation::handlers::*;
use crate::operations::operation_trait::*;
use crate::operations::structs_arithmetic_op::*;
use crate::operations::structs_boolean_op::*;
use crate::operations::structs_stack_op::*;


pub fn calculate(data: &mut Vec<String>) {
    let basic_operations = load_basic_operations_map();
    let stack_operations = load_stack_operations_map();
    let conditional_operations=load_conditional_operations_map();
    let mut ongoing_ops: Vec<Box<dyn Operation>> = Vec::new();
    let mut i = data.len() as i32 - 1;

    while i >= 0 {
        println!("{}", i);

        if let Some(op_fn) = basic_operations.get(&data[i as usize]) {
            ongoing_ops.push(op_fn());
            println!("Cargué operación: {}", &data[i as usize]);
            data.pop();
        } else if let Some(op_fn) = stack_operations.get(&data[i as usize]) {
            ongoing_ops.push(op_fn());
            println!("Cargué operación: {}", &data[i as usize]);
            data.pop();
        } else {
            if let Some(last_op) = ongoing_ops.last_mut() {
                if basic_operations.contains_key(last_op.name()) {
                    if basic_operations_handler(data, &mut i, last_op) {
                        ongoing_ops.pop();
                    }
                } else if stack_operations.contains_key(last_op.name()) {
                    if stack_operations_handler(data, &mut i, last_op) {
                        ongoing_ops.pop();
                    }
                }else if conditional_operations.contains_key(last_op.name()){
                    if conditional_operations_handler(data, &mut i, last_op){
                        ongoing_ops.pop();
                    }
                }
            }
        }
        i -= 1;
    }
    if ongoing_ops.len() > 0 {
        println!("Insuficientes operandos");
    } else {
        println!("finish calculations");
    }
}