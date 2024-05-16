extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Wind {
    pub direction: String,
    pub speed: i8,
}

impl Wind {
    pub fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }    
}

pub fn deserialize(string: String) -> Wind {
    serde_json::from_str(&string).unwrap()
}
