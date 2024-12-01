use std::{fs, io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}};

use jals_rust_web::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| { // Closure to execute as a job in the thread pool
            println!("Connection established!");
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // `BufReader` instance wraps a reference to the `stream`
    // and adds buffering by managing calls to the `std::io::Read` trait
    let buf_reader = BufReader::new(&stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let request_ar: Vec<&str> = request_line.split_whitespace().collect();

    let (status, filename): (&str, &str) = if request_ar[1] == "/" {
        ("HTTP/1.1 200 OK", "src/index.html")
    } else if request_ar[1] == "/status" {
        ("HTTP/1.1 200 OK", "src/index.html")
    } else if request_ar[1].starts_with("/") && request_ar[1].chars().count() == 9  {
        ("HTTP/1.1 200 OK", "src/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "src/404.html")
    };

    let content = fs::read_to_string(filename).unwrap();
    let content_length = content.len();

    let response = format!("{status}\r\nContent-Length: {content_length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();

    println!("Request: {request_line}");
}