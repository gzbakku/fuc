
#[path="./parse.rs"]
mod parse;

#[path="./files.rs"]
mod files;

#[path="./list.rs"]
mod list;

//main function
#[allow(dead_code)]
pub fn wordify(p:String,s:String){

    base_dir(p.clone());

    let a = arrayrify(s);

    for i in a {
        let c = parse::clean(i.to_string());
        let s = score(c.clone());
        let g = group(s.clone());
        dictonerify(p.clone(),c.clone(),s.clone());
        mapify(p.clone(),s.clone(),g.clone());
    }

}

//make base dirs
#[allow(dead_code)]
fn base_dir(p:String){
    let hold = vec![
        "\\search",
        "\\search\\words\\",
        "\\search\\words\\dict\\",
        "\\search\\map\\",
    ];
    for i in hold {
        files::make_dir(p.clone() + &i.to_string());
    }
}

//split words intoa  vector of Strings
#[allow(dead_code)]
pub fn arrayrify(s:String) -> Vec<String> {
    let mut h = Vec::new();
    for i in s.split(" ") {
        h.push(i.to_string());
    }
    h
}

//find the score of the word
#[allow(dead_code)]
pub fn score(w:String) -> String {
    let mut s: i64 = 0;
    let mut f:i64 = 0;
    let mut count = 0;
    for i in w.bytes() {
        if count == 0 {
            f += i64::from(i);
        } else {
            s += i64::from(i);
        }
        count += 1;
    }
    f.to_string() + &s.to_string()
}

//find what collection to put the word in
#[allow(dead_code)]
pub fn group(s:String) -> String {

    let n = s.clone().parse::<u32>().unwrap();

    if s.len() == 3 {
        if n < 500 {
            return "500".to_string();
        }
        if n > 500 {
            return "1000".to_string();
        }
    }

    let l = last_3(s.clone());

    fn last_3(s:String) -> u32 {
        let mut pool = Vec::new();
        for i in s.chars() {
            pool.push(i);
        }
        let mut l = pool.len();
        l -= 1;
        let mut dool = Vec::new();
        let mut count = 1;
        while count < 4 {
            dool.push(pool[l]);
            count += 1;
            l -= 1;
        }
        let h = dool[2].to_string() + &dool[1].to_string() + &dool[0].to_string();
        h.parse::<u32>().unwrap()
    }

    let g = groupify(s.clone());

    fn groupify(s:String) -> u32 {
        let mut pool = Vec::new();
        for i in s.chars() {
            pool.push(i);
        }
        let mut l = pool.len();
        l -= 1;
        let mut count = 1;
        while count < 4 {
            pool.remove(l);
            count += 1;
            l -= 1;
        }
        let mut h = String::new();
        for i in pool {
            h.push(i);
        }
        h.parse::<u32>().unwrap()
    }

    let b;
    if l < 500 {
        b = g.to_string() + &"500".to_string();
    } else {
        b = (g + 1).to_string() + &"000".to_string();
    }

    return b

}

//put word in the dict with score as file name
#[allow(dead_code)]
fn dictonerify(p:String,w:String,s:String){
    let path = p +
               &"\\search\\words\\dict\\".to_string() +
               &s.to_string() +
               &".fpoi".to_string();
    if files::check_file(path.clone()) == false {
        files::make_file(path.clone());
        files::write_file(path.clone(),vec![w]);
    }
}

//put word scores into maps and groups
#[allow(dead_code)]
fn mapify(p:String,s:String,g:String) {

    //add group id to the map
    let map_path = p.clone() + &"search\\words\\map.fump".to_string();

    //println!("map_path : {:?}",map_path);

    files::make_file(map_path.clone());
    let mut read = files::read_file(map_path.clone());
    let pos = read.iter().position(|r| r == &s.clone());
    match pos {
        Some(_n) => {},
        None => {
            read.push(s.clone());
            read.sort();
            files::write_file(map_path.clone(),read);
        }
    }

    //add score to group
    let group_path = p.clone() + &"search\\words\\".to_string() + &g + &".fgup".to_string();
    files::make_file(group_path.clone());
    let mut read = files::read_file(group_path.clone());
    let pos = read.iter().position(|r| r == &s.clone());
    match pos {
        Some(_n) => {},
        None => {
            read.push(s.clone());
            read.sort();
            files::write_file(group_path.clone(),read);
        }
    }

}
