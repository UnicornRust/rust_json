use std::{
    collections::HashMap, fmt:: { Display, Formatter, Result }, ops::{Index, IndexMut}
};

#[derive(Debug, Clone, Default, PartialEq)]
pub enum JsonNode {
    Object(HashMap<String, JsonNode>),
    Array(Vec<JsonNode>),
    String(String),
    Number(f64),
    Boolean(bool),
    #[default]
    Null,
}

impl JsonNode {
    pub fn new() -> JsonNode {
        JsonNode::Null
    }
}

impl Display for JsonNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            JsonNode::Object(obj) => {
                write!(f, "{{")?;
                for (idx, (k, v)) in obj.iter().enumerate() {
                    write!(f, "\"{}\": ", k)?;
                    (*v).fmt(f)?;
                    if (idx) != obj.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "}}")?;
            }
            JsonNode::Array(arr) => {
                write!(f, "[")?;
                for (idx, v) in arr.iter().enumerate() {
                    (*v).fmt(f)?;
                    if idx != arr.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")?;
            }
            JsonNode::String(s) => {
                write!(f, "\"{}\": ", s)?;
            }
            JsonNode::Number(n) => {
                write!(f, "{}", n)?;
            }
            JsonNode::Boolean(b) => {
                write!(f, "{}", b)?;
            }
            JsonNode::Null => {
                write!(f, "null");
            }
        };
        Ok(())
    }
}


impl <'a> Index<&'a str> for JsonNode {
    type Output = JsonNode;

    fn index(&self, index: &'a str) -> &Self::Output {
        match self {
            JsonNode::Object(obj) =>  {
                if obj.contains_key(index) {
                    obj.get(index).unwrap()
                }else {
                    panic!("Key Not found")
                }
            }
            _ => panic!("Cannot index non-object type")
        } 
    }
}

impl <'a> IndexMut<& 'a str> for JsonNode {
    fn index_mut(&mut self, index: & 'a str) -> &mut Self {
        match self {
            JsonNode::Object(obj) => {
                if obj.contains_key(index) {
                    obj.get_mut(index).unwrap()
                }else {
                    obj.insert(index.to_string(), JsonNode::Null);
                    obj.get_mut(index).unwrap()
                }
            }
            _ => panic!("Cannot index non-object type"),
        }
    }
}

impl Index<usize> for JsonNode {
    type Output = JsonNode;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            JsonNode::Array(arr) => {
                if index < arr.len() {
                    arr.get(index).unwrap()
                }else {
                    panic!("Index out of bounds")
                }
            }
            _ => panic!("Cannot index no-array type")
        }
    }
}

impl IndexMut<usize> for JsonNode {
    fn index_mut(&mut self, index: usize) -> &mut Self {
        match self {
            JsonNode::Array(arr ) => {
                if index < arr.len() {
                    arr.get_mut(index).unwrap()
                }else {
                    panic!("Index out of bounds")
                }

            }
            _ => panic!("Cannot Index no-array type")
        }
    }
}


pub trait FromAndToJson{
    fn from_json(json: &JsonNode) -> Self;
    fn to_json(&self) -> JsonNode;
}

impl FromAndToJson for String {
    fn from_json(json: &JsonNode) -> Self {
        match json {
            JsonNode::String(s) => {
                s.clone()
            }
            _ => panic!("Cannot convert non-string type to string")
        }
    }

    fn to_json(&self) -> JsonNode {
        JsonNode::String(self.clone())
    }
}

impl FromAndToJson for Vec<JsonNode> {

    fn from_json(json: &JsonNode) -> Self {
        match json {
            JsonNode::Array(arr) => arr.clone(),
            _ => panic!("Cannot convert non-array to array")
        }

    }

    fn to_json(&self) -> JsonNode {
        JsonNode::Array(self.clone())
    }
}

impl FromAndToJson for HashMap<String, JsonNode> {

    fn from_json(json: &JsonNode) -> Self {
        match json {
            JsonNode::Object(obj) => {
                obj.clone()
            }
            _ => panic!("Cannot convert non-object to object")
        } 
    }

    fn to_json(&self) -> JsonNode {
        JsonNode::Object(self.clone())
    }
}

impl FromAndToJson for bool {
    fn from_json(json: &JsonNode) -> Self {
        match json {
            JsonNode::Boolean(b) => {
                b.clone()
            }
            _ =>  panic!("Cannot convert non-bool type to boolean")
        }
    }

    fn to_json(&self) -> JsonNode {
        JsonNode::Boolean(self.clone())
    }
}

impl FromAndToJson for f64 {
    fn from_json(json: &JsonNode) -> Self {
        match json {
            JsonNode::Number(n) => {
                n.clone()
            }
            _ => panic!("Cannot convert non-number to number")
        }
    }

    fn to_json(&self) -> JsonNode {
        JsonNode::Number(self.clone())
    }
}

pub struct JsonObjIterRef<'a> {
    map_iter: std::collections::hash_map::Iter<'a, String, JsonNode>,
}
pub struct JsonObjIterMut<'a> {
    map_iter: std::collections::hash_map::IterMut<'a, String, JsonNode>,
}

pub struct JsonObjIntoIter {
    map_iter: std::collections::hash_map::IntoIter<String, JsonNode>,
}

pub struct JsonArrIterRef<'a> {
    vec_iter: std::slice::Iter<'a, JsonNode>,
}


pub struct JsonArrIterMut<'a> {
    vec_iter: std::slice::IterMut<'a, JsonNode>,
}


pub struct JsonArrIntoIter {
    vec_iter: std::vec::IntoIter<JsonNode>,
}

impl<'a> Iterator for JsonObjIterRef<'a> {

    type Item = (& 'a String, &'a JsonNode);
    fn next(&mut self) -> Option<Self::Item> {
        self.map_iter.next()
    }
}

impl <'a> Iterator for JsonObjIterMut<'a> {

    type Item = (&'a String, &'a mut JsonNode);
    fn next(&mut self) -> Option<Self::Item> {
        self.map_iter.next()
    }
}

impl Iterator for JsonObjIntoIter {

    type Item = (String, JsonNode);
    fn next(&mut self) -> Option<Self::Item> {
        self.map_iter.next()
    }
}

impl <'a> Iterator for JsonArrIterRef<'a> {

    type Item = &'a JsonNode;
    fn next(&mut self) -> Option<Self::Item> {
        self.vec_iter.next()
    }
}

impl <'a> Iterator for JsonArrIterMut<'a> {

    type Item = &'a mut JsonNode;
    fn next(&mut self) -> Option<Self::Item> {
        self.vec_iter.next()
    }
}

impl Iterator for JsonArrIntoIter {

    type Item = JsonNode;
    fn next(&mut self) -> Option<Self::Item> {
        self.vec_iter.next()
    }
}

impl JsonNode {
    pub fn obj_iter(&self) -> Option<JsonObjIterRef> {
        match self {
            JsonNode::Object(obj)  => Some(JsonObjIterRef{
                map_iter:  obj.iter()
            }),
            _ => None,

        }
    }
    
    pub fn obj_iter_mut(&mut self) -> Option<JsonObjIterMut> {
        match self {
            JsonNode::Object(obj) => Some(JsonObjIterMut{
                map_iter: obj.iter_mut(),
            }),
            _ => None,
        }
    }
    pub fn obj_into_iter(self) -> Option<JsonObjIntoIter> {
        match self {
            JsonNode::Object(obj) => Some(JsonObjIntoIter{ 
                map_iter: obj.into_iter()
            }),
            _ => None,
        } 
    }

    pub fn arr_iter(&self) -> Option<JsonArrIterRef>{
        match self {
            JsonNode::Array(arr) => Some(JsonArrIterRef {
                vec_iter: arr.iter(),
            }),
            _ => None,

        }
    }
    pub fn arr_iter_mut(&mut self) -> Option<JsonArrIterMut> {
        match self {
            JsonNode::Array(arr) => Some(JsonArrIterMut{
                vec_iter: arr.iter_mut(),
            }),
            _ => None,
        }
    }
    pub fn arr_into_iter(self) -> Option<JsonArrIntoIter> {
        match self {
            JsonNode::Array(arr) => Some(JsonArrIntoIter{
                vec_iter: arr.into_iter(),
            }),
            _ => None,
        }
    }

    pub fn insert(&mut self, key: String, value: JsonNode) {
        match self {
            JsonNode::Object(obj) => {
                obj.insert(key, value);
            }
            _ => panic!("Cannot insert into non-object type"),
        }
    }
    pub fn push(&mut self, value: JsonNode) {
        match self {
            JsonNode::Array(arr) => {
                arr.push(value);
            }
            _ => panic!("Cannot push into non-array type"),
        }
    }
    pub fn reserve(&mut self, additional: usize) {
        match self {
            JsonNode::Array(arr) => {
                arr.reserve(additional);
            },
            _ => panic!("Cannot reserve into non-array type"),
        }
    }
    pub fn len(&self) -> usize {
        match self {
            JsonNode::Object(obj) => obj.len(),
            JsonNode::Array(arr) => arr.len(),
            _ => panic!("Cannot get length of non-array or non-object type"),
        }
    }

    pub fn resize(&mut self, new_len: usize) {
        match self {
            JsonNode::Array(arr) => {
                arr.resize(new_len, JsonNode::Null);
            }
            _ => panic!("Cannot resize non-array type"),
        }
    }

    pub fn clear(&mut self) {
        match self {
            JsonNode::Object(obj) => {
                obj.clear();
            }
            JsonNode::Array(arr) => {
                arr.clear();
            }
            _ => panic!("Cannot clear non-arary or non-object type")
        }
    }
    pub fn remove(&mut self, key: &str) -> Option<JsonNode> {
        match self {
            JsonNode::Object(obj) => obj.remove(key),
            _ => panic!("Cannot remove from non-object type"),
        }
    }
    pub fn contains_key(&self, key: &str) -> bool {
        match self {
            JsonNode::Object(obj) => obj.contains_key(key),
            _ => panic!("Cannot check key in non-object type"),
        } 
    }
    pub fn set_null(&mut self) {
        *self = JsonNode::Null;
    }
}

// 装饰宏，用于匹配一定的规则后生成一段代码
macro_rules! impl_from_and_to_json_for_number {
    ($($t:ty),*) => {
        $(
            impl FromAndToJson for $t {
                fn from_json(json: &JsonNode) -> Self {
                    match json {
                        JsonNode::Number(n) => n.clone() as $t
                        _ => panic!("Cannot convert non-number type to number")
                    }
                }
                fn to_json(&self) -> JsonNode {
                    JsonNoce::Number(self.clone() as f64)
                }
            }
        )*
    };
}
