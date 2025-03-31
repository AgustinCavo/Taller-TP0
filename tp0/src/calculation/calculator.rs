use crate::calculation::handlers::*;
use crate::operations::operation_trait::*;
use crate::operations::structs_arithmetic_op::*;
use crate::operations::structs_boolean_op::*;
use crate::operations::structs_conditional_op::*;
use crate::operations::structs_printing_op::*;
use crate::operations::structs_stack_op::*;

pub fn calculate(data: &mut Vec<String>) {
    let basic_operations = load_basic_operations_map();
    let stack_operations = load_stack_operations_map();
    let conditional_operations = load_conditional_operations_map();
    let printing_operations = load_printing_operations_map();
    let mut ongoing_ops: Vec<Box<dyn Operation>> = Vec::new();
    let mut i: i32 = (data.len() - 1) as i32;
    while i >= 0 {
        if data[i as usize] == "cr" {
            data.remove(i as usize);
        } else if get_operation(data, &mut i, &mut ongoing_ops) {
            data.remove(i as usize);
        } else {
            if let Some(last_op) = ongoing_ops.last_mut() {
                if basic_operations.contains_key(last_op.name())
                    || conditional_operations.contains_key(last_op.name())
                {
                    if basic_operations_handler(data, &mut i, last_op) {
                        ongoing_ops.pop();
                    }
                } else if stack_operations.contains_key(last_op.name()) {
                    if stack_operations_handler(data, &mut i, last_op) {
                        ongoing_ops.pop();
                    }
                } else if printing_operations.contains_key(last_op.name()) {
                    if printing_operations_handler(data, &mut i, last_op) {
                        ongoing_ops.pop();
                    }
                } else if last_op.name() == "if" {
                    if conditional_operation_handler(data, &mut i, last_op) {
                        ongoing_ops.pop();
                    }
                }
            }
        }
        i -= 1;
    }
    if ongoing_ops.len() > 0 {
        println!("stack-underflow");
    }
}
fn get_operation(
    data: &mut Vec<String>,
    i: &mut i32,
    ongoing_ops: &mut Vec<Box<dyn Operation>>,
) -> bool {
    let basic_operations = load_basic_operations_map();
    let stack_operations = load_stack_operations_map();
    let conditional_operations = load_conditional_operations_map();
    let printing_operations = load_printing_operations_map();

    if let Some(op_fn) = basic_operations.get(&data[*i as usize]) {
        ongoing_ops.push(op_fn());
        return true;
    }

    if let Some(op_fn) = stack_operations.get(&data[*i as usize]) {
        ongoing_ops.push(op_fn());
        return true;
    }

    if let Some(op_fn) = conditional_operations.get(&data[*i as usize]) {
        ongoing_ops.push(op_fn());
        return true;
    }

    if let Some(op_fn) = printing_operations.get(&data[*i as usize]) {
        ongoing_ops.push(op_fn());
        return true;
    }
    if &data[*i as usize] == "then" {
        ongoing_ops.push(if_struct_creation(data, i));

        return true;
    }
    false
}
