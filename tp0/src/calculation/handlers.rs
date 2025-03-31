use crate::operations::operation_trait::Operation;

pub fn basic_operations_handler(
    data: &mut Vec<String>,
    i: &mut i32,
    last_op: &mut Box<dyn Operation>,
) -> bool {
    if let Ok(operand) = data[*i as usize].parse::<i16>() {
        last_op.add_operand(operand);     
        data.remove(*i as usize);

        if last_op.operands() == last_op.quantity() as usize {
            let result = last_op.make_operation();
            data.insert(*i as usize, result.to_string());
            *i += 1;
            return true;
        } else {
            return false;
        }
    }
    return false;
}
pub fn stack_operations_handler(
    data: &mut Vec<String>,
    i: &mut i32,
    last_op: &mut Box<dyn Operation>,
) -> bool {
    if let Ok(operand) = data[*i as usize].parse::<i16>() {
        last_op.add_operand(operand);
        if last_op.operands() == last_op.quantity() as usize {
            match last_op.name() {
                "drop" => {
                    data.pop();
                }
                "dup" => {
                    if let Some(last) = data.last() {
                        data.push(last.to_owned());
                    }
                    *i += 2;
                }
                "over" => {
                    let over_position = (*i) as usize;

                    if let Some(second) = data.get(over_position) {
                        data.push(second.to_owned());
                    }
                    *i += 1;
                }
                "swap" => {
                    data.swap(*i as usize, (*i + 1) as usize);
                    *i += 1;
                }

                "rot" => {
                    if let Some(a1) = data.get(0) {
                        data.push(a1.to_string());
                        data.remove(0);
                    }
                    *i += 3;
                }
                _ => {
                    return false;
                }
            }
            return true;
        } else {
            return false;
        }
    }
    println!("No hay suficientes elementos para realizar la operación.");
    return false;
}

pub fn printing_operations_handler(
    data: &mut Vec<String>,
    i: &mut i32,
    last_op: &mut Box<dyn Operation>,
) -> bool {
    if let Ok(operand) = data[*i as usize].parse::<i16>() {
        last_op.add_operand(operand);
        if last_op.operands() == last_op.quantity() as usize {
            match last_op.name() {
                "." => {
                    last_op.make_operation();
                    data.remove(*i as usize);
                }
                "emit" => {
                    match ascii_to_string(data[*i as usize].parse::<i16>().ok()) {
                        Some(result) => print!("{} ", result),
                        None => println!("Error: No se pudo convertir a String válido"),
                    }
                    data.remove(*i as usize);
                }
                "Quotes" => {
                    let over_position = (*i) as usize;

                    if let Some(second) = data.get(over_position) {
                        data.push(second.to_owned());
                    }
                    *i += 1;
                }
                _ => {
                    return false;
                }
            }
            return true;
        } else {
            return false;
        }
    }
    println!("No hay suficientes elementos para realizar la operación.");
    return false;
}

pub fn conditional_operation_handler(
    data: &mut Vec<String>,
    i: &mut i32,
    last_op: &mut Box<dyn Operation>,
) -> bool {
    if let Ok(operand) = data[*i as usize].parse::<i16>() {
        last_op.add_operand(operand);     

        if last_op.operands() == last_op.quantity() as usize {
            last_op.make_operation();
            let operands = last_op.get_operands().iter().cloned().collect::<Vec<String>>();

            for operand in operands {

                data.insert(0,operand);
                *i+=1;
            }
            
            return true;
        } else {
            return false;
        }
    }
    return false;
}

fn ascii_to_string(ascii_value: Option<i16>) -> Option<String> {
    if let Some(value) = ascii_value {
        if let Some(ch) = char::from_u32(value as u32) {
            return Some(ch.to_string());
        }
    }
    None
}
