

#[path="../server.rs"]
mod server;

#[path="../auth.rs"]
mod auth;

#[path="../files.rs"]
mod files;

#[path="../parse.rs"]
pub mod parse;

#[path="../list.rs"]
pub mod list;

#[path="../common.rs"]
pub mod common;

mod index;

use serde_json::{Result, Value};

pub fn controller(json: serde_json::value::Value) -> String {

    if
        json["user"].is_null() ||
        json["token"].is_null() ||
        json["address"].is_null() ||
        json["docs"].is_null()
    {
        return server::error("invalid-request".to_string());
    }

    //localize the vars
    let user = clean(json["user"].to_string());
    let token = clean(json["token"].to_string());
    let address = clean(json["address"].to_string());
    let docs = json["docs"].to_string();

    if parse::address_vec(address.clone()).len() == 0 {
        return error("invalid-address".to_string())
    }
    if parse::address_type(address.clone()) == "doc" {
        return error("invalid-address-cannot.Save.A.Doc.In.Another.Doc".to_string())
    }

    //verify the request
    let verify_token = auth::token_verify(user,token);
    if verify_token == false {
        return error("access-denied".to_string())
    }



    //extract docs in a vec from vec strings
    let hold = arrayrify(docs);
    let array:Value;
    match hold {
        Ok(n)=>{
            array = n;
        }
        Err(error)=>{
            panic!(error)
        }
    }
    //these are docs before chcking if they exists
    let docs_raw = documentify(array);
    //these are docs after checking if they exists
    let docs = check_docs(address.clone(),docs_raw.clone());

    //check if the checked docs are in limit
    if docs.len() > 100 {
        return error("docs array exeeding the limit of 100 items.".to_string());
    }

    //read the collection's index
    if check_index(address.clone()) == true {
        index::process(address.clone(),docs.clone());
    }

    //this function saves the docs as json file in the collection
    savify(address.clone(),docs.clone());

    return success();

}

//********************************************************
//modular functions

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

fn check_docs(address:String,docs:Vec<Value>) -> Vec<Value> {
    let collection_path = files::pathify(
        parse::address_locatify(address.clone()) +
        &"\\docs".to_string()
    );
    let mut coll : Vec<Value> = Vec::new();
    for i in docs {
        let doc_id = parse::md5(i.clone().to_string());
        let doc_path = collection_path.clone() + &"\\".to_string() + &doc_id + &".json".to_string();
        if files::check_file(doc_path) == false {
            coll.push(i.clone());
        }
    }
    return coll;
}

//insert serde doc as a string to a file
fn savify(address:String,docs:Vec<Value>){

    let collection_path = files::pathify(
        parse::address_locatify(address.clone()) +
        &"\\docs".to_string()
    );

    files::make_dir(collection_path.clone());

    for i in docs {
        let file_id = parse::md5(i.clone().to_string());
        let file_path = collection_path.clone() +
                        &"\\".to_string() +
                        &file_id.clone().to_string() +
                        &".json".to_string();
        list::insert(collection_path.clone(),file_id.clone().to_string());
        files::make_file(file_path.clone());
        files::write_file(file_path.clone(),vec![i.clone().to_string()]);
    }

}

//this function takes a json array of docs and convert it into a vec of docs
fn documentify(a:Value) -> Vec<Value> {
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
        pool.push(a[count].to_owned());
        count += 1;
    }
    pool
}

//this function takes a string of docs and converts it into a json array
fn arrayrify(d:String) -> Result<Value> {
    let v: Value = serde_json::from_str(&d)?;
    Ok(v)
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
