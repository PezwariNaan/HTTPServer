use HTTPServer::ThreadPool;
use std::{
    path::Path,
    fs,
    str,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

fn check_logfile_path(file_path: &str) {
    let file = Path::new(&file_path);
    
    if !file.exists() {
        if let Err(err) = fs::create_dir_all(file.parent().unwrap()) {
            eprintln!("Failed to Create Directory: {}.\nCheck Permissions", err);
            return;
        }
        if let Err(err) = fs::File::create(file) {
            eprintln!("Failed to Create File: {}.\nCheck Permissions", err);
            return;
        }
    }

    return;
}

fn auto_refresh() {
    // Code here
}

fn handle_connection(mut stream: TcpStream, file_path: &str,) {
    let mut buffer = [0; 1024];
    let number_of_bytes = stream.read(&mut buffer).unwrap();
    let request_log_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(file_path).unwrap();
    let request_details = str::from_utf8(&buffer[..number_of_bytes]);
    let ip = stream.peer_addr().unwrap();

    writeln!(&request_log_file, "Client: {ip}\n{:#?}\n", request_details)
        .expect("Failed to write to log. Check Permissions.");

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    return;
}

fn main() {
    let file_path = "logs/requests.log";
    check_logfile_path(&file_path);

    let listener = TcpListener::bind("127.0.0.1:9001").unwrap();
    let pool = ThreadPool::new(4);
    println!("[*]Server Started on Port 9001");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream, file_path);
        });
    }

    println!("Shutting down.");
}

