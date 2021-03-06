


pub mod register;
pub mod run;

#[path="../server.rs"]
mod server;

use iron::status;
use iron::prelude::*;

pub fn run_controller(req: &mut Request) -> IronResult<Response> {
    let json_body = req.get::<bodyparser::Json>();
    let y;
    match json_body {
        Ok(Some(json_body)) => {
            y = run::controller(json_body);
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

pub fn register_controller(req: &mut Request) -> IronResult<Response> {
    let json_body = req.get::<bodyparser::Json>();
    let y;
    match json_body {
        Ok(Some(json_body)) => {
            y = register::controller(json_body);
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
