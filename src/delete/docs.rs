
#[path="../server.rs"]
mod server;

#[path="../auth.rs"]
mod auth;

#[path="../parse.rs"]
pub mod parse;

#[path="../files.rs"]
pub mod files;

pub fn controller(json:serde_json::value::Value) -> String {

    if
        json["user"].is_null() ||
        json["token"].is_null() ||
        json["address"].is_null() ||
        json["docs"].is_null()
    {
        return server::error("invalid-request".to_string());
    }

    //localize the vars
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

    let docs = arrayrify(docs_object);
    let collection_path = files::pathify(parse::address_locatify(address.clone()));

    for i in docs {
        documentifier(collection_path.clone(),i);
    }

    return server::success();

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
    let col_path = p.clone() + &"\\".to_string() + &d + &"//".to_string();
    if files::check_dir(col_path.clone()) == true {
        files::delete_dir(col_path);
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



//this function takes a json array of strings and convert it into a vec of strings
fn arrayrify(a:serde_json::value::Value) -> Vec<String> {
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
