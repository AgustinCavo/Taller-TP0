use std::fs::File;
use std::io::{self, BufRead};

pub fn parse_fmt(path: &str)->io::Result<()>{
    

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

    Ok(())
    
}
