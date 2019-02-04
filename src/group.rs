
#[path="./common.rs"]
mod common;

const LOG:bool = false;

//find what collection to put the word in
#[allow(dead_code)]
pub fn groupify(s:String) -> String {

    common::log("----------------".to_string(),"".to_string(),LOG);

    common::log("groupifying".to_string(),"".to_string(),LOG);

    let n = convert_string_to_u64(s.clone());

    common::log("converted number :".to_string(),n.clone().to_string(),LOG);

    if n.to_string().len() == 3 {
        if n < 500 {
            return "500".to_string();
        }
        if n > 500 {
            return "1000".to_string();
        }
    }

    let s = n.to_string();

    let l = last_3(s.clone());

    common::log("last number :".to_string(),l.clone().to_string(),LOG);

    fn last_3(s:String) -> u64 {
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
        h.parse::<u64>().unwrap()
    }

    let g = groupificate(s.clone());

    common::log("initial number :".to_string(),g.clone().to_string(),LOG);

    //this function returns first numbers other then the last 3
    fn groupificate(s:String) -> u64 {
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
        h.parse::<u64>().unwrap()
    }

    let b;
    if l < 500 {
        b = g.to_string() + &"500".to_string();
    } else {
        b = (g + 1).to_string() + &"000".to_string();
    }

    common::log("final number :".to_string(),b.clone().to_string(),LOG);

    common::log("----------------".to_string(),"".to_string(),LOG);

    return b

}

pub fn convert_string_to_u64(n:String) -> u64 {

    common::log("convertifying".to_string(),"".to_string(),LOG);

    let pos = n.find(".");
    match pos {
        Some(_n)=>{
            let mut h = Vec::new();
            for i in n.to_string().split(".") {
                h.push(i.to_string());
            }
            return h[0].parse::<u64>().unwrap();
        },
        None=>{
            return n.to_string().parse::<u64>().unwrap();
        }
    }

}
