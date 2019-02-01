
#[path="../../parse.rs"]
mod parse;

#[path="../../files.rs"]
mod files;

#[path="../../list.rs"]
mod list;

pub fn make(mut p:String,t:String,d:serde_json::value::Value){
    let doc_id = parse::md5(d.to_string());
    p = p + &"order".to_string();
    files::make_dir(p.clone());
    let data_type = identify(t.clone(),d.clone());
    if data_type == "num" {
        let num = d[t.clone()].to_string().clone().parse::<f64>().unwrap();
        let list_dir = mapify(p.clone(),num.to_string().clone());
        list::insert(list_dir,num.to_string().clone());
    } else {
        list::insert(p.clone(),doc_id);
    }
}

fn mapify(p:String,i:String) -> String {
    let path = p.clone() + &"\\order.fump".to_string();
    let dir_path = p.clone() + &"\\".to_string() + &i.clone() + &"\\".to_string();
    files::make_file(path.clone());
    files::make_dir(dir_path.clone());
    let mut read = files::read_file(path.clone());
    let pos = read.iter().position(|r| r == &i.clone());
    match pos {
        Some(_n) => {},
        None => {
            read.push(i.clone());
            read.sort();
            files::write_file(path.clone(),read);
        }
    }
    dir_path
}

fn identify(key: String, json: serde_json::value::Value) -> String {
    let val = &json[key.clone()];
    if val.is_i64() || val.is_u64() || val.is_f64() {
        return "num".to_string();
    } else {
        return "none".to_string();
    }
}
