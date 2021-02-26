use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    println!("Starting to read from incoming...");
    let mut name = String::new();
    reader
        .read_line(&mut name)
        .expect("Couldn't read from buffer");

    println!(
        "{} Pinged us! Let's answer back with a 'Hello, {}'",
        name, name
    );
    let name_as_bytes = format!("Hello {}!", name).as_bytes().to_owned();
    stream
        .write(&name_as_bytes)
        .expect("Couldn't write to stream x_x");
}

fn main() {
    let address = "localhost";
    let port = 3000;
    let address_port = format!("{}:{}", address, port);
    let socket_server = TcpListener::bind(&address_port).unwrap();
    println!("Started listening on: {}", &address_port);

    for stream in socket_server.incoming() {
        let stream = stream.unwrap();
        std::thread::spawn(move || {
            handle_client(stream);
        });
        // match stream {
        //     Ok(s) => {
        //         println!("Received request:");
        //         handle_client(s);
        //     }
        //     Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
        //         println!("Err: {}", e);
        //         continue;
        //     }
        //     Err(e) => {
        //         panic!("Error on incoming connection x_x: {}", e)
        //     }
        // }
    }
}
