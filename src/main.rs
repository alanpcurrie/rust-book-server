use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    listener.incoming().for_each(|stream| match stream {
        Ok(stream) => {
            println!("Connection established");
            handle_connection(stream);
        }
        Err(err) => eprintln!("Error accepting connection: {}", err),
    });
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    const GET: &str = "GET / HTTP/1.1";
    const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND";
    const FOUND: &str = "HTTP/1.1 200 OK";

    let (status_line, filename) = if request_line == GET {
        (FOUND, "index.html")
    } else {
        (NOT_FOUND, "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
