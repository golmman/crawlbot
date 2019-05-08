extern crate websocket;

use loops::bot_loop::BotLoopState;
use loops::loop_stdin::run_loop_stdin;
use loops::reader_loop::ReaderLoopState;
use loops::writer_loop::run_writer_loop;
use loops::writer_loop::WriterLoopState;
use model::LoopState;
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

    let mut writer_loop_state = WriterLoopState::new(ws_writer, recv_bot);
    let mut reader_loop_state = ReaderLoopState::new(ws_reader, send_wsrecv);
    let mut bot_loop_state = BotLoopState::new(recv_stdin, recv_wsrecv, send_bot);

    let writer_loop = thread::spawn(move || writer_loop_state.start_loop());
    let reader_loop = thread::spawn(move || reader_loop_state.start_loop());
    let bot_loop = thread::spawn(move || bot_loop_state.start_loop());

    run_loop_stdin(send_stdin);

    let _ = writer_loop.join();
    let _ = reader_loop.join();
    let _ = bot_loop.join();

    log_info!("Exited");
}
