extern crate rand;

use std::thread;
use std::sync::mpsc::channel;
use std::time::Duration;
use rand::prelude::*;
use rts_lab_flight::rmq::send;
use rts_lab_flight::altitude::Altitude;

pub fn main() {
    let (s, r) = channel();

    thread::spawn(move || { // move the ownership of s to this scope
        loop {
            let mut rng = rand::thread_rng();
            let value: i8 = rng.gen();
            let wind = Altitude{
                value: value
            };
            s.send(wind).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    loop {
        let send_value = r.recv().unwrap();
        let _ = send(send_value.serialize(), "altitude");
    }
}
