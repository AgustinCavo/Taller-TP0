//use std::string;
mod operations;

use operations::structs_arithmetic_op::*;

fn main() {
    
    let mut pila = Vec::new();
    let element_emit="A";

    pila.push("255");
    pila.push("20");
    pila.push("+");
    let mut suma= Sum::new();
    while let Some(element)=pila.pop(){
        
        if element=="+"{
            println!("soy un mas");
            
        }else{
            if let Ok(parsed_element)=element.parse::<i16>(){
                suma.add_operand(parsed_element);
            }
            
        }
    }
    print!("{}",suma.make_operation());



    //inicio invocacion emit
    if element_emit.parse::<u8>().is_ok(){
        emit(element_emit);
    }else{
        println!("? ");
    }
    //fin emit de 1
    
}

fn dot(element: &str) {
    println!("{}",element);   
}

fn emit(element: &str){
    match element.parse::<u8>(){
        Ok(value)=>{
            println!("{}",value as char);
        },
        Err(_)=>{
            dot(element);
        }
    }
    
}

