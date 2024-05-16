extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pressure {
    pub value: i8,
}

impl Pressure {
    pub fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }    
}

pub fn deserialize(string: String) -> Pressure {
    serde_json::from_str(&string).unwrap()
}
