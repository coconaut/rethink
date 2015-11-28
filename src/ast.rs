extern crate byteorder;
use proto::{TermType, QueryType};
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;
use std::mem;
use self::byteorder::{WriteBytesExt, LittleEndian, BigEndian};
use net::*;

// use these to maintain vec
// enum MetaTerm {
//     _Term(Term),
//     _Table(Table),
//     _DB(DB)
// }

pub struct Term {
    db: DB,
    tt: TermType,
    args: &'static str,
    //optargs: &'static str,
    // TODO: and need to deal with various compose overloads -> may need that enum after all...?
    //chain: Vec<Term>
}

impl Term {
    pub fn run(self) {
        let start = QueryType::START as u32;
        let token: u64 = 1;

        let composed = self.compose();
        //[1,[15, [[14, [\"DeppFans\"]], \"Animals\"]],{}]
        let wrapped = format!("[{0}, {1}, {{}}]", start, composed);
        let bytes = wrapped.into_bytes();

        // take a slice
        let mut q = &bytes[..];
        let len = q.len();
        let mut buffer = [0; 100];

        let ref mut stream = self.db.r.connection.stream.unwrap();

        let _ = stream.write_u64::<BigEndian>(token);
        let _ = stream.write_u32::<LittleEndian>(len as u32);
        let _ = stream.write(q);
        let size = stream.read(&mut buffer).unwrap();
        let msg = str::from_utf8(&mut buffer).unwrap();
        println!("{}", msg);
    }

    fn compose(&self) -> String {
        let db_expr = self.db.compose();
        // need to go forward in chain: for loop and wrap
        // for now, just using this term!
        //[15, [[14, [\"DeppFans\"]], \"Animals\"]]
        let expressed = format!("[{0}, [{1}, \"{2}\"]]", self.tt as u32, db_expr, self.args);
        expressed
    }
}

// trait RqlQuery {
//     // operators, specific qb functs
//
// }
// impl on either term


// let r = R.new(stream);
pub struct R {
    pub connection: Connection
}

impl R {
    // static
    pub fn new() -> R {
        let r = R {connection : Connection::new()};
        r
    }

    pub fn connect(&mut self) {
        self.connection.connect();
    }

    pub fn db(self, db_name: &'static str) -> DB {
        let db = DB {r: self, tt: TermType::DB, name: db_name};
        db
    }

    // TODO: send query through connection (incr token, send through stream)
    // pub fn send(query: &[u8]) {
    //
    // }

    //TODO: recv response (as string at this level? have connection -> stream read?)
}

pub struct DB {
    r: R,
    tt: TermType,
    name: &'static str
}

impl DB {
    pub fn table(self, table_name: &'static str) -> Term {
        let t = Term {db: self, tt: TermType::TABLE, args: table_name};
        t
    }

    pub fn compose(&self) -> String {
        let db_expr = format!("[{0}, [\"{1}\"]]", self.tt as u32, self.name);
        db_expr
    }
}
