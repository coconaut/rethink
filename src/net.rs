extern crate byteorder;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;
use std::convert::AsRef;
use proto::{Version, Protocol};
use self::byteorder::{WriteBytesExt, LittleEndian, BigEndian};


pub struct Connection {
    addr: &'static str,
    pub token: u64,
    pub stream : Option<TcpStream>
}
impl Connection {
    // TODO: accept params
    pub fn new() -> Connection {
        let c = Connection {addr:"127.0.0.1:28015", token: 0, stream: None};
        c
    }

    pub fn connect(&mut self) -> bool {
        establish_connection(self);
        handshake(self)
    }
}

pub fn send_query(query: String, c: &mut Connection) {
    // get bytes (Vec)
    let bytes = query.into_bytes();

    // take a slice
    let mut q = &bytes[..];
    let len = q.len();

    match c.stream {
        None => println!("no active connection..."),
        Some(ref mut stream) => {
            // incr token
            c.token += 1;
            // write to stream
            let _ = stream.write_u64::<BigEndian>(c.token);
            let _ = stream.write_u32::<LittleEndian>(len as u32);
            let _ = stream.write(q);
        }
    }
}

pub fn read_query_response(c: &mut Connection) {
    match c.stream {
        None => println!("no active connection..."),
        Some(ref mut stream) => {
            let mut buffer = [0; 100];
            let size = stream.read(&mut buffer).unwrap();
            let msg = str::from_utf8(&mut buffer).unwrap();
            println!("{}", msg);
        }
    }
}

fn establish_connection(c: &mut Connection) {
    let stream_res = TcpStream::connect(c.addr);
    match stream_res {
        Ok(mut stream) => {
            c.stream = Some(stream);
        }
        Err(e) => {
            println!("Unable to connect: {}", e);
        }
    }
}

fn handshake(c: &mut Connection) -> bool {
    println!("performing handshake...");
    match c.stream {
        None => {
            println!("no active connection...");
            false
        }
        Some(ref mut stream) => {
            send_version_and_protocol(stream);
            let msg = read_handshake_response(stream);
            match msg.as_ref() {
                "SUCCESS" => {
                    println!("{}", msg);
                    true
                },
                x => {
                    println!("Auth failed!!! {} !!!", x);
                    false
                }
            }
        }
    }
}


// TODO: accept version and protocol params, auth token, etc.
fn send_version_and_protocol(stream: &mut TcpStream) {
    let version = Version::V0_4 as u32;
    let protocol = Protocol::JSON as u32;
    let mut buffer = [0; 100];
    let _ = stream.write_u32::<LittleEndian>(version); // note: write returns a Result<usize>
    let _ = stream.write_u32::<LittleEndian>(0);
    let _ = stream.write_u32::<LittleEndian>(protocol);
}


pub fn read_handshake_response(stream: &mut TcpStream) -> String {
    println!("reading response...");
    let mut buffer = [0;100];
    let size = stream.read(&mut buffer).unwrap();
    let mut msg_bytes = &buffer[0..(size-1)];
    let msg = str::from_utf8(&mut msg_bytes).unwrap();
    // passing as String to let buffer die
    msg.to_string()
}


pub fn read_query_test(stream: &mut TcpStream){
    println!("quering the database for a test read");
    let token: u64 = 1;
    // ReQL: r.db("DeppFans").table("Animals")
    let q = "[1,[15, [[14, [\"DeppFans\"]], \"Animals\"]],{}]".as_bytes();
    //let q = "[1, \"foo\", {} ]".as_bytes();
    let len = q.len();
    let mut buffer = [0; 100];
    let _ = stream.write_u64::<BigEndian>(token);
    let _ = stream.write_u32::<LittleEndian>(len as u32);
    let _ = stream.write(q);
    let size = stream.read(&mut buffer).unwrap();
    let msg = str::from_utf8(&mut buffer).unwrap();
    println!("{}", msg);
    // serialize query as UTF-encoded JSON -> will also need query type, opts
    // need to consider reading strategy with buffer, etc.
}

pub fn write_query_test(stream: &mut TcpStream) {
    println!("quering the database for a test write");
    let token: u64 = 2;
    // ReQL: r.db("DeppFans").table("Animals").insert(some json)
    let q =
    "[1,
        [56,
            [[15,
                [[14,
                    [\"DeppFans\"]
                ],
            \"Animals\"
            ]],
        {\"id\": 3, \"name\": \"wildebeest\"}
        ]],
    {}
    ]".as_bytes();
    //let q = "[1, \"foo\", {} ]".as_bytes();
    let len = q.len();
    let mut buffer = [0; 100];
    let _ = stream.write_u64::<BigEndian>(token);
    let _ = stream.write_u32::<LittleEndian>(len as u32);
    let _ = stream.write(q);
    let size = stream.read(&mut buffer).unwrap();
    let msg = str::from_utf8(&mut buffer).unwrap();
    println!("{}", msg);
}
