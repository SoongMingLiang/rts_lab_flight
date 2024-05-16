extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]

pub struct Actuator {
    pub tail_degree: i8,
    pub wing_degree: i8
}

impl Actuator {
    pub fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }    
}

pub fn deserialize(string: String) -> Actuator {
    serde_json::from_str(&string).unwrap()
}
