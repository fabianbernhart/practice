use std::{
    fs,
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

pub struct ThreadPool;

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "The size of pools needs to be bigger than 0");
        ThreadPool
    }

    pub fn execute<F, T>(&self, f: F)
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
    }
}

#[derive(Debug, Clone)]
struct Connection {
    app: Router,
}

impl Connection {
    pub fn new(_app: Router) -> Self {
        Self { app: _app }
    }

    fn handle_routes(self, request_line: &String) -> (&'static str, &'static str) {
        match request_line.as_str() {
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
            "GET /sleep HTTP/1.1" => {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", "hello.html")
            }

            _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
        }
    }

    pub fn handle_connection(self, mut stream: TcpStream) -> std::io::Result<()> {
        let buf_reader = BufReader::new(&stream);
        let http_request: Vec<String> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let request_line = &http_request[0];

        println!("{request_line}");

        let (status_line, filename) = self.handle_routes(request_line).clone();

        let contents = fs::read_to_string(filename)?;
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes())?;

        Ok(())
    }
}

/// Serves the app/router and the TcpListener
///
/// ```
///let listener = TcpListener::bind("127.0.0.1:7878")?;
///let app: Router = Router::new().route("GET / HTTP/1.1", MethodRouter::GET, get_number);
///
///serve(listener, app)?;
///
///
/// ```
///
pub fn serve(listener: TcpListener, app: Router) -> std::io::Result<()> {
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => pool.execute(move || {
                let connection = Connection::new(Router::new());
                connection.handle_connection(stream);
            }),
            Err(e) => {
                println!("Connection interrupted {e}")
            }
        }
    }
    Ok(())
}

/// Http-Request Methods
pub enum MethodRouter {
    POST,
    GET,
    PUT,
}

#[derive(Debug, Clone)]

pub struct Router {
    routes: Vec<String>,
}

impl Router {
    pub fn new() -> Self {
        let routes = Vec::new();

        Self { routes: routes }
    }

    pub fn route<T: Fn() -> io::Result<()>>(
        mut self,
        path: &str,
        method_router: MethodRouter,
        f: T,
    ) -> Self {
        let _ = f();
        let _ = method_router;

        self.routes.push(String::from(path));

        return self;
    }
}
