
#[path="../server.rs"]
mod server;

#[path="../files.rs"]
pub mod files;

#[path="../auth.rs"]
mod auth;

pub fn controller(json: serde_json::value::Value) -> String {

    files::db_dir();

    if
        json["key"] == "null" ||
        json["user"] == "null" ||
        json["password"] == "null"
    {
        return error("invalid_request-params".to_string())
    }

    let key = read_key();
    let hashed_key = auth::hash256(clean(json["key"].to_string()));

    if hashed_key != key {
        return error("access-denied".to_string())
    }

    if check_user(clean(json["user"].to_string())) == false {
        return error("invalid-user".to_string())
    }

    process_user(clean(json["user"].to_string()),clean(json["password"].to_string()));

    return success()

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

fn process_user(user:String, pass:String) {

    let path = files::pathify("\\fuc\\users\\".to_string() + &user + &".fuser".to_string());
    if check_user(user.clone()) == false {
        files::make_file(path.clone());
    }
    let hold = vec![auth::hash256(pass.clone())];
    files::write_file(path,hold);

}

pub fn read_key() -> String {
    let path = files::pathify("\\fuc\\keys\\register.fkey".to_string());
    let file = files::read_file(path);
    let hold = &file[0];
    hold.to_string()
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
