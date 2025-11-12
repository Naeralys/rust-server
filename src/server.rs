use core::time;
use std::fs::OpenOptions;
use std::net::TcpStream;
use std::str::from_utf8;
use std::thread;
use std::{net::TcpListener};
use std::io::{ErrorKind, Read, Write};

const LOCAL: &str = "127.0.0.1:3000";
const MSG_SIZE: usize = 32;

// Creates a new tcp listener and accepts connections
fn create_server () -> TcpListener {
    let listener = TcpListener::bind(LOCAL).expect("Failed to set up tcp listener");
    listener.set_nonblocking(true).expect("Failed to set nonblocking on tcplistener");
    
    println!("Listener was successfully set up");
    return listener;
}

fn append_to_file (msg: &str) {
    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open("logfile.txt") {
        if let Err(err) = writeln!(&mut file, "{}", msg){
            println!("Failed to write message to buffer: {}", err)
        };
    }
}

// Handle a connection - will print and append message to file
fn handle_connection (mut stream: TcpStream) {
    println!("Client connected");

    thread::spawn(move || {
        let mut buffer = vec![0; MSG_SIZE];
        loop {
            match stream.read(&mut buffer) {
                Ok(len) => {
                    // If no content was sent
                    if len == 0 {
                        break;
                    }

                    // Format to utf8 and print in console
                    let msg = from_utf8(&buffer[..len]).expect("Failed to format body");
                    println!("{}", msg);

                    // We want to save the message in a file
                    append_to_file(msg);
                }, 
                Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                Err(_) => {
                    break;
                }
            }
        }
        println!("Disconnected");
    });
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