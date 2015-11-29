extern crate byteorder;
use proto::{TermType, QueryType};
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;
use std::mem;
use std::boxed;
use self::byteorder::{WriteBytesExt, LittleEndian, BigEndian};
use net::*;

// use these to maintain vec
// enum MetaTerm {
//     _Term(Term),
//     _Table(Table),
//     _DB(DB)
// }


// -------------------------------------------
// RqlQuery: this is the base trait for all query terms
// -------------------------------------------
pub trait RqlQuery {
    // each term struct must have way to overload build method for the RqlQuery syntax
    fn build(&self) -> String;

    // standard run function: accepts a connection -> can have specifics that use defaults
    fn run(&self, c: Connection) {
        let start = QueryType::START as u32;
        let token: u64 = 1;

        let composed = self.build();
        //[1,[15, [[14, [\"DeppFans\"]], \"Animals\"]],{}]
        let wrapped = format!("[{0}, {1}, {{}}]", start, composed);
        println!("about to send: {0}", wrapped);
        let bytes = wrapped.into_bytes();

        // take a slice
        let mut q = &bytes[..];
        let len = q.len();
        let mut buffer = [0; 100];

        let ref mut stream = c.stream.unwrap();

        let _ = stream.write_u64::<BigEndian>(token);
        let _ = stream.write_u32::<LittleEndian>(len as u32);
        let _ = stream.write(q);
        let size = stream.read(&mut buffer).unwrap();
        let msg = str::from_utf8(&mut buffer).unwrap();
        println!("{}", msg);
    }
}

// -------------------------------------------
// Term structs -> don't worry about compose, just build
// -------------------------------------------

// basic Term
pub struct Term<T: RqlQuery> {
    tt: TermType,
    args: &'static str,
    //options: &'static str, keyword args, etc. may be better way than string!
    prev: Option<Box<T>> // TODO: this will probably be generic! T: RqlQuery
}

// impl Term {
//     fn run(self) {
//         let start = QueryType::START as u32;
//         let token: u64 = 1;
//
//         let composed = self.build();
//         //[1,[15, [[14, [\"DeppFans\"]], \"Animals\"]],{}]
//         let wrapped = format!("[{0}, {1}, {{}}]", start, composed);
//         let bytes = wrapped.into_bytes();
//
//         // take a slice
//         let mut q = &bytes[..];
//         let len = q.len();
//         let mut buffer = [0; 100];
//
//         let ref mut stream = self.db.r.connection.stream.unwrap();
//
//         let _ = stream.write_u64::<BigEndian>(token);
//         let _ = stream.write_u32::<LittleEndian>(len as u32);
//         let _ = stream.write(q);
//         let size = stream.read(&mut buffer).unwrap();
//         let msg = str::from_utf8(&mut buffer).unwrap();
//         println!("{}", msg);
//         // may need to return self, combinator?
//     }
// }

impl<T: RqlQuery> RqlQuery for Term<T> {
    fn build(&self) -> String {
        // TODO: make this more elegant
        match self.prev {
            None => {
                //just need to build self
                if self.args == "" {
                    format!("[{0}]", self.tt as u32)
                }
                else {
                    // TODO: JSON encode the args first!
                    format!("[{0}, [\"{1}\"]]", self.tt as u32, self.args)
                }
            }
            Some(ref prev_term) => {
                // recursively build and wrap previous terms!
                let pr = prev_term.build();
                if self.args == "" {
                    format!("[{0}, [{1}]]", self.tt as u32, pr)
                }
                else {
                    format!("[{0}, [{1}, \"{2}\"]]", self.tt as u32, pr, self.args)
                }
            }
        }
    }
}


// -------------------------------------------
// More specific RQL structs with defined functionality: db, table, etc.
// -------------------------------------------

// the root struct. useful in holding defualts.
// also contains db methods, connection methods, etc.
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
}

// the DB struct -> has table methods
pub struct DB {
    r: R,
    tt: TermType,
    name: &'static str
}

impl DB {
    pub fn table(self, table_name: &'static str) -> Table {
        Table::new(self, table_name)
    }

    //table list
    pub fn table_list(self) -> Term<DB> {
        Term {tt: TermType::TABLE_LIST, args: "", prev: Some(Box::new(self))}
    }

    //table create


    //table drop

}

impl RqlQuery for DB {
    fn build(&self) -> String {
        format!("[{0}, [\"{1}\"]]", self.tt as u32, self.name)
    }
}

pub struct Table {
    db: DB,
    tt: TermType,
    args: &'static str,
}

impl Table {
    pub fn new(db: DB, args: &'static str) -> Table {
        let t = Table {db: db, tt: TermType::TABLE, args: args};
        t
    }
}

impl RqlQuery for Table {
    fn build(&self) -> String {
        let db_expr = self.db.build();
        // need to go forward in chain: for loop and wrap
        // for now, just using this term!
        //[15, [[14, [\"DeppFans\"]], \"Animals\"]]
        let expressed = format!("[{0}, [{1}, \"{2}\"]]", self.tt as u32, db_expr, self.args);
        expressed
    }
}
