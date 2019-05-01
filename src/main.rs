extern crate websocket;

use loops::loop_bot::run_loop_bot;
use loops::loop_stdin::run_loop_stdin;
use loops::loop_wsrecv::run_loop_wsrecv;
use loops::loop_wssend::run_loop_wssend;
use std::io::stdin;
use std::net::TcpStream;
use std::sync::mpsc::channel;
use std::thread;
use websocket::client::ClientBuilder;
use websocket::receiver::Reader;
use websocket::sender::Writer;

mod logger;
mod loops;
mod model;
mod routines;

const CRAWL_SERVER_LOCATION: &str = "ws://localhost:8080/socket";

fn init_websocket(crawl_server_location: &str) -> (Reader<TcpStream>, Writer<TcpStream>) {
    let client = ClientBuilder::new(crawl_server_location)
        .expect("Unable to create ClientBuilder, exiting.")
        .add_protocol("rust-websocket")
        .connect_insecure()
        .expect("Unable to connect to websocket, exiting.");

    let (ws_reader, ws_writer) = client
        .split()
        .expect("Unable to split websocket client into receiver and sender, exiting.");

    log_info!("Successfully connected to {:?}", CRAWL_SERVER_LOCATION);

    (ws_reader, ws_writer)
}

fn main() {
    let (ws_reader, ws_writer) = init_websocket(CRAWL_SERVER_LOCATION);

    let (send_wsrecv, recv_wsrecv) = channel();
    let (send_stdin, recv_stdin) = channel();
    let (send_bot, recv_bot) = channel();

    let loop_wssend = thread::spawn(move || run_loop_wssend(ws_writer, recv_bot));
    let loop_wsrecv = thread::spawn(move || run_loop_wsrecv(ws_reader, send_wsrecv));
    let loop_bot = thread::spawn(move || run_loop_bot(recv_stdin, recv_wsrecv, send_bot));

    run_loop_stdin(send_stdin);

    let _ = loop_wssend.join();
    let _ = loop_wsrecv.join();
    let _ = loop_bot.join();

    log_info!("Exited");
}
