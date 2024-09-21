//#![allow(warnings)]

use std::{
    collections::BTreeMap,
    fmt::Debug,
    ops::{Index, IndexMut},
};

/// key: value pair
/// key can be string
/// value can be string, i64, vector object itself
/// Option<Box<dyn string+i64+struct> >  




// trait MapTrait: Debug {}
// trait JSONTrait: Debug {}
// impl JSONTrait for JSON {}
// impl JSONTrait for MyMap {}

#[derive(Debug)]
enum JSON {
    Integer(i32),
    String(String),
    Vector(Vec<JSON>),
    Map(MyMap),
}

#[derive(Debug)]
struct MyMap {
    map: BTreeMap<String, JSON>,
    fallback: String,
}

impl MyMap {
    fn new() -> MyMap {
        return MyMap {
            map: BTreeMap::new(),
            fallback: String::from("fallback_string"),
        };
    }
    fn get_mut(&mut self, key: &String) -> &mut Self {
        return match self.map.get_mut(key) {
            Some(_) => self,
            None => {
                self.map
                    .insert(key.clone(), JSON::String(self.fallback.clone()));
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
    //type Output = String;
    fn index_mut(&mut self, key: String) -> &mut Self::Output {
        self.get_mut(&key).map.get_mut(&key).unwrap()
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

fn main() {
    let x: BTreeMap<String, String> = BTreeMap::new();
    let mut y = MyMap::new();
    y[String::from("randome key")] = JSON::String(String::from("randome value"));
    y[String::from("internal object")] = JSON::Map(MyMap::new());
    y[5.to_string()] = JSON::Integer(5);
    y[String::from("internal object")][String::from("internal key")] = JSON::Integer(7);



    println!("Hello, world!,{:#?}\n{:#?}", x, y);
    println!("{:#?}", y[String::from("randome key")]);
    println!("{:#?}", y[String::from("internal object")][String::from("internal key")]);
}
