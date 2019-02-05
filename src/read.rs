
#[path="./files.rs"]
mod files;

#[path="./group.rs"]
mod group;

#[path="./common.rs"]
mod common;

const LOG:bool = false;

//read the order here
#[allow(dead_code)]
pub fn order(p:String,n:f64,l:u64,d:String,last:String) -> Vec<String> {

    //p:path n:num l:limit d:direction g:group

    common::log("-------------".to_string(),"".to_string(),LOG);

    common::log("reading order".to_string(),"".to_string(),LOG);

    let g = group::groupify(n.to_string()).parse::<u64>().unwrap();
    let mut docs : Vec<String> = Vec::new();

    //read fump
    let map_path = p.clone() + &"\\order\\group.fump".to_string();
    if files::check_file(map_path.clone()) == false {
        common::error("map not found".to_string());
        return docs;
    }

    // println!("map_path : {}",map_path);

    let read = files::read_file(map_path.clone());
    let mut groups = Vec::new();
    for i in read {
        let k = i.parse::<u64>().unwrap();
        if d == "asc" {
            if k <= g {
                groups.push(k);
            } else {
                break;
            }
        } else {
            if k >= g {
                groups.push(k);
            }
        }
    }
    let groups : Vec<u64> = groups.iter().rev().cloned().collect();

    if groups.len() == 0 {
        common::error("no groups found".to_string());
        return docs;
    }

    let lists = get_list_from_group(p.clone(),n.clone(),groups);

    if lists.len() == 0 {
        common::error("no lists found".to_string());
        return docs;
    }

    fn get_list_from_group(p:String,n:f64,g:Vec<u64>) -> Vec<String> {
        let mut hold: Vec<String> = Vec::new();
        for i in g {
            let group_path = p.clone() + &"\\order\\" + &i.to_string() + &".fgup".to_string();
            if files::check_file(group_path.clone()) == true {
                let read = files::read_file(group_path);
                for j in read {
                    let k = j.parse::<f64>().unwrap();
                    if k <= n {
                        hold.push(k.to_string());
                    } else {
                        break;
                    }
                }
            }
        }
        hold
    }

    let limit = l.to_string().parse::<usize>().unwrap();

    for i in lists {
        if docs.len() <= limit {
            let group_path = p.clone() + &"order\\" + &i.to_string();
            let hold = list(group_path,d.clone(),l.clone(),last.clone());
            for j in hold {
                if docs.len() <= limit {
                    docs.push(j.to_string());
                } else {
                    break;
                }
            }
        } else {
            break;
        }
    }

    docs

}

pub fn list(p:String,d:String,l:u64,last:String) -> Vec<String> {

    common::log("-------------".to_string(),"".to_string(),LOG);

    common::log("reading list".to_string(),"".to_string(),LOG);

    //println!("p : {} || d : {} || l : {}",p,d,l);

    let limit = l.to_string().parse::<usize>().unwrap();
    let mut docs : Vec<String> = Vec::new();

    //read map
    let map_path = p.clone() + &"\\list.fump".to_string();
    if files::check_file(map_path.clone()) == false {
        common::error("map not found".to_string());
        return docs;
    }

    common::log("mapified".to_string(),"".to_string(),LOG);

    //make list vec
    let read = files::read_file(map_path.clone());
    let mut lists : Vec<String> = Vec::new();
    if read.len() > 0 {
        let mut num = read[0].parse::<u64>().unwrap();
        let mut anchor = 100;
        while num >= 100 {
            lists.push(anchor.to_string());
            num -= 100;
            anchor += 100;
        }
    } else {
        common::error("no list found".to_string());
        return docs;
    }

    common::log("listified".to_string(),"".to_string(),LOG);

    if lists.len() == 0 {
        common::error("no lists found".to_string());
        return docs;
    }

    if d == "desc".to_string() {
        lists = lists.iter().rev().cloned().collect();
    }

    common::log("directionerified".to_string(),"".to_string(),LOG);

    //println!("lists : {:?}",lists);

    let mut control = false;
    if last.len() == 0 {
        control = true;
    }

    for i in lists {
        if docs.len() < limit {
            let list_path = p.clone() + &"\\".to_string() + &i.to_string() + &".fult".to_string();
            if files::check_file(list_path.clone()) == true {
                let read = files::read_file(list_path);
                for j in read {
                    if docs.len() < limit {
                        if control == true {
                            docs.push(j.to_string());
                        }
                    } else {
                        break;
                    }
                    if j.to_string() == last {
                        control = true;
                    }
                }
            }
        } else {
            break;
        }
    }

    common::log("docified".to_string(),"".to_string(),LOG);

    docs

}
