use std::io::{BufRead, BufReader, BufWriter, Write};
use std::thread;
use std::net::TcpListener;

fn main() {
    struct Server {
        url: String,
        port: i32,
    }

    let server = Server {
        url: "localhost".to_string(),
        port: 8080,
    };

    // // bind the server to the specified address // associa
    let listener = TcpListener::bind(format!("{}:{}", server.url, server.port)).unwrap(); // return the server or panic if it fails
    println!("{}", listener.local_addr().unwrap()); // return the local socket address of the server

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection received");

                thread::spawn(move || { //new thread created for each new connection
                    let stream_clone = stream.try_clone().unwrap();
                    let reader = BufReader::new(stream);
                    let mut writer = BufWriter::new(stream_clone);

                    for i in reader.lines() {
                        let msg = "Pong";
                        println!("{:?}", i);
                        writer.write_all(msg.as_bytes()).err();
                        writer.flush().unwrap();
                    }
                });
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
}
