extern crate iron;
extern crate router;
extern crate bodyparser;
extern crate persistent;
extern crate crypto;
extern crate rand;

extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;

//i have no idea what to do with this
// maybe it was used in iron to impliment max body size for incoming data
//use persistent::Read;

use iron::status;
use iron::prelude::*;
use router::Router;

//******************************************************
//mods

//user and connection mods
mod init;
mod user;

//document and query mods
mod insert;
mod query;
mod get;

//this is used to send erros
mod server;

//******************************************************
//main

fn main() {

    println!("listing on port 3000");

    let mut router = Router::new();

    router.get("/", index_controller, "index");
    router.post("/init", init::controller, "init");

    router.post("/user/register", user::register_controller, "user_register");
    router.post("/user/reset", user::reset_controller, "user_reset");
    router.post("/user/delete", user::delete_controller, "user_delete");
    router.post("/user/connect", user::connect_controller, "user_connect");

    router.post("/insert", insert_controller, "insert");

    router.post("/query/register", query::register_controller, "query_register");
    router.post("/query", query::run_controller, "query_do");

    router.post("/get/docs", get::docs_controller, "get_docs");
    router.post("/get/collection", get::collection_controller, "get_collection");

    Iron::new(router).http("localhost:3000").unwrap();

}

//******************************************************
//request controllers

fn index_controller(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "what are you doing here".to_string())))
}

fn insert_controller(req: &mut Request) -> IronResult<Response> {
    let json_body = req.get::<bodyparser::Json>();
    let y;
    match json_body {
        Ok(Some(json_body)) => {
            y = insert::controller(json_body);
        },
        Ok(None) => {
            y = server::error("invalid-request".to_string());
        },
        Err(_err) => {
            y = server::error("unknown-error".to_string());
        }
    }
    Ok(Response::with((status::Ok, y)))
}
