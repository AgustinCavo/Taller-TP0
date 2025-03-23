use std::env::{self, Args};
mod operations;
mod parsing;
use operations::structs_arithmetic_op::*;
use parsing::parsing::*;

const CANTIDAD_ARGUMENTOS: usize=3;

fn main() {
    let args:Vec<String>=env::args().collect();
    if args.len()!=CANTIDAD_ARGUMENTOS{
        eprint!("Pasar ruta del archivo como argumento");
        std::process::exit(1);
    }
    let path=&args[1];
    println!("{}",path);
    //let mut pila = Vec::new();
    let element_emit="A";

    if let Err(e)=parse_fmt(path){
        eprintln!("Hubo un error al procesar el archivo: {}",e);
    }

    //inicio invocacion emit
   /* if element_emit.parse::<u8>().is_ok(){
        emit(element_emit);
    }else{
        println!("? ");
    }
    //fin emit de 1*/
    
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

