
#[path="../server.rs"]
mod server;

#[path="../parse.rs"]
mod parse;

#[path="../files.rs"]
mod files;

#[path="../auth.rs"]
mod auth;

use serde_json::{Result, Value};

pub fn controller(json:Value) -> String {

    if
        json["user"].is_null() ||
        json["token"].is_null() ||
        json["address"].is_null() ||
        json["docs"].is_null()
    {
        return server::error("invalid-params".to_string())
    }

    let user = parse::clean(json["user"].to_string());
    let token = parse::clean(json["token"].to_string());
    let address = parse::clean(json["address"].to_string());
    let docs_object = json["docs"].clone();

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

    let docs = documentify(docs_object);
    let collection_path = files::pathify(
        parse::address_locatify(address.clone()) +
        &"\\docs\\".to_string()
    );

    let get = fetch(collection_path,docs);

    return server::success_docs(get);

}

fn fetch(p:String,c:Vec<String>) -> Vec<Value> {

    let mut pool : Vec<Value> = Vec::new();

    for i in c {
        let path = p.clone() + &i.to_string() + &".json".to_string();
        if files::check_file(path.clone()) == true {
            let read = files::read_file(path.clone());
            if read.len() > 0 {
                let k = jsonify(read[0].to_string());
                match k {
                    Ok(n)=>{
                        pool.push(n);
                    },
                    Err(error)=>{
                        println!("error while converting json string into serde object => error : {:?}",error);
                    }
                }
            }
        }
    }

    pool

}

//this function takes a string and converts it into a json object
fn jsonify(d:String) -> Result<Value> {
    let v: Value = serde_json::from_str(&d)?;
    Ok(v)
}

//this function takes a json array of docs and convert it into a vec of docs
fn documentify(a:Value) -> Vec<String> {
    let array_hold = a.as_array();
    let array_len;
    match array_hold {
        Some(n)=>{
            array_len = n.len();
        },
        None=>{
            array_len = 0;
        }
    }
    let mut count = 0;
    let mut pool = Vec::new();
    while count < array_len {
        pool.push(parse::clean(a[count].to_string()));
        count += 1;
    }
    pool
}
