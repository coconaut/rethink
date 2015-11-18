extern crate rethink;
use rethink::handshake;


fn main() {
    let stream = handshake::connect();
    match stream {
        Some(mut s) => handshake::read_query_test(&mut s),
        //Some(mut s) => handshake::write_query_test(&mut s),
        None => println!("Never established handshake... exiting...")
    }
}
