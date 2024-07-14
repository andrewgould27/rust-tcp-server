# rust-tcp-server
A basic TCP server written in Rust. 

This TCP server listens on port `8888` and returns a stream of bytes from an HTML file located in the `./src/html` directory. 
In `main.rs`, you'll find the `match` function that maps the request line of the incoming HTTP GET request to a location of a file. 

This server uses a `ThreadPool` to allow the handling of simultaneous requests. The `ThreadPool` has a group of `Workers` that handle a specific thread and executes the `Job` that's handed to them. Using this approach, the server won't spawn an unwieldy amount of threads that the host cannot handle, but can
also handle many requests at once! 

This Rust TCP server is extremely rudimentary, and as such, should never be used for anything important. 

## Starting the server
To start the server, run `cargo run`. 

## Default settings 
The server, by default, listens on `127.0.0.1:8888`.
By default, `4` threads are allocated to the `ThreadPool`. 