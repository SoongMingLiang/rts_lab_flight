use std::thread;

use rts_lab_flight::{actuator, rmq::consume};

pub fn main() {
    let mut tail_degree:i8 = 10;
    let mut wing_degree:i8 = 20;

    thread::spawn(move || {
        loop {
            let actuator_string = consume("actuator");
            let actuator = actuator::deserialize(actuator_string);
            
            tail_degree = actuator.tail_degree;
            wing_degree = actuator.wing_degree;

            println!("Tail degree thrust: {}", tail_degree);
            println!("Wing degree thrust: {}", wing_degree);
        }
    });

    loop {
        
    }
}