#[path="../server.rs"]
mod server;

#[path="../files.rs"]
mod files;

#[path="../auth.rs"]
mod auth;

#[path="../parse.rs"]
mod parse;

use serde_json::{Result, Value};

mod equal;
mod order;
mod search;

//********************************************************
//main

pub fn controller(json: serde_json::value::Value) -> String {

    //localize the vars
    let user = clean(json["user"].to_string());
    let token = clean(json["token"].to_string());
    let address = clean(json["address"].to_string());
    let query = clean(json["query"].to_string());
    let limit = clean(json["limit"].to_string());
    let params = arrayrify(clean(json["params"].to_string()));

    //verify the request
    let verify_token = auth::token_verify(user,token);
    if verify_token == false {
        return error("access-denied".to_string())
    }
    if verify_query(query.clone().to_string(),address.clone().to_string()) == false {
        return error("invalid-query".to_string())
    }

    //get the index
    let index = parse::indexify(query.clone().to_string());
    let collection_path = files::pathify(
        parse::address_locatify(address.clone()) +
        &"\\index".to_string()
    );
    let mut path = collection_path.clone();

    //loop through tags
    //break the loop if the tag fails
    for i in index.clone().tags {
        if i.function == "equal" {
            path = equal::run(collection_path.clone(),i.tag,params.clone());
        } else if i.function == "weight" {
            path = order::run(collection_path.clone(),i.tag,params.clone());
        } else if i.function == "search" {
            path = search::run(collection_path.clone(),i.tag,params.clone());
        }
    }

    if index.clone().order_exists == true {
        path = order::run(collection_path.clone(),i.order.tag,params.clone());
    }

    return success();

}

//********************************************************
//modular func's here

//this function takes a string of docs and converts it into a json array
fn arrayrify(d:String) -> Result<Value> {
    let v: Value = serde_json::from_str(&d)?;
    Ok(v)
}

//check if the query exists in the collection index fui
fn verify_query(q:String,a:String) -> bool {
    let collection_id = parse::collection_id(a.clone());
    let path = files::pathify("\\fuc\\index\\".to_string() + &collection_id.to_string() + &".fui".to_string());
    if files::check_file(path.clone()) == false {
        return false
    }
    let read = files::read_file(path.clone());
    let pos = read.iter().position(|r| r == &q);
    match pos {
        Some(_n)=>{
            return true
        },
        None => {
            return false
        }
    }
}

//********************************************************
//common

fn success() -> String {
    stringify(server::Result {
        success:true,
        error:String::new(),
        docs:String::new(),
        message:String::new(),
    })
}

fn error(err:String) -> String {
    stringify(server::Result {
        success:false,
        error:String::from(err),
        docs:String::new(),
        message:String::new(),
    })
}

fn stringify(hold: server::Result) -> String {
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

fn clean(s:String) -> String {
    let mut hold = String::new();
    for i in s.chars() {
        for j in i.to_string().bytes(){
            if j != 34 {
                hold.push_str(&i.to_string());
            }
        }
    }
    hold
}
