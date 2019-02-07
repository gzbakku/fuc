
#[path="../server.rs"]
mod server;

#[path="../auth.rs"]
mod auth;

#[path="../parse.rs"]
pub mod parse;

#[path="../files.rs"]
pub mod files;

#[path="../list.rs"]
pub mod list;

#[path="../common.rs"]
pub mod common;

#[path="../insert/index/mod.rs"]
mod index;

//get the doc id and doc object
//check the doc exists
//clear the doc index
//reindex the doc with new params
//remove the old file
//write the new file

use iron::status;
use iron::prelude::*;

pub fn update_controller(req: &mut Request) -> IronResult<Response> {
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

pub fn run(json:serde_json::value::Value) -> String {

    if
        json["user"].is_null() ||
        json["token"].is_null() ||
        json["address"].is_null() ||
        json["id"].is_null() ||
        json["doc"].is_null()
    {
        return server::error("invalid-request".to_string());
    }

    //localize the vars
    let user = parse::clean(json["user"].to_string());
    let token = parse::clean(json["token"].to_string());
    let address = parse::clean(json["address"].to_string());
    let doc_id = parse::clean(json["id"].to_string());
    let doc = json["doc"].clone();

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

    let collection_path = files::pathify(parse::address_locatify(address.clone()));

    documentifier(collection_path.clone(),doc_id.clone());

    //this function is imported from insert mod
    if check_index(address.clone()) == true {
        index::process(address.clone(),vec![doc.clone()]);
    }

    let new_id = savify(collection_path.clone(),doc.clone());


    return server::success_update(new_id);

}

//**********************************************************
//process doc

fn documentifier(p:String,d:String){
    refrencify(p.clone(),d.clone());
    deletify(p.clone(),d.clone());
}

fn deletify(p:String,d:String){
    let doc_path = p.clone() + &"\\docs\\".to_string() + &d + &".json".to_string();
    if files::check_file(doc_path.clone()) == true {
        files::delete_file(doc_path);
    }
    let ref_path = p.clone() + &"\\refs\\".to_string() + &d + &".fref".to_string();
    if files::check_file(ref_path.clone()) == true {
        files::delete_file(ref_path);
    }
}

//this function deletes the doc id from refence lists
fn refrencify(p:String,d:String){

    let path = p + &"\\refs\\".to_string() + &d + &".fref".to_string();
    if files::check_file(path.clone()) == false {
        return;
    }

    let refs = files::read_file(path.clone());
    for i in refs {
        if files::check_file(i.clone()) == true {
            let mut read = files::read_file(i.clone());
            let pos = read.iter().position(|r| r == &d.clone());
            match pos {
                Some(n)=>{
                    read.remove(n);
                },
                None=>{
                    println!("doc id not found in list");
                }
            }
            files::re_write_file(i.clone(),read);
        }
    }
    //loop ends here

}
//refrencifier ends here

fn check_index(address:String) -> bool {

    let collection_id = parse::collection_id(address.clone());

    let collection_path = files::pathify(
        "\\fuc\\index\\".to_string() +
        &collection_id.to_string() +
        &".fui".to_string()
    );

    if files::check_file(collection_path.clone()) == false {
        common::error("index-not_found".to_string());
        return false;
    }

    let read = files::read_file(collection_path.clone());

    if read.len() == 0 {
        common::error("index-empty".to_string());
        return false;
    } else {
        return true;
    }

}

//insert serde doc as a string to a file
fn savify(path:String,doc:serde_json::value::Value) -> String {

    let collection_path = path + &"\\docs".to_string();

    files::make_dir(collection_path.clone());

    let file_id = parse::md5(doc.clone().to_string());
    let file_path = collection_path.clone() +
                    &"\\".to_string() +
                    &file_id.clone().to_string() +
                    &".json".to_string();

    list::insert(collection_path.clone(),file_id.clone().to_string());
    files::make_file(file_path.clone());
    files::write_file(file_path.clone(),vec![doc.clone().to_string()]);
    file_id

}
