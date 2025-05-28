use std::{
    io::{self},
    net::TcpListener,
};

use http_server::{serve, MethodRouter, Router};

fn get_number() -> io::Result<()> {
    println!("hihi");

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    let app: Router = Router::new().route("GET / HTTP/1.1", MethodRouter::GET, get_number);

    println!("{:#?}", app);

    serve(listener, app)?;

    Ok(())
}
