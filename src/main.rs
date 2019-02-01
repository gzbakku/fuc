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
mod connect;
mod user;

//document and query mods
mod insert;
mod query;

//this is used to send erros
mod server;

//******************************************************
//main

fn main() {

    println!("listing on port 3000");

    let mut router = Router::new();

    router.get("/", index_controller, "index");
    router.post("/insert", insert_controller, "insert");

    router.post("/user/register", user_register_controller, "register");
    router.post("/user/reset", user_reset_controller, "reset");
    router.post("/user/delete", user_delete_controller, "delete");

    router.post("/connect", connect_controller, "connect");

    router.post("/query/register", query_register_controller, "query_register");
    router.post("/query", query_run_controller, "query_do");

    Iron::new(router).http("localhost:3000").unwrap();

}

//******************************************************
//request controllers

pub fn query_run_controller(req: &mut Request) -> IronResult<Response> {
    let json_body = req.get::<bodyparser::Json>();
    let y;
    match json_body {
        Ok(Some(json_body)) => {
            y = query::run::controller(json_body);
        },
        Ok(None) => {
            y = errorify("invalid-request".to_string());
        },
        Err(_err) => {
            y = errorify("unknown-error".to_string());
        }
    }
    Ok(Response::with((status::Ok, y)))
}

pub fn query_register_controller(req: &mut Request) -> IronResult<Response> {
    let json_body = req.get::<bodyparser::Json>();
    let y;
    match json_body {
        Ok(Some(json_body)) => {
            y = query::register::controller(json_body);
        },
        Ok(None) => {
            y = errorify("invalid-request".to_string());
        },
        Err(_err) => {
            y = errorify("unknown-error".to_string());
        }
    }
    Ok(Response::with((status::Ok, y)))
}

pub fn connect_controller(req: &mut Request) -> IronResult<Response> {
    let json_body = req.get::<bodyparser::Json>();
    let y;
    match json_body {
        Ok(Some(json_body)) => {
            y = connect::controller(json_body);
        },
        Ok(None) => {
            y = errorify("invalid-request".to_string());
        },
        Err(_err) => {
            y = errorify("unknown-error".to_string());
        }
    }
    Ok(Response::with((status::Ok, y)))
}

pub fn user_delete_controller(req: &mut Request) -> IronResult<Response> {
    let json_body = req.get::<bodyparser::Json>();
    let y;
    match json_body {
        Ok(Some(json_body)) => {
            y = user::delete::controller(json_body);
        },
        Ok(None) => {
            y = errorify("invalid-request".to_string());
        },
        Err(_err) => {
            y = errorify("unknown-error".to_string());
        }
    }
    Ok(Response::with((status::Ok, y)))
}

pub fn user_reset_controller(req: &mut Request) -> IronResult<Response> {
    let json_body = req.get::<bodyparser::Json>();
    let y;
    match json_body {
        Ok(Some(json_body)) => {
            y = user::reset::controller(json_body);
        },
        Ok(None) => {
            y = errorify("invalid-request".to_string());
        },
        Err(_err) => {
            y = errorify("unknown-error".to_string());
        }
    }
    Ok(Response::with((status::Ok, y)))
}

pub fn user_register_controller(req: &mut Request) -> IronResult<Response> {
    let json_body = req.get::<bodyparser::Json>();
    let y;
    match json_body {
        Ok(Some(json_body)) => {
            y = user::register::controller(json_body);
        },
        Ok(None) => {
            y = errorify("invalid-request".to_string());
        },
        Err(_err) => {
            y = errorify("unknown-error".to_string());
        }
    }
    Ok(Response::with((status::Ok, y)))
}

fn index_controller(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "what are you doing here".to_string())))
}

pub fn insert_controller(req: &mut Request) -> IronResult<Response> {
    let json_body = req.get::<bodyparser::Json>();
    let y;
    match json_body {
        Ok(Some(json_body)) => {
            y = insert::controller(json_body);
        },
        Ok(None) => {
            y = errorify("invalid-request".to_string());
        },
        Err(_err) => {
            y = errorify("unknown-error".to_string());
        }
    }
    Ok(Response::with((status::Ok, y)))
}

//******************************************************
//common

fn errorify(error:String) -> String {
    stringify(server::Result {
        success:false,
        error:String::from(error),
        docs:String::new(),
        message:String::new(),
    })
}

fn stringify(hold:server::Result) -> String {
    let dulo = serde_json::to_string(&hold);
    match dulo {
        Ok(n) => {
            return n
        },
        Err(err) => {
            println!("{:?}",err);
            return "error".to_string()
        }
    };
}
