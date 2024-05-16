use std::thread;

use rts_lab_flight::{engine, rmq::consume};

pub fn main() {
    let mut e1:i8 = 10;
    let mut e2:i8 = 20;

    thread::spawn(move || {
        loop {
            let engine_string = consume("engine");
            let engine = engine::deserialize(engine_string);
            
            e1 = engine.thrust;
            e2 = engine.thrust;

            println!("Engine 1 thrust: {}", e1);
            println!("Engine 2 thrust: {}", e2);
        }
    });

    loop {
        
    }
}