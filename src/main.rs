#![allow(warnings)]

use std::{
    collections:: BTreeMap, fmt::Debug, fs::File, io::{BufReader, Read}, ops::{Index, IndexMut}, time
};


fn test_json() {


    let mut file: BufReader<File> = BufReader::new(File::open("./test.json").expect("open failed"));


    let mut file_content: String = String::new();

    file.read_to_string(&mut file_content).expect("not able to read file");
    file_content = String::from(file_content.trim());
    // match file_content.last()  {
    //     Some(value) => {
    //         if *value != ']'  && *value != '}' {
    //             panic!("json not in proper format or residual data is present at the end");
    //         }
    //     }
    //     None => {
    //         panic!("json not in proper format");
    //     }
    // }
    //file.read_vectored(bufs)
    let start_time = time::Instant::now();
    let (_i,json) = calculate_vector(0, &file_content);
    let parsing_time_duration = start_time.elapsed();
    let start_time = time::Instant::now();
    let _stringified_json = json.to_string();
    let stringify_time_duration = start_time.elapsed();

    println!("parsing time duration = {}", parsing_time_duration.as_micros());
    println!("stringifying time duration = {}", stringify_time_duration.as_micros());


}



fn main() {
    test_json();
   
     
}


#[derive(Debug)]
enum JSON {
    Integer(f64),
    String(String),
    Vector(Vec<JSON>),
    Map(MyMap),
    Boolean(bool),
    NULL,
}



impl JSON {
    fn to_string(&self) -> String{
        match self {
            JSON::Map(my_map) => {
                return Self::map_to_string(my_map);
            }
            JSON::Vector(vector) => {
                return Self::vector_to_string(vector);
            }
            _=> {
                panic!("not a json object");
            }
        }
    }
    fn map_to_string(my_map: &MyMap) -> String {
        let mut ret_value = String::new();
        ret_value.push('{');
        for (key, value) in my_map {
            ret_value.push_str(&Self::string_to_string(key));
            ret_value.push(':');
            match value {
                JSON::Integer(number) => {
                    ret_value.push_str(&Self::number_to_string(number));
                }
                JSON::String(str) => {
                    ret_value.push_str(&Self::string_to_string(str));
                }
                JSON::Map(value_map) => {
                    ret_value.push_str(&Self::map_to_string(value_map));
                }
                JSON::Vector(vector) => {
                    ret_value.push_str(&Self::vector_to_string(vector));
                }
                JSON::Boolean(boolean) => {
                    ret_value.push_str(&Self::boolean_to_string(boolean));
                }
                JSON::NULL => {
                    ret_value.push_str(&String::from("null"));
                }
            }
            ret_value.push(',');
        }
        ret_value.pop();
        ret_value.push('}');
        return ret_value;
    }

    fn string_to_string(str: &String) -> String {
        let mut ret_value = String::from('\"')+&(*str).clone();
        ret_value.push('\"');
        return ret_value;
    }

    fn number_to_string(number: &f64) -> String {
        return (*number).to_string();
    }

    fn vector_to_string(vector: &Vec<JSON>) -> String {
        let mut ret_value = String::new();
        ret_value.push('[');
        for value in vector {
            
            match value {
                JSON::Integer(number) => {
                    ret_value.push_str(&Self::number_to_string(number));
                }
                JSON::String(str) => {
                    ret_value.push_str(&Self::string_to_string(str));
                }
                JSON::Map(value_map) => {
                    ret_value.push_str(&Self::map_to_string(value_map));
                }
                JSON::Vector(vector) => {
                    ret_value.push_str(&Self::vector_to_string(vector));
                }
                JSON::Boolean(boolean) => {
                    ret_value.push_str(&Self::boolean_to_string(boolean));
                }
                JSON::NULL => {
                    ret_value.push_str(&String::from("null"));
                }
            }

            ret_value.push(',');
        }
        ret_value.pop();
        ret_value.push(']');
        return ret_value;
    }

    fn boolean_to_string(boolean: &bool) -> String {
        if *boolean {
            return String::from("true");
        } else {
            return String::from("false");
        }
    }

}

#[derive(Debug)]
struct MyMap {
    map: BTreeMap<String, JSON>,
}

impl MyMap {
    fn new() -> MyMap {
        return MyMap {
            map: BTreeMap::new(),
        };
    }
    fn get_mut(&mut self, key: &String) -> &mut Self {
        return match self.map.get_mut(key) {
            Some(_) => self,
            None => {
                self.map
                    .insert(key.clone(), JSON::NULL);
                return self;
            }
        };
    }
}

impl Index<String> for MyMap {
    type Output = JSON;
    fn index(&self, key: String) -> &Self::Output {
        match self.map.get(&key) {
            Some(value) => value,
            None => {
                return self.map.get(&key).unwrap();
            }
        }
    }
}

impl IndexMut<String> for MyMap {
    fn index_mut(&mut self, key: String) -> &mut Self::Output {
        self.get_mut(&key).map.get_mut(&key).unwrap()
    }
}
impl<'a> IntoIterator for &'a MyMap { //TODO implement iterator feature for map
    type Item = (&'a String, &'a JSON);
    type IntoIter = std::collections::btree_map::Iter<'a, String, JSON>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}
impl Index<String> for JSON {
    type Output = JSON;
    fn index(&self, key: String) -> &Self::Output {
        match self {
            JSON::Map(map) => {
                return &map[key];
            }
            _ => {
                panic!("key {} is not an object",key)
            }
        }
    }
}


impl IndexMut<String> for JSON {
    fn index_mut(&mut self, key: String) -> &mut Self::Output {
        match self {
            JSON::Map(map) => {
                return &mut map[key];
            }
            _ => {
                panic!("key {} is not an object",key)
            }
        }
    }
}

impl Index<usize> for JSON {
    type Output = JSON;
    fn index(&self, key: usize) -> &Self::Output {
        match self {
            JSON::Vector(vector) => {
                &vector[key]
            }
            _=>{
                panic!("not a vector");
            }
        }
    }
}
impl IndexMut<usize> for JSON {
    fn index_mut(&mut self, key: usize) -> &mut Self::Output {
        match self {
            JSON::Vector(vector) => {
                &mut vector[key]
            }
            _=>{
                panic!("not a vector");
            }
        }
    }
}




fn calculate_map(mut i: usize, input: &str) -> (usize, JSON) {//TODO
    let mut map: MyMap = MyMap::new();
    let mut key: String = String::new();
    let mut value: JSON = JSON::NULL;

    i += 1;

    let mut has_key = false;
    let mut has_value = false;


    loop {
        match input.as_bytes()[i] {
            b' '|b'\t'|b'\n' => {
                i += 1;
            }
            b'"' => {
                if has_key && has_value {
                    panic!("multiple values");
                } else if has_key {
                    panic!("multiple keys");
                } else {
                    (i,key) = calculate_key(i+1, input);
                    has_key = true;
                }
            }
            b':' => {
                if !has_key {
                    panic!("no key but specified for value");
                }
                (i,value) = calculate_value(i+1, input);
                has_value = true;
            }
            b',' => {
                if has_key && has_value {
                    map.map.insert(key, value);
                    key = String::new();
                    value = JSON::Integer(-1 as f64);
                } else {
                    panic!("no value or no keyHer");
                }

                i += 1;
                has_key = false;
                has_value = false;
            }
            b'}' => {
                if has_key && has_value {
                    map.map.insert(key, value);
                    return (i+1, JSON::Map(map));
                }
                else if (!has_key) && (!has_value) {
                    return (i+1, JSON::Map(map));
                }
                else {
                    panic!("ending curly braces has problems");
                }
            }
            _=> {
                panic!("unknown character, {}{}{}->{}",input.as_bytes()[i-1] as char,input.as_bytes()[i] as char,input.as_bytes()[i+1] as char,i);
            }
        }
    }
    panic!("error parsing map ending");
}

fn calculate_key(mut i: usize, input : &str) -> (usize, String) {

    let mut key: String = String::new();
    let ch = b'\\';
    let mut is_backslash_on = false;

    loop {
        match is_backslash_on {
            true => {
                match input.as_bytes()[i] {
                    b'\'' => {key.push('\'');},
                    b'\"' => { key.push('\"');},
                    b'n' => { key.push('\n');},
                    b'r' => { key.push('\r');},
                    b'\\' => { key.push('\\');},
                    c => { key.push('\\'); key.push(c as char);}
                }
                is_backslash_on = false;
                i += 1;
            }
            false => {
                if input.as_bytes()[i] == ch {
                    is_backslash_on = true;
                    i += 1;
                } else if input.as_bytes()[i] == b'"'  {
                    return (i+1, key);
                } else {
                    key.push(input.as_bytes()[i] as char);
                    i += 1;
                }
            }
        }
    }
    panic!("error parsing string string");
}

fn calculate_string(mut i:usize, input: &str) -> (usize, JSON){

    let mut value: String = String::new();
    let ch = b'\\';
    let mut is_backslash_on = false;

    loop {
        match is_backslash_on {
            true => {
                match input.as_bytes()[i] {
                    b'\'' => {value.push('\'');},
                    b'\"' => { value.push('\"');},
                    b'n' => { value.push('\n');},
                    b'r' => { value.push('\r');},
                    b'\\' => { value.push('\\');},
                    c => { value.push('\\'); value.push(c as char);}
                }
                is_backslash_on = false;
                i += 1;
            }
            false => {
                if input.as_bytes()[i] == ch {
                    is_backslash_on = true;
                    i += 1;
                } else if input.as_bytes()[i] == b'"'  {
                    return (i+1, JSON::String(value));
                } else {
                    value.push(input.as_bytes()[i] as char);
                    i += 1;
                }
            }
        }
    }
    panic!("error parsing string string");
}

fn calculate_boolean(mut i: usize, input: &str) -> (usize, JSON){
    let mut boolean: String = String::new();
    loop {
        if (input.as_bytes()[i] as char).is_alphabetic() {
            boolean.push(input.as_bytes()[i] as char);
            i += 1;
        }
        else if input.as_bytes()[i] == b',' || input.as_bytes()[i] == b'\t' || input.as_bytes()[i] == b'\n' || input.as_bytes()[i] == b' ' || input.as_bytes()[i] == b'}' || input.as_bytes()[i] == b']' {
            return (i,JSON::Boolean(boolean.parse().expect("not a boolean")));
        }
        else{
            panic!("unknown character between number");
        }
    }
    panic!("error parsing boolean");
}

fn calculate_number(mut i:usize, input: &str) -> (usize, JSON){
    let mut number: String = String::new();
    loop {
        if (input.as_bytes()[i]).is_ascii_digit() || input.as_bytes()[i] == b'.' {
            number.push(input.as_bytes()[i] as char);
            i += 1;
        }
        else if input.as_bytes()[i] == b',' || input.as_bytes()[i] == b'\t' || input.as_bytes()[i] == b'\n' || input.as_bytes()[i] == b' ' || input.as_bytes()[i] == b'}' || input.as_bytes()[i] == b']' {
            return (i,JSON::Integer(number.parse().expect("not a float")));
        }
        else{
            panic!("unknown character between number");
        }
    }
    panic!("error parsing string number");
}

//for just purpose of inclusion
fn calculate_null(mut i:usize, input:&str) -> (usize,JSON){ 
    let mut null = String::new();
    loop {
        if (input.as_bytes()[i] as char).is_alphabetic() {
            null.push(input.as_bytes()[i] as char);
            i += 1;
        }
        else if input.as_bytes()[i] == b',' || input.as_bytes()[i] == b'\t' || input.as_bytes()[i] == b'\n' || input.as_bytes()[i] == b' ' || input.as_bytes()[i] == b'}' || input.as_bytes()[i] == b']' {
            if null == String::from("null") {
                return (i,JSON::NULL);
            }
            panic!("not null value");
        }
        else{
            panic!("unknown character between null");
        }
    }
    panic!("error parsing null");

}

fn calculate_vector(mut i:usize, input: &str) -> (usize, JSON){

    let mut vector: Vec<JSON> = Vec::new();
    let mut value: JSON = JSON::Integer(-1 as f64);

    i += 1;

    let mut has_value = false;


    loop {
        match input.as_bytes()[i] {
            b'"' => {
                if has_value {
                    panic!("multiple values string");
                } else {
                    (i,value) = calculate_string(i+1, input);
                    has_value = true;
                }
            }
            b',' => {
                if  has_value {
                    vector.push(value);
                    value = JSON::Integer(-1 as f64);
                } else {
                    panic!("no value or no key");
                }
                has_value = false;
                i += 1;
            }
            b']' => {
                if has_value {
                    vector.push(value);
                }
                return (i+1, JSON::Vector(vector));
            }
            b't'|b'f' => {
                if has_value {
                    (i,value) = calculate_boolean(i, input);
                }
                //panic!("key should be wrapped in double quotes");
            }
            b'{' => {
                if has_value {
                    panic!("multiple values, object");
                }
                (i,value) = calculate_map(i, input);
                has_value = true;
            }
            b' ' | b'\t' | b'\n' => {
                i += 1;
            }
            c => {
                if c.is_ascii_digit() {
                    return calculate_number(i, input);
                }
                panic!("unknown character vector, {:#?}{}{}{}{}{}{}",value,input.as_bytes()[i-4],input.as_bytes()[i-3],input.as_bytes()[i-2],input.as_bytes()[i-1],c,i);
            }
        }
    }
    panic!("error parsing string array");
}



fn calculate_value(mut i:usize, input: &str) -> (usize, JSON) {
    loop {
        match input.as_bytes()[i] {
            b'{' => {
                return calculate_map(i, input);
            },
            b'"' => {
                return calculate_string(i+1, input);
            },
            b'[' => {
                return  calculate_vector(i, input);
            },
            b't'|b'f' => {
                return calculate_boolean(i, input);
            }
            b'n' => {
                return calculate_null(i, input);
            }
            b' ' | b'\t' | b'\n' => {
                i += 1;
            }
            c => {
                if c.is_ascii_digit() {
                    return calculate_number(i, input);
                }
                else {
                    panic!("unknown character recognized {}{}",c,i);//TODO boolean data
                }
            }
        }
        
    }
    panic!("error parsing value")
}




