use crate::map::MyMap;

#[derive(Debug)]
pub enum JSON {
    Integer(f64),
    String(String),
    Vector(Vec<JSON>),
    Map(MyMap),
    Boolean(bool),
    NULL,
}



impl JSON {
    pub fn to_string(&self) -> String{
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
