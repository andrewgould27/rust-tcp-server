# rust-tcp-server
A basic TCP server written in Rust. 

This TCP server listens on port `8888` and returns a stream of bytes from an HTML file located in the `./src/html` directory. 
In `main.rs`, you'll find the `match` function that maps the request line of the incoming HTTP GET request to a location of a file. 

This server uses a `ThreadPool` to allow the handling of simultaneous requests. The `ThreadPool` has a group of `Workers` that handle a specific thread and executes the `Job` that's handed to them. Using this approach, the server won't spawn an unwieldy amount of threads that the host cannot handle, but can
also handle many requests at once! 

This Rust TCP server is extremely rudimentary, and as such, should never be used for anything important. 

## Starting the server
To start the server, run `cargo run`. 

Alternatively, run `cargo build` and run the executable directly inside the `./target/debug/` directory.

## Arguments
| Flag          | Value | Explanation                                           |
| ----------    | ----- | ----------------------------------------------------- |
| -p / -port    | u32   | Port that the TCP server will listen on               |
| -t / -threads | usize | Number of threads to be available to the `ThreadPool` | 

For example, to start the server on port `8080` and allocate `16` threads, you'd run:
`cargo run -- -p 8080 -t 16`
or
`./target/debug/rust-tcp-server.exe -p 8080 -t 16`

## Default settings 
The server, by default, listens on `127.0.0.1:8888`.

By default, `4` threads are allocated to the `ThreadPool`. 

