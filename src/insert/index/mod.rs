
#[path="../../parse.rs"]
mod parse;

#[path="../../files.rs"]
mod files;

mod equal;
mod order;
mod search;

pub fn process(address:String,docs:Vec<serde_json::value::Value>){
    let collection_id = parse::collection_id(address.clone());
    let index_vec = read_index(collection_id.clone(),address.clone());
    let collection_path = files::pathify(
        parse::address_locatify(address.clone()) +
        &"\\index".to_string()
    );
    files::make_dir(collection_path.clone());
    process_index(collection_path,index_vec,docs);
    //for testing
    //process_doc(collection_path,index_vec[0].clone(),docs[0].clone());
    //process_index(collection_path,vec![index_vec[1].clone()],vec![docs[0].clone()]);
}

fn process_index(p:String,i:Vec<parse::Index>,d:Vec<serde_json::value::Value>){
    for h in i {
        let a = p.clone() + &"\\".to_string() + &h.index_id.to_string();
        process_docs(a.clone(),h,d.clone());
    }
}

fn process_docs(p:String,i:parse::Index,d:Vec<serde_json::value::Value>){
    for h in d {
        process_doc(p.clone(),i.clone(),h);
    }
}

fn process_doc(mut p:String,i:parse::Index,d:serde_json::value::Value){
    if i.tags_exists == false {
        return
    }
    let tags = i.clone().tags;
    let len = tags.clone().len();
    let mut count = 1;
    for k in tags {
        if k.function == "equal" {
            if count == len && i.order_exists == false {
                p = equal::make(p.clone(),k.clone().tag.to_string(),d.clone(),true);
            } else {
                p = equal::make(p.clone(),k.clone().tag.to_string(),d.clone(),false);
            }
        } else if k.function == "weight" {
            order::make(p.clone(),k.clone().tag.to_string(),d.clone())
        } else if k.function == "search" {
            search::make(p.clone(),k.clone().tag.to_string(),d.clone());
        }
        count += 1;
    }
    if i.order_exists == true {
        order::make(p.clone(),i.clone().order.tag.to_string(),d.clone())
    }
}

fn read_index(id:String,a:String) -> Vec<parse::Index> {
    let file_address = files::pathify("\\fuc\\index\\".to_string() + &id.to_string() + &".fui".to_string());
    if files::check_file(file_address.clone()) == false {
        files::make_file(file_address.clone());
    }
    let read = files::read_file(file_address.clone());
    let mut h = Vec::new();
    for i in read {
        h.push(parse::indexify(i.to_string(),a.clone()));
    }
    h
}
