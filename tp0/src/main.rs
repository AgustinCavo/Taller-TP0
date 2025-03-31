use std::env::{self};
use std::fs::File;
use std::io::{self, Write};
mod calculation;
mod operations;
mod parsing;
use calculation::calculator::*;
use parsing::parsing::*;


const ERROR_DEFINICIONES: &str = "Error al procesar las definiciones:";
const ERROR_LECTURA_ARCHIVO: &str = "Error al procesar el archivo:";
const ERROR_ESCRITURA_ARCHIVO: &str = "Error al escribir el archivo de salida:";

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    match parse_fth(path) {
        Ok(data) => match analize_definitions(data) {
            Ok(mut data_cleaned) => {
                calculate(&mut data_cleaned);
                match write_stack_results(data_cleaned) {
                    Ok(_) => {
                        
                    }
                    Err(e) => {
                        println!("{} {}", ERROR_ESCRITURA_ARCHIVO, e);
                    }
                }
            }
            Err(e) => {
                println!("{} {}", ERROR_DEFINICIONES, e);
            }
        },
        Err(e) => {
            println!("{} {}", ERROR_LECTURA_ARCHIVO, e);
        }
    }
}

fn write_stack_results(vec: Vec<String>) -> io::Result<()> {
    let mut resutls = match File::create("stack.fth") {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

   for element in vec {
        write!(resutls, "{} ", element)?;
    }

    Ok(())
}
