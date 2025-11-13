use std::{io::{self}, net::TcpStream, sync::mpsc, thread};
use std::io::{Write};
use std::sync::mpsc::{Receiver, Sender};

const LOCAL: &str = "127.0.0.1:3000";

fn handle_send_to_server (rx: Receiver<String>, mut client: TcpStream) {
    thread::spawn(move || loop {
        // Retrieve message from channel and send to
        // socket/server if not empty
        let msg = &rx.recv().unwrap();
        if msg.chars().count() > 0 {
            write!(client, "{:?}", msg).unwrap();
        }
    });
}

fn handle_write_message (tx: Sender<String>) {
    println!("Write a Message:");
    // Handle write message and send to channel
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("reading from stdin failed");
        let msg = buff.trim().to_string();

        // Quit on command or fail
        if msg == ":quit" || tx.send(msg).is_err() {break}
    }
}

pub fn run() {
    // Set up client and connect to the server
    let client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("failed to initiate non-blocking");

    // Set up channel to handle messaging between threads 
    let (tx, rx) = mpsc::channel::<String>();

    handle_send_to_server(rx, client);
    handle_write_message(tx);
}

