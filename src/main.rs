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
use std::env;
use std::net::TcpListener;

//******************************************************
//mods

//user and connection mods
mod init;
mod user;

//document and query mods
mod insert;
mod query;
mod get;
mod delete;
mod update;

//this is used to send erros
mod server;

//******************************************************
//main

fn main() {

    let args: Vec<String> = env::args().collect();

    let port;

    if args.len() >= 2 {
        let port_object = &args[1];
        if port_object.parse::<u16>().is_ok() {
            port = port_object.parse::<u16>().unwrap();
        } else {
            port = "3000".to_string().parse::<u16>().unwrap();
        }
    } else {
        port = "3000".to_string().parse::<u16>().unwrap();
    }

    if check_port(port.clone()) == true {
        serve(port.to_string());
    } else {
        println!("!!! port in use");
    }

}

fn check_port(port:u16) -> bool {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => {return true;},
        Err(_) => {return false;},
    }
}

fn serve(port:String){

    println!("listing on port : {}",port.clone());

    let url = "127.0.0.1:".to_string() + &port.to_string();

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

    router.post("/delete/docs", delete::docs_controller, "delete_docs");
    router.post("/delete/collection", delete::collection_controller, "delete_collection");

    router.post("/update", update::update_controller, "update_doc");

    Iron::new(router).http(url).unwrap();

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
