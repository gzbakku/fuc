
#[path="../../parse.rs"]
mod parse;

#[path="../../files.rs"]
mod files;

#[path="../../read.rs"]
mod read;

pub fn run(p:String,t:String,d:serde_json::value::Value) -> String {

    let path = p +
               &"\\".to_string() +
               &t.clone() + &"\\".to_string() +
               &parse::clean(d[t.clone()].to_string()) +
               &"\\".to_string();

    return path;

}

pub fn check(p:String,t:String,d:serde_json::value::Value) -> bool {

    let path = p +
               &"\\".to_string() +
               &t.clone() + &"\\".to_string() +
               &parse::clean(d[t.clone()].to_string()) +
               &"\\".to_string();

    files::check_dir(path.clone())

}

pub fn fetch(p:String,d:serde_json::value::Value) -> Vec<String> {

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
