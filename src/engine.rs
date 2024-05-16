extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]

pub struct Engine {
    pub thrust: i8
}

impl Engine {
    pub fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }    
}

pub fn deserialize(string: String) -> Engine {
    serde_json::from_str(&string).unwrap()
}
