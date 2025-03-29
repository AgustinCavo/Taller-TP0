use std::fs::File;
use std::collections::HashMap;
use std::io::{self, BufRead, Error, ErrorKind};


use crate::operations::structs_arithmetic_op::*;

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

    Ok(data)
    
}
pub fn analize_definitions(data: Vec<String>) -> io::Result<Vec<String>> {
    
    match get_definitions(data) {
        Ok((mut data_cleaned, mut new_words_map)) => {
            match apply_definitions(&mut data_cleaned, &mut new_words_map){
                Ok(())=> Ok(data_cleaned),
                Err(e)=>{
                    Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
                }
            }
        }
        Err(e) => {
            Err(e)
        }
    }
}   
fn get_definitions(data: Vec<String>) -> io::Result<(Vec<String>, HashMap<String, Vec<String>>)> {
    let mut remeaning_operations_data: Vec<String> = Vec::new();
    let mut new_words_map = HashMap::new();
    let mut key_being_checked: Option<String> = None;
    let mut values_for_actual_key: Vec<String> = Vec::new();
    let mut parsing_values = false;
    let mut iter = data.into_iter();
    let basic_operations = load_basic_operations_map();

    while let Some(item) = iter.next() {
        if item == ":" {
            parse_colon(&mut iter, &mut key_being_checked, &mut values_for_actual_key, &mut parsing_values)?;
        } else if item == ";" {
            parse_semicolon(&mut key_being_checked, &mut values_for_actual_key, &mut parsing_values, &mut new_words_map,&basic_operations)?;
        } else {
            if parsing_values {
               values_for_actual_key.push(item);
            } else {
                remeaning_operations_data.push(item);
            }
        }
    }

    finalize_key(&key_being_checked, &values_for_actual_key)?;

    Ok((remeaning_operations_data, new_words_map))
}

fn parse_colon(iter: &mut impl Iterator<Item = String>, key_being_checked: &mut Option<String>, values_for_actual_key: &mut Vec<String>, parsing_values: &mut bool) -> io::Result<()> {
    if let Some(key) = iter.next() {
        let valid_word = &key;

        if key_being_checked.is_some() {
            return Err(Error::new(ErrorKind::InvalidData, ERROR_STACKING_DEFINITIONS));
        } else if is_valid_word_definition(valid_word) {
            *key_being_checked = Some(key);
            values_for_actual_key.clear();
            *parsing_values = true;
        } else {
            return Err(Error::new(ErrorKind::InvalidData, ERROR_NAME_DEFINITION));
        }
    } else {
        return Err(Error::new(ErrorKind::InvalidData, ERROR_NAME_DEFINITION));
    }
    Ok(())
}

fn parse_semicolon(key_being_checked: &mut Option<String>, values_for_actual_key: &mut Vec<String>, parsing_values: &mut bool, new_words_map: &mut HashMap<String, Vec<String>>,basic_operations: &HashMap<String, Box<dyn Fn() -> Box<dyn ArithmeticOp>>>,) -> io::Result<()> {
    if let Some(key) = key_being_checked.take() {

     for i in 0..values_for_actual_key.len(){
        if new_words_map.contains_key(&values_for_actual_key[i]){
            if let Some(replacement) = new_words_map.get(&values_for_actual_key[i]) {
                values_for_actual_key.splice(i..i + 1, replacement.iter().cloned());
            }
        }
        
     }
     new_words_map.insert(key, values_for_actual_key.to_vec());
     values_for_actual_key.clear();
     *parsing_values = false;
        
    } else {
        return Err(Error::new(ErrorKind::InvalidData, ERROR_MISSING_STARITING_INDICATOR));
    }
    Ok(())
}

fn finalize_key(key_being_checked: &Option<String>, values_for_actual_key: &Vec<String>) -> io::Result<()> {
    if key_being_checked.is_some() && !values_for_actual_key.is_empty() {
        return Err(Error::new(ErrorKind::InvalidData, ERROR_MISSING_FINISHING_INDICATOR));
    }
    Ok(())
}
fn is_valid_word_definition(key: &str)->bool{
        match key.parse::<i16>(){
            Ok(_)=>{
                return false;
            },
            Err(_)=>{
                return true;
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
fn apply_definitions(data: &mut Vec<String>, new_operations: &mut HashMap<String, Vec<String>>)-> io::Result<()>{
    
    let mut i = 0;
    while i < data.len() {
        if new_operations.contains_key(&data[i]) {
            if let Some(replacement) = new_operations.get(&data[i]) {
                
                data.splice(i..i + 1, replacement.iter().cloned());
                
                i += replacement.len();
                continue;
            }
        }
        i += 1;
    }
    println!("finish");
    
    Ok(()) 
}