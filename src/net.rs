extern crate byteorder;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;
use proto::{Version, Protocol};
use self::byteorder::{WriteBytesExt, LittleEndian, BigEndian};

// hardcoding for now...
const addr: &'static str = "127.0.0.1:28015";


fn log(s: &str) {
    println!("==> {}", s);
}


// need to connect via TCP, form handshake, retreive success.
pub fn connect() -> Option<TcpStream> {
    let stream_res = TcpStream::connect(addr);
    let s =
        match stream_res {
            Ok(mut stream) => {
                let success = handshake(&mut stream);
                match success {
                    true => Some(stream),
                    false => None
                }
            },
            Err(e) => {
                println!("Unable to connect: {}", e);
                None
            }
        };
    s
}


// TODO: accept auth key
pub fn handshake(stream: &mut TcpStream) -> bool {
    log("performing handshake");
    let version = Version::V0_4 as u32;
    let protocol = Protocol::JSON as u32;
    let mut buffer = [0; 100];
    let _ = stream.write_u32::<LittleEndian>(version); // note: write returns a Result<usize>
    let _ = stream.write_u32::<LittleEndian>(0);
    let _ = stream.write_u32::<LittleEndian>(protocol);

    // TODO: add timeouts?
    let size = stream.read(&mut buffer).unwrap();
    let mut msg_bytes = &buffer[0..(size - 1)];
    let msg = str::from_utf8(&mut msg_bytes);
    match msg {
        Ok(m) if m == "SUCCESS" => {
            println!("{}", m);
            true
        },
        Ok(x) => {
            println!("Auth failed: {}!!!", x);
            false
        },
        Err(e) => panic!("Error: {}", e)
    }
}

pub fn read_query_test(stream: &mut TcpStream){
    log("quering the database for a test read");
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
    log("quering the database for a test write");
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
