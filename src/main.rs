use std::{
    fs,
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
};

fn handle_connection(mut stream: TcpStream) {
    let buffer_reader = BufReader::new(&mut stream);
    let request_line = buffer_reader.lines().next().unwrap().unwrap();

    println!("{}", request_line);

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
    stream.write_all(response.as_bytes()).unwrap();
}  

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9001").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let ip = stream.peer_addr().unwrap();

        println!("Connection Established\nClient: {}\n", ip);

        handle_connection(stream);
    }
}

