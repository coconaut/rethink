extern crate byteorder;
use proto::{TermType, QueryType};
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;
use self::byteorder::{WriteBytesExt, LittleEndian, BigEndian};

// use these to maintain vec
// enum MetaTerm {
//     _Term(Term),
//     _Table(Table),
//     _DB(DB)
// }

pub struct Term<'a> {
    r: &'a mut R<'a>,
    db: &'a mut DB<'a>,
    tt: TermType,
    args: &'static str,
    //optargs: &'static str,
    // TODO: and need to deal with various compose overloads -> may need that enum after all...?
    //chain: Vec<Term>
}
// impl<'a> Term<'a> {
//     pub fn run(&self) {
//         let start = QueryType::START as u32;
//         let token: u64 = 1;
//         let composed = self.compose();
//         //[1,[15, [[14, [\"DeppFans\"]], \"Animals\"]],{}]
//         let wrapped = format!("[{0}, {1}, {{}}]", start, composed);
//         let q = wrapped.as_str().as_bytes();
//         let len = q.len();
//         let mut buffer = [0; 100];
//         let _ = self.r.connection.write_u64::<BigEndian>(token);
//         let _ = self.r.connection.write_u32::<LittleEndian>(len as u32);
//         let _ = self.r.connection.write(q);
//         let size = self.r.connection.read(&mut buffer).unwrap();
//         let msg = str::from_utf8(&mut buffer).unwrap();
//         println!("{}", msg);
//     }
//
//     pub fn compose(&self) -> String {
//         let db_expr = self.db.compose();
//         // need to go forward in chain: for loop and wrap
//         // for now, just using this term!
//         //[15, [[14, [\"DeppFans\"]], \"Animals\"]]
//         let expressed = format!("[{0}, [{1}, \"{2}\"]]", self.tt as u32, db_expr, self.args);
//         expressed
//     }
// }

// trait RqlQuery {
//     // operators, specific qb functs
//
// }
// impl on either term


// let r = R.new(stream);
pub struct R<'a> {
    connection: &'a mut TcpStream
}
impl<'a> R<'a> {
    pub fn new(&mut self, stream: &'a mut TcpStream) -> &'a R {
        self.connection = stream;
        self
    }

    pub fn db(&'a mut self, db_name: &'static str) -> DB<'a> {
        let db = DB {r: self, tt: TermType::DB, name: db_name};
        db
    }
}

pub struct DB<'a> {
    r: &'a mut R<'a>,
    tt: TermType,
    name: &'static str
}
impl<'a> DB<'a> {
    pub fn table(&'a mut self, table_name: &'static str) -> Term<'a> {
        let t = Term {r: self.r, db: self, tt: TermType::TABLE, args: table_name};
        t
    }

    pub fn compose(self) -> String {
        let db_expr = format!("[{0}, [\"{1}\"]]", self.tt as u32, self.name);
        db_expr
    }
}
