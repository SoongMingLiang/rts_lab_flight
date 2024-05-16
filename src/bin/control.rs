use std::{sync::mpsc::channel, thread};

use rts_lab_flight::{actuator::Actuator, altitude, engine::Engine, pressure, rmq::{consume, send}, wind};

pub fn main() {
    let (send_wind, receive_wind) = channel();
    let (send_pressure, receive_pressure) = channel();
    let (send_altitude, receive_altitude) = channel();

    thread::spawn(move || {
        loop {
            let wind_string = consume("wind");
            let wind = wind::deserialize(wind_string);
            send_wind.send(wind).unwrap();
        }
    });

    thread::spawn(move || {
        loop {
            let pressure_string = consume("pressure");
            let pressure = pressure::deserialize(pressure_string);
            send_pressure.send(pressure).unwrap();
        }
    });

    thread::spawn(move || {
        loop {
            let altitude_string = consume("altitude");
            let altitude = altitude::deserialize(altitude_string);
            send_altitude.send(altitude).unwrap();
        }
    });
    
    loop {
        match receive_wind.try_recv() {
            Ok (data) => {
                let act = Actuator{
                    tail_degree: data.speed,
                    wing_degree: data.speed,
                };
                let _ = send(act.serialize(), "actuator");

                let engine = Engine{
                    thrust: 1,
                };
                let _ = send(engine.serialize(), "engine");
            }
            Err(_) => {},
        }
        match receive_pressure.try_recv() {
            Ok (data) => {
                let act = Actuator{
                    tail_degree: data.value,
                    wing_degree: data.value,
                };
                let _ = send(act.serialize(), "actuator");

                let engine = Engine{
                    thrust: 2,
                };
                let _ = send(engine.serialize(), "engine");
            }
            Err(_) => {},
        }
        match receive_altitude.try_recv() {
            Ok (data) => {
                let act = Actuator{
                    tail_degree: data.value,
                    wing_degree: data.value,
                };
                let _ = send(act.serialize(), "actuator");

                let engine = Engine{
                    thrust: 3,
                };
                let _ = send(engine.serialize(), "engine");
            }
            Err(_) => {},
        }
    }
}