use std::fs::File;
use std::collections::{hash_map, HashMap};
use std::io::{self, BufRead, Error, ErrorKind};

use crate::operations::structs_arithmetic_op::*;
const ERROR_DEFINITION: &str="Error al procesar las definiciones:";
const ERROR_STACKING_DEFINITIONS: &str="No se termino una definicion antes de empezar otra";
const ERROR_MISSING_STARITING_INDICATOR: &str="Se encontro ; sin : en la definicion";
const ERROR_MISSING_FINISHING_INDICATOR: &str="No se encontro ; en la definicion";
const ERROR_NAME_DEFINITION: &str="Error en la palabra utilizada para la definicion";

pub fn parse_fth(path: &str)->io::Result<Vec<String>>{
    

    let file = File::open(path)?;   
    let reader = io::BufReader::new(file);
    let mut data = Vec::new();
    
    for line in reader.lines() {
        let line = line?; 
        
        for word in line.split_whitespace() {
            data.push(word.to_string());
        }
    }
    
    println!("{:?}", data);

    Ok(data)
    
}
pub fn analize_definitions(data: Vec<String>)->io::Result<Vec<String>>{

    match get_definitions(data){
        
        Ok(tuple)=>{
            Ok(tuple.0)
            
        }
        Err(e)=>{
            println!("{} {}", ERROR_DEFINITION, e);
            Err(e)
        }
    }
    

}

fn get_definitions(data: Vec<String>) -> io::Result<(Vec<String>, HashMap<String, Vec<String>>)>{
    let mut remeaning_operations_data: Vec<String> = Vec::new();
       
    let mut result = HashMap::new();
    
    let mut key_being_checked: Option<String> = None;
    let mut values_for_actual_key: Vec<String> = Vec::new();
    
    
    let mut parsing_values = false;
    
    let mut iter = data.into_iter();

    
    let basic_operations=load_basic_operations_map();

    while let Some(item) = iter.next() {
        if item == ":" {
            if let Some(key) = iter.next() {
                
                let valid_word=&key;
                
                if key_being_checked.is_some() {
                    return Err(Error::new(ErrorKind::InvalidData, ERROR_STACKING_DEFINITIONS));
                }else if is_valid_word_definition(valid_word,&basic_operations) {
                    key_being_checked = Some(key);
                    values_for_actual_key.clear();
                    parsing_values = true;
                }else{
                    return Err(Error::new(ErrorKind::InvalidData, ERROR_STACKING_DEFINITIONS));
                }
                
            } else{
                return Err(Error::new(ErrorKind::InvalidData, ERROR_NAME_DEFINITION));
            }
        } else if item == ";" {
    
            if let Some(key) = key_being_checked.take() {
                result.insert(key, values_for_actual_key.clone());
                parsing_values = false;
            } else {
                return Err(Error::new(ErrorKind::InvalidData, ERROR_MISSING_STARITING_INDICATOR));
            }
        } else {
    
            if parsing_values {
                values_for_actual_key.push(item);
            } else {
                remeaning_operations_data.push(item);
            }
        }
    }
    
    if key_being_checked.is_some() && !values_for_actual_key.is_empty() {
        return Err(Error::new(ErrorKind::InvalidData, ERROR_MISSING_FINISHING_INDICATOR));
    }

    Ok((remeaning_operations_data, result))

}
fn is_valid_word_definition(key: &str, basic_operations_map: &HashMap<String, Box<dyn Fn() -> Box<dyn ArithmeticOp>>>)->bool{
    if basic_operations_map.contains_key(key){
        return false;
    }else{
        match key.parse::<u8>(){
            Ok(_)=>{
                return false;
            },
            Err(_)=>{
                return true;
            }
        }
    }
}

fn load_basic_operations_map() -> HashMap<String, Box<dyn Fn() -> Box<dyn ArithmeticOp>>> {
    let mut basic_operations: HashMap<String, Box<dyn Fn() -> Box<dyn ArithmeticOp>>> = HashMap::new();
    
    
    basic_operations.insert("+".to_string(), Box::new(|| Box::new(Sum { quantity: 2, operands: Vec::new() })));
    basic_operations.insert("-".to_string(), Box::new(|| Box::new(Sub { quantity: 2, operands: Vec::new() })));
    basic_operations.insert("/".to_string(), Box::new(|| Box::new(Div { quantity: 2, operands: Vec::new() })));
    basic_operations.insert("*".to_string(), Box::new(|| Box::new(Mul { quantity: 2, operands: Vec::new() })));
    
    basic_operations
}
fn applied_definitions(data: (Vec<String>, HashMap<String, Vec<String>>))-> io::Result<Vec<String>>{
    let (vec_data, map_data) = data;

    
    println!("Datos procesados: {:?}", vec_data);
    println!("Definiciones en el mapa: {:?}", map_data);

    
    Ok(vec_data) 
}

