extern crate rethink;
use rethink::net;
use rethink::ast::*;
use rethink::proto::*;

fn main() {
    // rethink root struct
    let mut r = R::new();

    // set up a connection and borrower for convenience
    let mut c = net::Connection::new();
    c.connect();
    let conn = &mut c;

    // run some queries
    r.db("DeppFans").table("Animals").run(conn);
    r.db("DeppFans").table_list().run(conn);
    r.db("DeppFans").table_create("Clouds").run(conn);
    r.db("DeppFans").table_list().run(conn);


}
