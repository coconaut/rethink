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


// -------------------------------------------
// RqlQuery: this is the base trait for all query terms
// -------------------------------------------
pub trait RqlQuery {

    // each term struct must have way to overload build method for the RqlQuery syntax
    fn build(&self) -> String;

    // standard run function: accepts a connection -> can have specifics that use defaults
    // may want to move method to the connection...
    // should return some type of result, need to parse response message
    // either err, result, or cursor...
    fn run(&self, c: &mut Connection) {
        let start = QueryType::START as u32;
        let composed = self.build();
        let query = format!("[{0}, {1}, {{}}]", start, composed);
        println!("about to send: {0}", query);
        send_query(query, c);
        read_query_response(c);
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
    prev: Option<Box<T>>
}

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
    pub connection: Option<Connection>
}

impl R {
    // static
    pub fn new() -> R {
        //let r = R {connection : Connection::new()};
        R {connection: None}
    }

    pub fn use_connection(&mut self, conn: Connection) {
        self.connection = Some(conn);
    }

    // r should also have a default db, use_db, e.g.? or would this mean r needs all of db's methods...
    // could do with a trait, then just have Option<db>, wouldn't be too difficult

    pub fn connect(&mut self) -> bool{
        match self.connection {
            None => {
                println!("no active connection");
                false
            },
            Some(ref mut c) => c.connect()
        }
    }

    // TODO: this must be mutable ref, need to deal with lifetime specifers
    pub fn db<'a>(&'a mut self, db_name: &'static str) -> DB<'a> {
        let db = DB {r: self, tt: TermType::DB, name: db_name};
        db
    }
}

// the DB struct -> has table methods
pub struct DB<'a> {
    r: &'a mut R,
    tt: TermType,
    name: &'static str
}

impl<'a> DB<'a> {
    pub fn table(self, table_name: &'static str) -> Table<'a> {
        Table::new(self, table_name)
    }

    //table list
    pub fn table_list(self) -> Term<DB<'a>> {
        Term {tt: TermType::TABLE_LIST, args: "", prev: Some(Box::new(self))}
    }

    //table create -> TODO: options!
    pub fn table_create(self, table_name: &'static str) -> Term<DB<'a>> {
        Term {tt: TermType::TABLE_CREATE, args: table_name, prev: Some(Box::new(self))}
    }

    //table drop

}

impl<'a> RqlQuery for DB<'a> {
    fn build(&self) -> String {
        format!("[{0}, [\"{1}\"]]", self.tt as u32, self.name)
    }
}

pub struct Table<'a> {
    // really doesn't need db, but r
    db: DB<'a>,
    tt: TermType,
    args: &'static str,
}

impl<'a> Table<'a> {
    pub fn new(db: DB<'a>, args: &'static str) -> Table<'a> {
        let t = Table {db: db, tt: TermType::TABLE, args: args};
        t
    }
}

impl<'a> RqlQuery for Table<'a> {
    fn build(&self) -> String {
        let db_expr = self.db.build();
        // need to go forward in chain: for loop and wrap
        // for now, just using this term!
        //[15, [[14, [\"DeppFans\"]], \"Animals\"]]
        let expressed = format!("[{0}, [{1}, \"{2}\"]]", self.tt as u32, db_expr, self.args);
        expressed
    }
}
