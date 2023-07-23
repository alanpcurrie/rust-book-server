# Rust Web Server

This is a simple web server written in Rust that listens for incoming HTTP requests on "127.0.0.1:7878". The server handles connections concurrently using a custom thread pool implementation.

## Features

- Concurrent handling of incoming connections with a thread pool.
- Serves different responses based on the content of the HTTP request.
- Provides a simple error page when the maximum connection limit is exceeded.

## Getting Started

### Prerequisites

To run this project, you need to have Rust and Cargo installed on your system. If you haven't installed Rust, you can do so by following the instructions on the official website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Build and Run

Clone this repository to your local machine:

```bash
git clone https://github.com/your_username/rust-web-server.git
```

```bash
cd rust-web-server
```

```bash
cargo build --release
```

```bash
cargo run --release
```

The server will start listening for incoming connections on "127.0.0.1:7878".
Usage

To test the server, open your web browser and navigate to <http://127.0.0.1:7878/> or <http://127.0.0.1:7878/sleep> to simulate a delayed response.

The server serves different pages based on the request path. For the root path ("/"), it serves "index.html". For the "/sleep" path, it simulates a delay of 5 seconds before serving "index.html". For any other path, it returns a "404 Not Found" error page.
Limitations

This server is intended for educational and testing purposes and may not be suitable for production use.

The error handling is minimal and may not handle all possible error scenarios robustly.
The server does not support HTTPS or any form of authentication.

[Rust Book 2021: Final Project: A Web Server](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)

## License

This project is licensed under the MIT License - see the LICENSE file for details.
Acknowledgments
