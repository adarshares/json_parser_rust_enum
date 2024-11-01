#![allow(warnings)]

use std::{
    collections:: BTreeMap, fmt::Debug, fs::File, io::{BufReader, Read}, ops::{Index, IndexMut}, time
};

mod json;
mod parse;
mod map;


fn test_json() {

    let mut file: BufReader<File> = BufReader::new(File::open("./large.json").expect("open failed"));
    let mut file_content: String = String::new();

    file.read_to_string(&mut file_content).expect("not able to read file");
    file_content = String::from(file_content.trim());


    let start_time = time::Instant::now();
    //start parsing
    let (_i, json) = if file_content.starts_with('{') {
        parse::calculate_map(0, &file_content)
    } else if file_content.starts_with('['){
        parse::calculate_vector(0, &file_content)
    } else {
        panic!("not in proper json format");
    };
    
    let parsing_time_duration = start_time.elapsed();
    

    println!("parsing time duration = {}", parsing_time_duration.as_micros());


}



fn main() {
    test_json();
   
     
}




