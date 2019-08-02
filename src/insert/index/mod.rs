
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
    let ref_path = files::pathify(
        parse::address_locatify(address.clone()) +
        &"\\refs\\".to_string()
    );

    files::make_dir(collection_path.clone());

    process_index(collection_path,index_vec,docs,ref_path);

}

fn process_index(p:String,i:Vec<parse::Index>,d:Vec<serde_json::value::Value>,ref_path:String){
    for h in i {
        let a = p.clone() + &"\\".to_string() + &h.index_id.to_string();
        process_docs(a.clone(),h,d.clone(),ref_path.clone());
    }
}

fn process_docs(p:String,i:parse::Index,d:Vec<serde_json::value::Value>,ref_path:String){
    for h in d {
        process_doc(p.clone(),i.clone(),h,ref_path.clone());
    }
}

fn process_doc(p:String,i:parse::Index,d:serde_json::value::Value,ref_path:String){

    if i.tags_exists == false {
        return
    }

    let mut refs = Vec::new();
    let mut index_path = p.clone();
    let doc_id = parse::md5(d.clone().to_string());

    for k in i.clone().tags {
        if k.function == "equal" {
            index_path = equal::pathify(index_path.clone(),k.clone().tag.to_string(),d.clone());
        } else if k.function == "weight" {
            refs = order::make(index_path.clone(),k.clone().tag.to_string(),d.clone())
        } else if k.function == "search" {
            refs = search::make(index_path.clone(),k.clone().tag.to_string(),d.clone());
        }
    }

    if i.index_type == "equal" {
        refs = equal::make(index_path.clone(),doc_id.clone());
    }
    if i.order_exists == true {
        refs = order::make(index_path.clone(),i.clone().order.tag.to_string(),d.clone())
    }

    make_ref(ref_path,doc_id,refs);

}

fn make_ref(p:String,id:String,refs:Vec<String>){

    files::make_dir(p.clone());

    let doc_path = p.clone() + &id + &".fref".to_string();

    if files::check_file(doc_path.clone()) == false {
        files::make_file(doc_path.clone());
    }

    let mut read = files::read_file(doc_path.clone());

    for i in refs {
        let pos = read.iter().position(|r| r == &i);
        match pos {
            Some(_n)=>{},
            None=>{
                read.push(i.to_string());
            }
        }
    }

    files::write_file(doc_path,read);

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
