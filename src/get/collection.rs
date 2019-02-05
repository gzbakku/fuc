
#[path="../server.rs"]
mod server;

#[path="../parse.rs"]
mod parse;

#[path="../files.rs"]
mod files;

#[path="../auth.rs"]
mod auth;

#[path="../read.rs"]
mod read;

use serde_json::Value;

pub fn controller(json:Value) -> String {

    if
        json["user"].is_null() ||
        json["token"].is_null() ||
        json["address"].is_null() ||
        json["params"].is_null()
    {
        return server::error("invalid-params".to_string())
    }

    let user = parse::clean(json["user"].to_string());
    let token = parse::clean(json["token"].to_string());
    let address = parse::clean(json["address"].to_string());
    let params = json["params"].clone();

    if parse::address_vec(address.clone()).len() == 0 {
        return server::error("invalid-address".to_string())
    }
    if parse::address_type(address.clone()) == "doc" {
        return server::error("invalid-address-cannot.Save.A.Doc.In.Another.Doc".to_string())
    }

    //verify the request
    let verify_token = auth::token_verify(user,token);
    if verify_token == false {
        return server::error("access-denied".to_string())
    }

    let collection_path = files::pathify(
        parse::address_locatify(address.clone()) +
        &"\\docs".to_string()
    );

    let docs = fetch(collection_path,params);

    //println!("docs : {:?}",docs);

    return server::success_query(docs);

}

fn fetch(p:String,d:Value) -> Vec<String> {

    let limit_object = parse::clean(d.clone()["limit"].to_string());
    let dir_object = parse::clean(d.clone()["dir"].to_string());
    let last_object = parse::clean(d.clone()["last"].to_string());

    let limit;
    if limit_object.parse::<u64>().is_ok() {
        limit = limit_object.parse::<u64>().unwrap();
    } else {
        limit = "10".parse::<u64>().unwrap();
    }

    let dir;
    if dir_object == "asc" || dir_object == "desc" {
        dir = dir_object.to_string();
    } else {
        dir = "desc".to_string();
    }

    let last;
    if d.clone()["last"].is_null() {
        last = String::new();
    } else {
        last = last_object;
    }

    read::list(p,dir,limit,last)

}
