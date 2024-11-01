use std::collections::BTreeMap;
use std::ops::{Index, IndexMut};
use crate::json::JSON;

#[derive(Debug)]
pub(crate) struct MyMap {
    pub(crate) map: BTreeMap<String, JSON>,
}

impl MyMap {
    pub(crate) fn new() -> MyMap {
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
impl<'a> IntoIterator for &'a MyMap { 
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

