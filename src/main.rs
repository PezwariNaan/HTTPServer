use std::{
    fs,
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
};

fn handle_connection(mut stream: TcpStream) {
    let buffer_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buffer_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", http_request);

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();
    
    let response = 
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    stream.write_all(response.as_bytes()).unwrap();
}  

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9001").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let ip = stream.peer_addr().unwrap();

        println!("Connection Established\nClient: {}", ip);

        handle_connection(stream);
    }
}

