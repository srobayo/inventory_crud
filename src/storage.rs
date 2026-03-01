use crate::models::{Product, Salable, Service};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn load_items(path: &str) -> Result<Vec<Box<dyn Salable>>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut list: Vec<Box<dyn Salable>> = vec![];

    for line in reader.lines() {
        let l = line?;
        let parts: Vec<&str> = l.split(',').collect();
        match parts[0] {
            "PRODUCT" => list.push(Box::new(Product {
                name: parts[1].trim().to_string(),
                quantity: parts[2].trim().parse()?,
                price: parts[3].trim().parse()?,
            })),
            "SERVICE" => list.push(Box::new(Service {
                description: parts[1].trim().to_string(),
                price: parts[2].trim().parse()?,
            })),
            _ => {}
        }
    }

    Ok(list)
}

pub fn save_items(path: &str, list: &[Box<dyn Salable>]) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    for item in list {
        writeln!(file, "{}", item.a_csv())?;
    }
    Ok(())
}
