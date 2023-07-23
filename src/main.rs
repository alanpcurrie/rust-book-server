extern crate rust_book_server; // Import the `rust_book_server` crate.
use rust_book_server::ThreadPool; // Import the `ThreadPool` struct from the crate.

use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a TCP listener bound to "127.0.0.1:7878".
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // Create a ThreadPool with a size of 4 threads.
    let pool = ThreadPool::new(4);

    // Loop over incoming TCP streams, taking up to 2 connections.
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        // Execute a closure in a separate thread from the ThreadPool for each incoming stream.
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    // The loop ends when 2 connections have been processed.
    // At this point, all incoming streams have been processed by the pool.

    println!("Shutting down.");
}

// Function to handle an individual connection (stream).
fn handle_connection(mut stream: TcpStream) {
    // Create a buffer to read the request from the stream.
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // Define two types of requests: "GET / HTTP/1.1\r\n" and "GET /sleep HTTP/1.1\r\n".
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // Check if the request starts with "GET /" or "GET /sleep".
    // Based on the request, set the response status line and the filename to be served.
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html") // For "GET /" requests, serve "index.html".
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5)); // Simulate a delay for "GET /sleep" requests.
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html") // After sleep, serve "index.html".
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html") // For other requests, serve "404.html".
    };

    // Open the specified file and read its contents into a string.
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Combine the status line and file contents to create the HTTP response.
    let response = format!("{}{}", status_line, contents);

    // Write the response to the stream (sending it back to the client).
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
