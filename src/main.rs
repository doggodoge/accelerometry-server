use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::{accept, Error, Message};

fn main() {
    let server = TcpListener::bind("0.0.0.0:8765").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let processed_msg = match websocket.read_message() {
                    Ok(msg) => {
                        let msg = msg.to_string();
                        let split_msg: Vec<&str> = msg.split(',').collect();
                        format!(
                            "timestamp: {}\nx: {}\ty: {}\tz: {}\n",
                            split_msg[0], split_msg[1], split_msg[2], split_msg[3]
                        )
                    },
                    Err(_) => {
                        String::from("Connection failed")
                    }
                };

                println!("{}", processed_msg)
            }
        });
    }
}
