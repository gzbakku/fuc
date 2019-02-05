
#[path="./server.rs"]
mod server;

#[path="./files.rs"]
pub mod files;

#[path="./auth.rs"]
pub mod auth;

#[path="./parse.rs"]
pub mod parse;

use iron::status;
use iron::prelude::*;

pub fn controller(req: &mut Request) -> IronResult<Response> {
    let json_body = req.get::<bodyparser::Json>();
    let y;
    match json_body {
        Ok(Some(json_body)) => {
            y = run(json_body);
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

pub fn run(json: serde_json::value::Value) -> String {

    //make base db dirs
    files::db_dir();

    let base = parse::clean(json["base"].to_string());
    let hash = auth::hash256(base);

    //read user file
    let path = files::pathify("\\fuc\\keys\\register.fkey".to_string());

    if files::check_file(path.clone()) == true {
        return server::error("db-already_initiated".to_string());
    } else {
        make_key(path.clone(),hash.clone());
        return success(hash.clone());
    }

}

//********************************************************
//modular functions

fn make_key(p:String,key:String) {
    files::make_file(p.clone());
    files::write_file(p.clone(),vec![auth::hash256(key.clone())]);
}

//********************************************************
//common

fn success(key:String) -> String {

    let hold = server::Key {
        key:key,
    };

    stringify_token(server::ResultKey {
        success:true,
        error:String::new(),
        docs:hold,
        message:String::new(),
    })

}

fn stringify_token(hold: server::ResultKey) -> String {
    let work = serde_json::to_string(&hold);
    match work {
        Ok(n) => {
            return n
        },
        Err(err) => {
            println!("{:?}",err);
            return "error".to_string()
        }
    };
}
