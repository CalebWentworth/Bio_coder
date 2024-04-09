use std::{
    collections::HashSet, env, error::Error, ffi::OsString, fs::File, iter::Filter, process::{self, Output}, ptr::read, result, vec
};
use rand::prelude::*;


fn parse_codes() -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let file_path = get_path(1)?;
    let file = File::open(file_path)?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    
    let mut codes: Vec<Vec<String>> = Vec::new();

    for result in reader.records() {
        let record = result?;
        if codes.is_empty() {
            // Initialize `codes` with empty vectors for each column
            for _ in 0..record.len() {
                codes.push(Vec::new());
            }
        }
        for (i, field) in record.iter().enumerate() {
            if !field.is_empty() {
                codes[i].push(field.to_string());
            }
        }
    }
    
    Ok(codes)
}

fn get_path(n:usize) -> Result<OsString,Box<dyn Error>> {
    match env::args_os().nth(n){
        None => Err(From::from("Expected path argument")),
        Some(path) => Ok(path)
    }
}

fn parse_exclude() -> Result<Vec<String>,Box<dyn Error>>{

    let file_path = get_path(2)?;
    let file = File::open(file_path)?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let mut exclusion: Vec<String> = Vec::new();

    for result in reader.records(){
        let record = result?;
        if let Some(first_field) = record.get(0) {
            exclusion.push(first_field.to_string());
        }
    }

    Ok(exclusion)
}

fn generate_combinations(codes: &[Vec<String>], prefix: String) -> Vec<String> {
    if codes.is_empty() {
        return vec![prefix];
    }

    let mut combinations = Vec::new();
    for code in &codes[0] {
        let new_prefix = format!("{}{}", prefix, code);
        let new_combinations = generate_combinations(&codes[1..], new_prefix);
        combinations.extend(new_combinations);
    }

    combinations
}

fn hamming(a:&String,b:&String)->i32{
    let mut i = 0;
    for (a_byte, b_byte) in a.as_bytes().iter().zip(b.as_bytes().iter()) {
        if a_byte != b_byte{
            i+=1;
        }
    }
    i
}
/*
fn filter(mut com: Vec<String>, exc: Vec<String>, sen: i32){
    println!("hi!");
    let mut result:Vec<String> = Vec::new();

    //primary pass for similarity to exclusion list.
    for i in &com {
        println!("{:?}",i);
        if exc.iter().all(|j| hamming(i, j) >= sen) {
            result.push(i.clone());
        }
    }

    //filter for other values in com.
    let com = result;
    result = Vec::new();
    let mut num=0;

    for i in &com {
        num+=1;
        print!("{:?}",num);
        println!("{:?}",i);
        if com.iter().all(|j| hamming(i, j) >= sen) {
            result.push(i.clone());
        }
    }

    let mut file = File::create("Barcodes.txt").expect("create failed");
    let output = result.join("\n");
    std::io::Write::write_all(&mut file, output.as_bytes()).expect("write failed");
}
*/
fn filter(mut com: Vec<String>, exc: Vec<String>, sen: i32){
    println!("hi!");
    let mut result:Vec<String> = Vec::new();

    //primary pass for similarity to exclusion list.
    for i in &com {
        println!("{:?}",i);
        if exc.iter().all(|j| hamming(i, j) >= sen) {
            result.push(i.clone());
        }
    }

    //filter for other values in com.
    let mut result: Vec<String> = Vec::new();
    let mut num = 0;

    while !com.is_empty() {
        num += 1;
        let i = com.remove(0);
        print!("{:?}", num);
        println!("{:?}", &i);
        let mut j = 1;
        while j < com.len() {
            if hamming(&i, &com[j]) < sen {
                com.remove(j);
            } else {
                j += 1;
            }
        }
        result.push(i);
    }
    
    let mut file = File::create("Barcodes.txt").expect("create failed");
    let output = result.join("\n");
    std::io::Write::write_all(&mut file, output.as_bytes()).expect("write failed");
}
fn main() {

    let codes = parse_codes().expect("Failed to parse Codes");
    let exclusions = parse_exclude().expect("Failed to Exclusions ");
    let mut combinations = generate_combinations(&codes, String::new());
    //println!("{:?}",combinations.len());
    let mut rng = thread_rng();
    let sensitivity: i32;
    
    match env::args_os().nth(4){
        Some(arg) =>{
            sensitivity = arg.to_string_lossy().parse::<i32>().unwrap_or(1);
        }
        None => {
            panic!("Specify sensitivity of missmatches")
        }
    }
    match env::args_os().nth(3) {
        Some(arg) => {
            if arg == "r" {
                println!("here");
                combinations.shuffle(&mut rng);
                filter(combinations, exclusions, sensitivity);
            }
            else if arg == "d"{
                println!("here!");
                filter(combinations, exclusions, sensitivity);
            }
        },
        None => {
            panic!("Specifiy r/d for random or deterministic generation of barcodes!")
        },
    }

}
