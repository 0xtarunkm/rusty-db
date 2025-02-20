use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6380").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection")
            }
            Err(e) => {
                println!("error: {e}")
            }
        }
    }
}