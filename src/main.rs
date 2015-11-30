extern crate rethink;
use rethink::net;
use rethink::ast::*;
use rethink::proto::*;

fn main() {
    // let stream = net::connect();
    // match stream {
    //     Some(mut s) => {
    //         let mut r = R::new(&mut s);
    //         r.db("DeppFans").table("Animals").run();
    //
    //     }
    //     //net::read_query_test(&mut s),
    //     //Some(mut s) => net::write_query_test(&mut s),
    //     None => println!("Never established handshake... exiting...")
    // }

    let mut r = R::new();
    //r.connect();
    // worry about r.use type deal later..
    let mut conn = net::Connection::new();
    conn.connect();
    //r.db("DeppFans").table("Animals").run(conn);
    r.db("DeppFans").table_list().run(&mut conn);
    //r.db("DeppFans").table_create("Clouds").run(&mut conn);
    r.db("DeppFans").table_list().run(&mut conn);


}
