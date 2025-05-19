use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream)?;
            }
            Err(e) => {
                println!("Connection interupted {e}")
            }
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    
    let mut buffer = [0;1024];

    match stream.read(&mut buffer) {
        Ok(size) => {
            println!("Bytes: {size}");

            let request = String::from_utf8_lossy(&buffer[..size]);
            println!("Request {request}");

            let request_line = request.lines().next().unwrap_or("");

            println!("{request_line}");


            let parts = request.split_whitespace();

            println!("{:#?}", parts)


        }
        Err(err) => {}

    }



    Ok(())
}


