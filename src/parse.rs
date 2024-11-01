use crate::json::JSON;
use crate::map::MyMap;

pub(crate) fn calculate_map(mut i: usize, input: &str) -> (usize, JSON) {//TODO
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

pub(crate) fn calculate_vector(mut i:usize, input: &str) -> (usize, JSON){

    let mut vector: Vec<JSON> = Vec::new();
    let mut value: JSON = JSON::NULL;

    i += 1;

    let mut has_value = false;


    loop {
        match input.as_bytes()[i] {
            b' ' | b'\t' | b'\n' => {
                i += 1;
            }
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
            b' ' | b'\t' | b'\n' => {
                i += 1;
            }
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
