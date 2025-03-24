use std::env::{self, Args};
use std::io::{self, BufRead};

use std::process;
use std::collections::HashMap;
mod operations;
mod parsing;
use parsing::parsing::*;

const CANTIDAD_ARGUMENTOS: usize=3;
const ERROR_DEFINICIONES: &str="Error al procesar las definiciones:";
const ERROR_LECTURA_ARCHIVO: &str="Error al procesar el archivo:";

fn main() {
    let args:Vec<String>=env::args().collect();
    if args.len()!=CANTIDAD_ARGUMENTOS{
        eprint!("Pasar la ruta del archivo como argumento");
        std::process::exit(1);
    }
    let path=&args[1];
    println!("{}",path);
    
    match parse_fth(path){
        Ok(data)=>{
            match analize_definitions(data){
                Ok(data_cleaned) => {
                    // Si es Ok, muestra los elementos del Vec<String>
                    /*for item in &data_cleaned {
                        println!("{}", item);
                    }*/
                    println!("ok");
                }
                Err(e) => {
                    // Si es Err, muestra el HashMap
                    println!("{} {}", ERROR_DEFINICIONES, e);
               
                }
            }
        }Err (e) =>{
            println!("{} {}", ERROR_LECTURA_ARCHIVO, e);
        }
    }
    
}

/*    //inicio invocacion emit
    if element_emit.parse::<u8>().is_ok(){
        emit(element_emit);
    }else{
        println!("? ");
    }
    //fin emit de 1
    


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
*/
