
#[path="../../parse.rs"]
mod parse;

#[path="../../files.rs"]
mod files;

#[path="../../list.rs"]
mod list;

#[path="../../common.rs"]
mod common;

pub fn pathify(mut p:String,t:String,d:serde_json::value::Value) -> String {
    p.push_str(&("\\".to_string() + &t.clone() + &"\\".to_string() + &parse::clean(d[t.clone()].to_string()) + &"\\".to_string()));
    files::make_dir(p.clone());
    p
}

//make index if last make a static list here
pub fn make(p:String,d:String) -> Vec<String> {
    let l = list::insert(p,d);
    if l.len() > 0 {
        return vec![l.clone()];
    } else {
        common::error("list didnt returned a string".to_string());
        return Vec::new();
    }
}
