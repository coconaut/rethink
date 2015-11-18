extern crate rethink;
use rethink::handshake;


fn main() {
    let stream = handshake::connect();
    match stream {
        Some(mut s) => handshake::query(&mut s),
        None => println!("Never established handshake... exiting...")
    }
}
