use std::net::TcpListener;
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use http_server::http_status::StatusCode;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)?;
    }

    Ok(())
}

pub fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let buf_read = BufReader::new(&stream);

    let request_line = buf_read.lines().next().unwrap().unwrap();

    let (status_line, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => (StatusCode::OK.as_str(), "hello.html"),

        _ => (StatusCode::Notfound.as_str(), "404.html"),
    };

    let contents = fs::read_to_string(filename)?;
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

    Ok(())
}
