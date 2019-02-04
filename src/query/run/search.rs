
#[path="../../parse.rs"]
mod parse;

#[path="../../words.rs"]
mod words;

#[path="../../files.rs"]
mod files;

#[path="../../common.rs"]
mod common;

#[path="../../read.rs"]
mod read;

//const LOG:bool = true;

pub fn run(p:String,t:String,d:serde_json::value::Value) -> Vec<String> {

    let mut docs : Vec<String> = Vec::new();
    let val = parse::clean(d.clone()[t.clone()].to_string());

    let limit_object = parse::clean(d.clone()["limit"].to_string());
    let dir_object = parse::clean(d.clone()["dir"].to_string());

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

    let path = p.clone() +
               &"\\search\\".to_string() +
               &t.clone() +
               &"\\".to_string();

    let mut words = check_words(path.clone(),val.clone());
    if words.len() == 0 {
        words = search_words(path.clone(),val.clone());
    }
    if words.len() == 0 {
        common::error("no words found".to_string());
        return docs;
    }

    // println!("words : {:?}",words.clone());

    if check_tree(path.clone(),words.clone()) == false {
        docs = read_tree(path.clone(),vec![words.clone()[0].to_string()],dir,limit);
    } else {
        docs = read_tree(path.clone(),words.clone(),dir,limit);
    }

    docs

}

//*******************************************************
//words

fn check_words(mut p:String,s:String) -> Vec<String> {

    p = p.clone() + &"\\words\\dict\\".to_string();

    let mut pool : Vec<String> = Vec::new();

    let mut h = Vec::new();
    for i in s.split(" ") {
        h.push(i.to_string());
    }

    for i in h {
        let word = i;
        let score = words::score(word.clone());
        let search_path = p.clone() + &score + &".fpoi".to_string();
        if files::check_file(search_path) == true {
            pool.push(word);
        }
    }

    if pool.len() > 0 {
        pool.sort();
    }

    pool

}

fn search_words(mut p:String,s:String) -> Vec<String> {

    p = p.clone() + &"words\\".to_string();
    let mut pool : Vec<String> = Vec::new();
    let words = wordify(s);
    for i in words {
        let word = search_word(p.clone(),i.to_string());
        if word.len() > 0 {
            pool.push(word);
        }
    }
    pool.sort();
    pool

}

fn search_word(p:String,w:String) -> String {

    let mut pool = String::new();

    let score = words::score(w.clone());
    let group = words::group(score.clone());
    let group_path = p.clone() + &group.clone() + &".fgup".to_string();
    if files::check_file(group_path.clone()) == false {
        common::error("group not found".to_string());
        return pool;
    }

    let score = score.parse::<u64>().unwrap();
    let read = files::read_file(group_path.clone());
    if read.len() == 0 {
        common::error("no scores found in group".to_string());
        return pool;
    }
    let mut prev_diff = "99999999999999".to_string().parse::<u64>().unwrap();
    let mut hold = String::new();

    for i in read {
        let k = i.to_string().parse::<u64>().unwrap();
        let diff;
        if k < score {
            diff = score.clone() - k.clone();
        } else {
            diff = k.clone() - score.clone();
        }
        if diff < prev_diff {
            prev_diff = diff;
            hold = k.clone().to_string();
        } else {
            break;
        }
    }

    //println!("diff : {:?} || closest score : {:?}",prev_diff.clone(),hold.clone());

    let word_path = p.clone() + &"dict\\".to_string() + &hold.clone() + &".fpoi".to_string();
    if files::check_file(group_path.clone()) == false {
        common::error("word_path not found".to_string());
        return pool;
    }
    let read = files::read_file(word_path.clone());
    if read.len() == 0 {
        common::error("nothing to read".to_string());
        return pool;
    }

    pool = read[0].to_string();
    pool

}

//*******************************************************
//tree

fn check_tree(p:String,w:Vec<String>) -> bool {
    let address = build_tree_address(p,w);
    files::check_dir(address)
}

fn read_tree(p:String,w:Vec<String>,d:String,l:u64) -> Vec<String> {

    let pool;
    let address = build_tree_address(p,w);
    pool = read::list(address,d,l);
    pool

}

fn build_tree_address(p:String,w:Vec<String>) -> String {
    let mut path = p.clone() + &"map\\".to_string();
    for i in w {
        path = path.clone() + &i.to_string() + &"\\".to_string();
    }
    path
}

fn wordify(s:String) -> Vec<String> {
    let mut h : Vec<String> = Vec::new();
    for i in s.split(" ") {
        h.push(i.to_string());
    }
    h
}
