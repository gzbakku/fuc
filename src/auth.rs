
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
pub fn get_time() -> u64 {
    let start = SystemTime::now();
    let time_base = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    time_base.as_secs()
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Token {
    pub user:String,
    pub time:String,
    pub token:String,
    pub disguise:String
}

#[allow(dead_code)]
pub fn hash256(s:String) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(&s);
    hasher.result_str()
}

#[allow(dead_code)]
pub fn token_issue(user:String) -> Token {

    //get random number
    let mut rng = rand::thread_rng();
    let num = rng.gen::<i32>();

    //get time in secs
    let start = SystemTime::now();
    let time_base = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let time = time_base.as_secs().to_string();

    //mix user num and time here
    let mix = user.clone() + &"_".to_string() + &num.to_string() + &"_".to_string() + &time;

    //hash em
    let token = hash256(mix);
    let disguise = hash256(token.clone());

    //make the struct
    Token {
        user:user,
        time:time,
        disguise:disguise,
        token:token
    }

}

#[allow(dead_code)]
mod files;

#[allow(dead_code)]
pub fn token_verify(user:String,token:String) -> bool {
    let path = files::pathify("\\fuc\\tokens\\ ".to_string() + &user.clone() + &".ftok".to_string());
    if files::check_file(path.clone()) == false {
        println!("file not found : {} || result : {}",path,files::check_file(path.clone()));
        return false
    }
    //hash256
    let read = files::read_file(path.clone());
    let disguise = hash256(token);
    if disguise == read[2].to_string() {
        return true
    } else {
        return false
    }
}
