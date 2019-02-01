
#[path="../../parse.rs"]
mod parse;

#[path="../../files.rs"]
mod files;

#[path="../../list.rs"]
mod list;

//make index if last make a static list here
pub fn make(mut p:String,t:String,d:serde_json::value::Value,l:bool) -> String {
    p.push_str(&("\\".to_string() + &parse::clean(d[t.clone()].to_string()) + &"\\".to_string()));
    files::make_dir(p.clone());
    if l == true {
        let doc_name = parse::md5(d.to_string());
        list::insert(p.clone(),doc_name);
    }
    p
}
