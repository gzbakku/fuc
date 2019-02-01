
#[path="../../parse.rs"]
mod parse;

#[path="../../files.rs"]
mod files;

#[path="../../list.rs"]
mod list;

#[path="../../words.rs"]
mod words;

pub fn make(p:String,t:String,d:serde_json::value::Value) {
    let doc_id = parse::md5(d.clone().to_string());
    let tag_value = d[t].clone().to_string();
    words::wordify(p.clone(),tag_value.clone());
    searchify(doc_id,p.clone(),tag_value.clone());
}

//***************************************************************
//searchify functions

fn searchify(id:String,p:String,s:String){
    let v = clean(words::arrayrify(s.clone()));
    for i in v.clone() {
        individualify(p.clone(),i.clone(),id.to_string());
    }
    treedefy_controller(p.clone(),v.clone(),id.clone());
    breakify_controller(p.clone(),v.clone(),id.clone());
}

fn breakify_controller(p:String,mut v:Vec<String>,id:String){
    while v.len() > 0 {
        treedefy_controller(p.clone(),v.clone(),id.clone());
        v.remove(0);
    }
}

fn treedefy_controller(p:String,v:Vec<String>,id:String){
    let mut path = p.clone() + &"search\\map\\".to_string();
    for i in v {
        path = treedefy(path.clone(),i,id.clone());
    }
}

fn treedefy(p:String,w:String,id:String) -> String {
    let path = p + &w.to_string() + &"\\".to_string();
    files::make_dir(path.clone());
    list::insert(path.clone(),id);
    path
}

fn individualify(p:String,w:String,id:String){
    let path = p.clone() + &"search\\map\\".to_string() + &w.to_string() + &"\\".to_string();
    files::make_dir(path.clone());
    list::insert(path,id);
}

//***************************************************************
//order func here

fn clean(a:Vec<String>) -> Vec<String> {
    let mut h = Vec::new();
    for i in a {
        h.push(parse::clean(i.to_string()));
    }
    h
}
