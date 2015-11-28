extern crate rethink;
use rethink::net;
use rethink::ast::*;
use rethink::proto::*;

fn main() {
    let stream = net::connect();
    match stream {
        Some(mut s) => net::read_query_test(&mut s),
        //Some(mut s) => net::write_query_test(&mut s),
        None => println!("Never established handshake... exiting...")
    }

    //let g = Term {tt:TermType::EQ, st: "still a test"};

}
