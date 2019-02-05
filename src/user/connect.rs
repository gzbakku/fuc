
#[path="../server.rs"]
mod server;

#[path="../files.rs"]
pub mod files;

#[path="../auth.rs"]
pub mod auth;

#[path="../parse.rs"]
pub mod parse;

use crypto::sha2::Sha256;
use crypto::digest::Digest;

pub fn controller(json: serde_json::value::Value) -> String {

    //make base db dirs
    files::db_dir();

    //chekc if auth feilds exists in json
    if
        json["user"].is_null() ||
        json["password"].is_null()
    {
        return server::error("invalid_request-params".to_string());
    }

    //extract feilds from json as strings
    let user = parse::clean(json["user"].to_string());
    let pass = parse::clean(json["password"].to_string());

    //check if user exists
    if check_user(user.clone()) == false {
        return error("invalid-user".to_string())
    }

    //read user file
    let path = files::pathify("\\fuc\\users\\".to_string() + &user + &".fuser".to_string());
    let read = files::read_file(path);
    let mut pass_hasher = Sha256::new();
    pass_hasher.input_str(&pass.to_string());
    let hashed_pass = pass_hasher.result_str();

    //check user password
    if hashed_pass != read[0] {
        return error("access-denied".to_string())
    }

    let hold = process_user(user);

    return success(hold.user,hold.token)

}

//********************************************************
//modular functions

fn check_user(user:String) -> bool {
    let path = files::pathify("\\fuc\\users\\".to_string() + &user + &".fuser".to_string());
    if files::check_file(path.clone()) == false {
        false
    } else {
        true
    }
}

fn process_user(user:String) -> server::Token {

    let token = auth::token_issue(user.clone());
    let path = files::pathify("\\fuc\\tokens\\ ".to_string() + &user.clone() + &".ftok".to_string());
    if files::check_file(path.clone()) == true {
        files::delete_file(path.clone());
    }
    files::make_file(path.clone());
    let hold = vec![token.user,token.time,token.disguise];
    files::write_file(path.clone(),hold);

    server::Token {
        user:user.clone(),
        token:token.token
    }

}

//********************************************************
//common

fn success(user:String,token:String) -> String {

    let hold = server::Token {
        user:user,
        token:token
    };

    stringify_token(server::ResultToken {
        success:true,
        error:String::new(),
        docs:hold,
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

fn stringify_token(hold: server::ResultToken) -> String {
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
