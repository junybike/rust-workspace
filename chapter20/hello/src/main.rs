use std::{
    fs,
    io::{prelude::*, BufReader}, 
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // bind returns a Result<T,E>
    // TcpListener returns iterator that gives sequence of streams

    let pool = ThreadPool::new(4);

    // Process each connection in turn and produce a series of streams for us to handle
    for stream in listener.incoming()
    {
        let stream = stream.unwrap();
        pool.execute(|| {handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) 
{
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap(); // only looks at first line of HTTP request

    let (status_line, filename) = match &request_line[..]
    {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}