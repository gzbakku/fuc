
#[path="../../parse.rs"]
mod parse;

#[path="../../files.rs"]
mod files;

#[path="../../list.rs"]
mod list;

#[path="../../words.rs"]
mod words;

pub fn make(mut p:String,t:String,d:serde_json::value::Value) -> Vec<String> {
    p = p + &"\\search\\".to_string() + &t.clone() + &"\\".to_string();
    files::make_dir(p.clone());
    let doc_id = parse::md5(d.clone().to_string());
    let tag_value = d[t].clone().to_string();
    words::wordify(p.clone(),tag_value.clone());
    searchify(doc_id,p.clone(),tag_value.clone())
}

//***************************************************************
//searchify functions

fn searchify(id:String,p:String,s:String) -> Vec<String> {

    let v = clean(words::arrayrify(s.clone()));

    let mut collect = Vec::new();

    let refs_0 = individualify_controller(p.clone(),v.clone(),id.clone());
    for i in refs_0 {
        collect.push(i.to_string());
    }

    let refs_1 = treedefy_controller(p.clone(),v.clone(),id.clone());
    for i in refs_1 {
        collect.push(i.to_string());
    }

    let refs_2 = breakify_controller(p.clone(),v.clone(),id.clone());
    for i in refs_2 {
        collect.push(i.to_string());
    }

    collect

}

fn individualify_controller(p:String,v:Vec<String>,id:String) -> Vec<String> {
    let mut refs = Vec::new();
    for i in v {
        let hold = individualify(p.clone(),i,id.to_string());
        if hold.len() > 0 {
            refs.push(hold);
        }
    }
    refs
}

fn individualify(p:String,w:String,id:String) -> String {
    let path = p.clone() + &"map\\".to_string() + &w.to_string() + &"\\".to_string();
    files::make_dir(path.clone());
    list::insert(path,id)
}

fn breakify_controller(p:String,mut v:Vec<String>,id:String) -> Vec<String> {
    let mut refs = Vec::new();
    while v.len() > 0 {
        let hold = treedefy_controller(p.clone(),v.clone(),id.clone());
        if hold.len() > 0 {
            for i in hold {
                refs.push(i.to_string());
            }
        }
        v.remove(0);
    }
    refs
}

fn treedefy_controller(p:String,v:Vec<String>,id:String) -> Vec<String> {
    let mut path = p.clone() + &"map\\".to_string();
    let mut refs = Vec::new();
    for i in v {
        path = path + &i.to_string() + &"\\".to_string();
        files::make_dir(path.clone());
        let hold = list::insert(path.clone(),id.clone());
        if hold.len() > 0 {
            refs.push(hold);
        }
    }
    refs
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
