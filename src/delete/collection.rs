
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
        json["address"].is_null()
    {
        return server::error("invalid-request".to_string());
    }

    //localize the vars
    let user = parse::clean(json["user"].to_string());
    let token = parse::clean(json["token"].to_string());
    let address = parse::clean(json["address"].to_string());

    if parse::address_vec(address.clone()).len() == 0 {
        return server::error("invalid-address".to_string())
    }
    if parse::address_type(address.clone()) == "doc" {
        return server::error("invalid-address".to_string())
    }

    //verify the request
    let verify_token = auth::token_verify(user,token);
    if verify_token == false {
        return server::error("access-denied".to_string())
    }
    let collection_path = files::pathify(parse::address_locatify(address.clone()));

    if files::check_dir(collection_path.clone()) == true {
        files::delete_dir(collection_path);
    }

    return server::success();

}
