
#[path="../../parse.rs"]
mod parse;

#[path="../../files.rs"]
mod files;

#[path="../../list.rs"]
mod list;

#[path="../../group.rs"]
mod group;

#[path="../../common.rs"]
mod common;

const LOG:bool = false;

pub fn make(mut p:String,t:String,d:serde_json::value::Value){

    //println!("inserting into order");

    let doc_id = parse::md5(d.to_string());
    p = p + &"order".to_string();
    files::make_dir(p.clone());

    let data_type = identify(t.clone(),d.clone());
    let num = d[t.clone()].to_string();

    if data_type == "num" {
        mapify(p.clone(),num.to_string().clone(),doc_id.clone());
    } else {
        mapify(p.clone(),"0".to_string().clone(),doc_id.clone());
    }

}

fn mapify(p:String,i:String,id:String) -> String {

    //p:path i:num id:doc_id

    common::log("----------------".to_string(),"".to_string(),LOG);

    common::log("mapifying : ".to_string(),i.clone().to_string(),LOG);

    //paths and group
    let map_path = p.clone() + &"\\group.fump".to_string();
    let g = group::groupify(i.clone());
    let group_path = p.clone() + &"\\".to_string() + &g.clone() + &".fgup".to_string();

    common::log("grouped".to_string(),"".to_string(),LOG);

    //insert group into map
    files::make_file(map_path.clone());
    let mut read = files::read_file(map_path.clone());
    let pos = read.iter().position(|r| r == &g.clone());
    match pos {
        Some(_n) => {},
        None => {
            read.push(g.clone());
            read.sort();
            files::write_file(map_path.clone(),read);
        }
    }

    common::log("maped".to_string(),"".to_string(),LOG);

    //insert num into group
    files::make_file(group_path.clone());
    let mut read = files::read_file(group_path.clone());
    let pos = read.iter().position(|r| r == &i.clone());
    match pos {
        Some(_n) => {},
        None => {
            read.push(i.clone());
            read.sort();
            files::write_file(group_path.clone(),read);
        }
    }

    common::log("inserted".to_string(),"".to_string(),LOG);

    //make num list here
    let num_path = p.clone() + &"\\".to_string() + &i.clone() + &"\\".to_string();
    files::make_dir(num_path.clone());
    list::insert(num_path.clone(),id.clone());

    common::log("listed".to_string(),"".to_string(),LOG);

    common::log("----------------".to_string(),"".to_string(),LOG);

    num_path

}



fn identify(key: String, json: serde_json::value::Value) -> String {
    let val = &json[key.clone()];
    if val.is_i64() || val.is_u64() || val.is_f64() {
        return "num".to_string();
    } else {
        return "none".to_string();
    }
}
