
#[path="../../read.rs"]
mod read;

#[path="../../parse.rs"]
mod parse;

pub fn run(p:String,t:String,d:serde_json::value::Value) -> Vec<String> {

    let limit_object = &d["limit"];
    let dir_object = &d["dir"];
    let last_object = &d["last"];

    let num;
    if parse::clean(d[t.clone()].to_string()).parse::<f64>().is_ok() {
        num = parse::clean(d[t.clone()].to_string()).parse::<f64>().unwrap();
    } else {
        num = "0.0".to_string().parse::<f64>().unwrap();
    }

    let limit;
    if limit_object.is_u64() {
        limit = parse::clean(limit_object.to_string()).parse::<u64>().unwrap();
    } else {
        limit = "5".to_string().parse::<u64>().unwrap();
    }

    let dir;
    if dir_object.is_string() {
        if dir_object.to_string() == "asc" || dir_object.to_string() == "desc" {
            dir = dir_object.to_string();
        } else {
            dir = "desc".to_string();
        }
    } else {
        dir = "desc".to_string();
    }

    let last;
    if last_object.is_null() {
        last = String::new();
    } else {
        last = parse::clean(last_object.to_string());
    }

    read::order(p.clone(),num.clone(),limit,dir,last)

}
