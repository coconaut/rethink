extern crate byteorder;
use proto::{TermType, QueryType};
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;
use std::mem;
use self::byteorder::{WriteBytesExt, LittleEndian, BigEndian};

// use these to maintain vec
// enum MetaTerm {
//     _Term(Term),
//     _Table(Table),
//     _DB(DB)
// }

pub struct Term<'a> {
    db: DB<'a>,
    tt: TermType,
    args: &'static str,
    //optargs: &'static str,
    // TODO: and need to deal with various compose overloads -> may need that enum after all...?
    //chain: Vec<Term>
}
impl<'a> Term<'a> {
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

        let ref mut stream = self.db.r.connection;

        let _ = stream.write_u64::<BigEndian>(token);
        let _ = stream.write_u32::<LittleEndian>(len as u32);
        let _ = stream.write(q);
        let size = stream.read(&mut buffer).unwrap();
        let msg = str::from_utf8(&mut buffer).unwrap();
        println!("{}", msg);
    }

    fn compose(&self) -> String {
        let db_expr = compose(&self.db);
        // need to go forward in chain: for loop and wrap
        // for now, just using this term!
        //[15, [[14, [\"DeppFans\"]], \"Animals\"]]
        let expressed = format!("[{0}, [{1}, \"{2}\"]]", TermType::TABLE as u32, db_expr, self.args);
        expressed
    }
}

// trait RqlQuery {
//     // operators, specific qb functs
//
// }
// impl on either term


// let r = R.new(stream);
pub struct R<'a> {
    pub connection: &'a mut TcpStream
}
impl<'a> R<'a> {
    pub fn new(&mut self, stream: &'a mut TcpStream) -> &mut R<'a> {
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
    pub fn table(self, table_name: &'static str) -> Term<'a> {
        let t = Term {db: self, tt: TermType::TABLE, args: table_name};
        t
    }
}

pub fn compose(db: &DB) -> String {
    let db_expr = format!("[{0}, [\"{1}\"]]", TermType::DB as u32, db.name);
    db_expr
}
