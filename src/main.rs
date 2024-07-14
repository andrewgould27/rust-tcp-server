use std::{
    env,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use rust_tcp_server::ThreadPool;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut port: u32 = 8888;
    let mut num_threads: usize = 4;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-p" | "-port" => {
                if i + 1 < args.len() {
                    port = args[i + 1].parse::<u32>().unwrap();
                }
            }
            "-t" | "-threads" => {
                if i + 1 < args.len() {
                    num_threads = args[i + 1].parse::<usize>().unwrap();
                }
            }
            _ => ()
        }

        i += 1;
    }

    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();
    let thread_pool = ThreadPool::new(num_threads);

    println!("Listening on port {port}.");
    println!("Thread pool initialized with {num_threads} threads.");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread_pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down!");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Map for routes to files
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "./src/html/index.html"),
        "GET /about HTTP/1.1" => ("HTTP/1.1 200 OK", "./src/html/about.html"),
        "GET /test HTTP/1.1" => ("HTTP/1.1 200 OK", "./src/html/test.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "./src/html/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}