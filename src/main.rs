
extern crate websocket;

use std::io::stdin;
use std::sync::mpsc::channel;
use std::thread;



use websocket::client::ClientBuilder;
use websocket::{Message, OwnedMessage};

mod logger;

const CONNECTION_URL: &str = "ws://localhost:8080/socket";

fn test123(mut ws_sender: websocket::sender::Writer<std::net::TcpStream>, async_receiver: std::sync::mpsc::Receiver<websocket::OwnedMessage>) {
        loop {
            let message = match async_receiver.recv() {
                Ok(m) => m,
                Err(e) => {
                    log_error!("Send Loop: {:?}", e);
                    return;
                }
            };

            if let OwnedMessage::Close(close_data_option) = message {
                log_info!("Server sent 'close'.");
                if let Some(close_data) = close_data_option {
                    log_info!(
                        "Close was with status {:?} and reason {:?}.",
                        close_data.status_code,
                        close_data.reason,
                    );
                }
                let _ = ws_sender.send_message(&Message::close());
                return;
            }

            // Send the message
            match ws_sender.send_message(&message) {
                Ok(()) => (),
                Err(e) => {
                    log_error!("Send Loop: {:?}", e);
                    let _ = ws_sender.send_message(&Message::close());
                    return;
                }
            }
        }
    }

fn main() {
    let client = ClientBuilder::new(CONNECTION_URL)
        .expect("Unable to create ClientBuilder")
        .add_protocol("rust-websocket")
        .connect_insecure()
        .expect("Unable to connect to websocket");

    log_info!("Successfully connected to {:?}", CONNECTION_URL);

    let (mut ws_receiver, ws_sender) = client.split().unwrap();

    let (async_sender_for_stdin, async_receiver) = channel();
    let async_sender_for_ws_receiver = async_sender_for_stdin.clone();

    // let send_loop = thread::spawn(move || {
    //     loop {
    //         let message = match async_receiver.recv() {
    //             Ok(m) => m,
    //             Err(e) => {
    //                 log_error!("Send Loop: {:?}", e);
    //                 return;
    //             }
    //         };

    //         if let OwnedMessage::Close(close_data_option) = message {
    //             log_info!("Server sent 'close'.");
    //             if let Some(close_data) = close_data_option {
    //                 log_info!(
    //                     "Close was with status {:?} and reason {:?}.",
    //                     close_data.status_code,
    //                     close_data.reason,
    //                 );
    //             }
    //             let _ = ws_sender.send_message(&Message::close());
    //             return;
    //         }

    //         // Send the message
    //         match ws_sender.send_message(&message) {
    //             Ok(()) => (),
    //             Err(e) => {
    //                 log_error!("Send Loop: {:?}", e);
    //                 let _ = ws_sender.send_message(&Message::close());
    //                 return;
    //             }
    //         }
    //     }
    // });

    let send_loop = thread::spawn(move || test123(ws_sender, async_receiver));

    let receive_loop = thread::spawn(move || {
        // Receive loop
        for message_result in ws_receiver.incoming_messages() {
            let message = match message_result {
                Ok(m) => m,
                Err(e) => {
                    println!("Receive Loop: {:?}", e);
                    let _ = async_sender_for_ws_receiver.send(OwnedMessage::Close(None));
                    return;
                }
            };

            match message {
                OwnedMessage::Close(_) => {
                    // Got a close message, so send a close message and return
                    let _ = async_sender_for_ws_receiver.send(OwnedMessage::Close(None));
                    return;
                }
                OwnedMessage::Ping(data) => {
                    match async_sender_for_ws_receiver.send(OwnedMessage::Pong(data)) {
                        // Send a pong in response
                        Ok(()) => (),
                        Err(e) => {
                            println!("Receive Loop: {:?}", e);
                            return;
                        }
                    }
                }
                // Say what we received
                _ => println!("Receive Loop: {:?}", message),
            }
        }
    });

    loop {
        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();

        let message = match trimmed {
            "/close" => {
                // Close the connection
                let _ = async_sender_for_stdin.send(OwnedMessage::Close(None));
                break;
            }
            // Send a ping
            "/ping" => OwnedMessage::Ping(b"PING".to_vec()),
            // Otherwise, just send text
            _ => OwnedMessage::Text(trimmed.to_string()),
        };

        match async_sender_for_stdin.send(message) {
            Ok(()) => (),
            Err(e) => {
                println!("Main Loop: {:?}", e);
                break;
            }
        }
    }

    // We're exiting

    println!("Waiting for child threads to exit");

    let _ = send_loop.join();
    let _ = receive_loop.join();

    println!("Exited");
}
