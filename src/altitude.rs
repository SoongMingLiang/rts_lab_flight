extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Altitude {
    pub value: i8,
}

impl Altitude {
    pub fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }    
}

pub fn deserialize(string: String) -> Altitude {
    serde_json::from_str(&string).unwrap()
}
