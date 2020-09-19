use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;


fn handle_client(stream: TcpStream) {
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer);
    println!("incoming: {}", buffer);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }
    
    // close the socket server
    drop(listener);
}

