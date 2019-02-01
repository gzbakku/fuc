
use std::path::Path;
use std::fs;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader, BufRead};

//check if file exists
#[allow(dead_code)]
pub fn check_file(p:String) -> bool {
    let x = Path::new(&p).exists();
    if x == true {
        return true
    } else {
        return false
    }
}

//read file with path
#[allow(dead_code)]
pub fn read_file(p:String) -> Vec<String> {
    let o = File::open(p.clone()).unwrap();
    let r = BufReader::new(o);
    let mut pool = Vec::new();
    for line in r.lines() {
        match line {
            Ok(data) => {
                pool.push(data.to_string());
            },
            Err(error) => {
                panic!("!!! file not found : {} || error : {}",p.clone(),error);
            }
        }
    }
    pool
}

//make file with path
#[allow(dead_code)]
pub fn make_file(p:String) -> bool {
    let x = Path::new(&p).exists();
    if x == false {
        File::create(&p).expect("Unable to create file");
        return true
    }
    return false
}

//delete file with path
#[allow(dead_code)]
pub fn delete_file(p:String) -> bool {
    if check_file(p.clone()) == true {
        let s = fs::remove_file(p.clone());
        match s {
            Ok(_n) => {
                return true
            },
            Err(_error) => {
                //panic!("delete file failed, path : {} || error : {}",p.clone(),error);
                return false
            }
        }
    } else {
        return false
    }
}

//write file with path
#[allow(dead_code)]
pub fn write_file(p:String,v:Vec<String>){
    let mut w = OpenOptions::new().write(true).open(&p).unwrap();
    for i in v {
        write!(w,"{}",i + &"\n").expect("Unable to write to file");
    }
}

//make base db dirs
#[allow(dead_code)]
pub fn db_dir(){
    let d = vec![
        "\\fuc\\users\\",
        "\\fuc\\collections\\",
        "\\fuc\\keys\\",
        "\\fuc\\tokens\\",
        "\\fuc\\index\\",
    ];
    for i in d {
        make_dir(pathify(i.to_string()));
    }
}

//make directory by folder name
#[allow(dead_code)]
pub fn make_dir(p:String){
    let x = Path::new(&p).exists();
    if x == false {
        fs::create_dir_all(&p).expect("Unable to create directories");
    }
}

//pathify the follow address with the cwd
#[allow(dead_code)]
pub fn pathify(p:String) -> String {
    cwd() + &p
}

//get current directoy path
#[allow(dead_code)]
pub fn cwd() -> String {
    let cwd = env::current_dir();
    match cwd {
        Ok(cwd) => {
            return cwd.display().to_string()
        },
        Err(error) => panic!(error)
    };
}
