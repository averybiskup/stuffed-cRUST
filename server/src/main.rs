use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}
};

fn main() {
    // unwrap stops program if errors happen
    // listener becomes a sequence of streams of type TcpStream
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // process each connection and allow us to handle each stream
    for stream in listener.incoming() {

        // unwrap -> terminate if stream has any errors
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()                                // returns iterator
        .map(|result| result.unwrap())          // map and unwrap each result
        .take_while(|line| !line.is_empty())
        .collect();                             // add lines to vec

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
