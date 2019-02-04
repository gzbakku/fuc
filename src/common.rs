

#[allow(dead_code)]
pub fn log(t:String,s:String,l:bool){
    if l == true {
        let hold = ">>> ".to_string()  + &t + &" ".to_string() + &s;
        println!("{}",hold);
    }
}

#[allow(dead_code)]
pub fn error(e:String){
    let hold = "!!! ".to_string() + &e;
    println!("{}",hold);
}
