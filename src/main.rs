extern crate rethink;
//extern crate byteorder;
use rethink::handshake;
//use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};


fn main() {
    handshake::connect();

}
