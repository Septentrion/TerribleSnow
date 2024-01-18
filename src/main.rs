use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};
use std::fs;

// Multi-thread
use std::time::Duration;
use std::thread;
//use http_server_tutorial::ThreadPool;

#[allow(dead_code)]
#[derive(Clone, PartialEq, Eq, Hash)]
enum Status {
    Valid(String),
    Fail(String),
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct HttpStatus;

impl HttpStatus {
    pub const OK: &'static str = "HTTP/1.1 200 OK";
    pub const NOT_FOUND: &'static str = "HTTP/1.1 404 NOT FOUND";
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        //thread::spawn(|| {
        //pool.execute(|| {
            println!("Connexion");
            handle_connection(stream);
        //});

    }
}

fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        println!("Request Method : {:?}", http_request[0]);

        let method: Vec<_> = http_request[0].split(' ').collect();
        
        if method[0] == "GET" {
            let (status_line, filename) = match method[1] {
                "/" => (HttpStatus::OK, "hello.html"),
                "/sleep" => {
                    thread::sleep(Duration::from_secs(50));
                    (HttpStatus::OK, "hello.html")
                },
                _ => (HttpStatus::NOT_FOUND, "404.html")
            };

            let content = fs::read_to_string(filename).unwrap();
            let response_length = content.len();
            println!("{status_line}\r\nContent-Length: {response_length}\r\n\r\n{content}");
            let response = format!("{status_line}\r\nContent-Length: {response_length}\r\n\r\n{content}");
            stream.write_all(response.as_bytes()).unwrap();
        } else {
            //
        }        
}
