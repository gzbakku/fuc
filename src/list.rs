

mod files;

//insert the doc md5
#[allow(dead_code)]
pub fn insert(p:String,s:String) -> String {
    let fult = get_fult(p);
    let mut read = files::read_file(fult.clone());
    let pos = read.iter().position(|r| r == &s);
    match pos {
        Some(_n)=>{
            return String::new();
        },
        None => {
            read.push(s);
            files::write_file(fult.clone(),read);
            return fult;
        }
    }
}

//get fult here
#[allow(dead_code)]
pub fn get_fult(p:String) -> String {
    //get fump
    make_map(p.clone());
    let fump = p.clone() + &"\\list.fump".to_string();
    let read = files::read_file(fump.clone());
    let fult_name = read[0].to_string();
    //read fult
    let fult_address = p.clone() + &"\\" + &fult_name + &".fult".to_string();
    let read = files::read_file(fult_address.clone());
    if read.len() == 100 {
        //update fump
        let read = files::read_file(fump.clone());
        let next_fult_name = (read[0].parse::<i32>().unwrap() + i32::from(100)).to_string();
        let next_fult_address = p.clone() + &"\\" + &next_fult_name + &".fult".to_string();
        files::write_file(fump.clone(),vec![next_fult_name]);
        files::make_file(next_fult_address.clone());
        return next_fult_address
    } else {
        return fult_address
    }
}

//make map before asking for fult
#[allow(dead_code)]
pub fn make_map(p:String){
    let fump = p.clone() + &"\\list.fump".to_string();
    let fult = p.clone() + &"\\100.fult".to_string();
    if files::check_file(fump.clone()) == false {
        files::make_file(fump.clone());
        files::make_file(fult);
    }
    files::write_file(fump,vec!["100".to_string()]);
}
