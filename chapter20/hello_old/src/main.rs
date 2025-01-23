use std::{
    fs,
    io::{prelude::*, BufReader}, 
    net::{TcpListener, TcpStream},
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // bind returns a Result<T,E>
    // TcpListener returns iterator that gives sequence of streams

    // Process each connection in turn and produce a series of streams for us to handle
    for stream in listener.incoming()
    {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection2(mut stream: TcpStream)
{
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader       // to collect lines of request the browser sensd to server
        .lines()                                // returns iterator of Result<String, std::io::Error>
                                                // splits stream of data whenever it sees a newline byte
        .map(|result| result.unwrap())          // to get each string, map and unwrap each Result
        .take_while(|line| !line.is_empty())
        .collect();
    
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    // holds the success message's data
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");      
    stream.write_all(response.as_bytes()).unwrap(); // as_bytes: convert response's string data to bytes
                                                    // write_all method on stream takes &[u8] and sends those bytes directly down to connection
}

fn handle_connection3(mut stream: TcpStream) 
{
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap(); // only looks at first line of HTTP request

    // Check if request_line is equal to the request line of GET request to / path
    // If it does, if block returns content of our HTML file
    if request_line == "GET / HTTP/1.1"     
    {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    } 
    // Received some other request.
    // Respond with error status code
    else 
    {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    }
}

fn handle_connection(mut stream: TcpStream) 
{
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap(); // only looks at first line of HTTP request

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" 
    {
        ("HTTP/1.1 200 OK", "hello.html")
    } 
    else 
    {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}