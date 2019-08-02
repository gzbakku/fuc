
#[path="../server.rs"]
mod server;

#[path="../files.rs"]
mod files;

#[path="../auth.rs"]
mod auth;

#[path="../parse.rs"]
mod parse;

pub fn controller(json: serde_json::value::Value) -> String {

    files::db_dir();

    //check json keys
    if
        json["tokens"] == "null" ||
        json["user"] == "null" ||
        json["query"] == "null" ||
        json["address"] == "null"
    {
        return error("invalid_request-params".to_string())
    }

    //localize the vars
    let user = clean(json["user"].to_string());
    let token = clean(json["token"].to_string());
    let query = clean(json["query"].to_string());
    let address = clean(json["address"].to_string());

    //verify the request
    let verify_token = auth::token_verify(user,token);
    if verify_token == false {
        return error("access-denied".to_string())
    }

    //parse index and address
    let index = parse::indexify(query.clone(),address.clone());
    let address_type = parse::address_type(address.clone());
    let address_vec = parse::address_collection_vec(address.clone());

    //check index and address
    if index.valid == false  {
        return error(index.error);
    }
    if index.tags.len() == "0".parse::<usize>().unwrap() {
        return error("not_found-valid_tags".to_string())
    }
    if address_vec.len() == "0".parse::<usize>().unwrap() || &address_type == "doc" {
        return error("invalid-address".to_string())
    }

    //make index id and file address
    let index_id = parse::md5(parse::addressify(address.to_string()));
    let file_address = files::pathify("\\fuc\\index\\".to_string() + &index_id.to_string() + &".fui".to_string());

    //make file and insert the index
    if files::check_file(file_address.clone()) == false {
        files::make_file(file_address.clone());
    }
    let process = filefy(file_address.clone(),query);
    if process == false {
        return error("index-already_exists".to_string())
    }

    return success()

}

//********************************************************
//modular functions

fn filefy(address:String,query:String) -> bool {
    let mut read = files::read_file(address.clone());
    let pos = read.iter().position(|r| r == &query.clone());
    match pos {
        Some(_n)=>{
            return false
        },
        None => {
            read.push(query.to_string());
        }
    }
    files::write_file(address.clone(),read);
    true
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
