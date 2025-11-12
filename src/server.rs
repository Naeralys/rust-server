use core::time;
use std::net::TcpStream;
use std::sync::mpsc::channel;
use std::thread;
use std::{net::TcpListener, sync::mpsc};
use std::io::Read;

const LOCAL: &str = "127.0.0.1:3000";
const MSG_SIZE: usize = 32;

// Creates a new tcp listener and accepts connections
fn create_server () -> TcpListener {
    let listener = TcpListener::bind(LOCAL).expect("Failed to set up tcp listener");
    listener.set_nonblocking(true).expect("Failed to set nonblocking on tcplistener");
    
    println!("Listener was successfully set up");
    return listener;
}

fn handle_connection (mut stream: TcpStream) {
    let ( sender, _ ) = channel();
    let client = thread::spawn(move || {
        let mut buffer = [0; MSG_SIZE];
        println!("{:?}", sender.send(stream.read_exact(&buffer)))
    });

    println!("Reading...");
    stream.read_exact(&mut buffer).expect("Failed to parse buffer content");
}

pub fn run() {
    let server = create_server();

    for stream in server.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(_e) => ()
        }

        thread::sleep(time::Duration::from_millis(100));
    }

}