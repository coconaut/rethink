mod test;
extern crate byteorder;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;
use self::byteorder::{WriteBytesExt, LittleEndian};

// hardcoding for now...
const addr: &'static str = "127.0.0.1:28015";

// Version:
const V0_1: u32 = 0x3f61ba36;
const V0_2: u32 = 0x723081e1; // Authorization key during handshake
const V0_3: u32 = 0x5f75e83e; // Authorization key and protocol during handshake
const V0_4: u32 = 0x400c2d20; // Queries execute in parallel


// Protocol: the protocol to use after the handshake, specified in V0_3
const PROTOBUF: u32  = 0x271ffc41;
const JSON: u32      = 0x7e6970c7;


// need to connect via TCP, form handshake, retreive success.
pub fn connect() {
    let mut buffer = [0; 100];
    let mut stream = TcpStream::connect(addr).unwrap(); // TODO: handle result here, don't just unwrap!
    let _ = stream.write_u32::<LittleEndian>(V0_3); // note: write returns a Result<usize>
    stream.write_u32::<LittleEndian>(0);
    stream.write_u32::<LittleEndian>(JSON);
    stream.read(&mut buffer);
    let msg = str::from_utf8(&mut buffer);
    match msg {
        Ok(m) => println!("{}", m),
        Err(e) => panic!("Error: {}", e),
    };
}
