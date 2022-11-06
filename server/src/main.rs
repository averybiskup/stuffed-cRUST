use std::net::TcpListener;

fn main() {
    // unwrap stops program if errors happen
    // listener becomes a sequence of streams of type TcpStream
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // process each connection and allow us to handle each stream
    for stream in listener.incoming() {

        // unwrap -> terminate if stream has any errors
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
