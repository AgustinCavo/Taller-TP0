use std::env::{self, Args};
use std::io::{self, Write, BufRead};
use std::fs::File;
mod operations;
mod parsing;
use parsing::parsing::*;

const CANTIDAD_ARGUMENTOS: usize=3;
const ERROR_DEFINICIONES: &str="Error al procesar las definiciones:";
const ERROR_LECTURA_ARCHIVO: &str="Error al procesar el archivo:";
const ERROR_ESCRITURA_ARCHIVO: &str="Error al escribir el archivo de salida:";

fn main() {
    let args:Vec<String>=env::args().collect();
    if args.len()!=CANTIDAD_ARGUMENTOS{
        eprint!("Pasar la ruta del archivo como argumento");
        std::process::exit(1);
    }
    let path=&args[1];
    
    match parse_fth(path){
        Ok(data)=>{
            match analize_definitions(data){
                Ok(data_cleaned) => {
                
                    for item in &data_cleaned {
                        println!("{}", item);
                    }
                    match write_stack_results(data_cleaned ){
                        Ok(_)=>{
                            println!("ok");
                        }
                        Err(e)=>{
                            println!("{} {}", ERROR_ESCRITURA_ARCHIVO, e);
                        }
                    }
                }
                Err(e) => {
                 
                    println!("{} {}", ERROR_DEFINICIONES, e);
               
                }
            }
        }Err (e) =>{
            println!("{} {}", ERROR_LECTURA_ARCHIVO, e);
        }
    }

}

fn write_stack_results(vec: Vec<String>) -> io::Result<()> {
  
    let mut resutls = match File::create("stack.fth"){
        Ok(file)=>file,
        Err(e)=>return Err(e),
    };

    for element in vec {
        writeln!(resutls, "{}", element); 
    }

    Ok(())
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
