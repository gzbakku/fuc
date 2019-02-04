
use crypto::md5::Md5;
use crypto::digest::Digest;

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct OrderBy {
    pub tag:String,
    pub direction:String
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Tag {
    pub tag:String,
    pub data_type:String,
    pub function:String
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Index {
    pub valid:bool,
    pub error:String,
    pub order_exists:bool,
    pub tags_exists:bool,
    pub order:OrderBy,
    pub tags:Vec<Tag>,
    pub hash:String,
    pub index_id:String,
    pub index_type:String
}

//this makes a index struct from a index query stirng
#[allow(dead_code)]
pub fn indexify(q:String,a:String) -> Index {

    //init index struct
    let mut index = Index {
        valid:true,
        error:String::new(),
        order_exists:false,
        order:OrderBy{
            tag:String::new(),
            direction:String::new()
        },
        tags_exists:false,
        tags:Vec::new(),
        hash:collection_id(a),
        index_id:md5(q.clone()),
        index_type:String::new()
    };

    //split order and tags
    let mut pool = Vec::new();
    for i in q.split("_") {
        pool.push(i);
    }

    if pool.len() > 3 {
        index.error = "invalid-query-cannot.use._.in.tag".to_string();
        index.valid = false;
    }

    //process order here
    if pool.len() == 2 {
        let mut k = Vec::new();
        for i in pool[1].split(".") {
            k.push(i);
        }
        if k.len() == 2 {
            index.order_exists = true;
            index.order.tag = k[0].to_string();
            index.order.direction = k[1].to_string();
            index.index_type = "order".to_string();
        } else {
            index.error = "invalid-order_by".to_string();
            index.valid = false;
        }
    }
    if index.valid == false {
        return index
    }

    //process tags here
    let tags = pool[0];
    let mut search = false;
    let mut weight = false;

    let mut tags_vec = Vec::new();

    for i in tags.split("||") {
        let mut z = Vec::new();
        for j in i.split(".") {
            z.push(j);
        }
        if z.len() == 3 {
            let hold = Tag {
                tag:z[0].to_string(),
                data_type:z[1].to_string(),
                function:z[2].to_string()
            };
            if hold.function == "weight" {
                if search == true || weight == true {
                    index.error = "invalid-query-weight/search-both_exists".to_string();
                    index.valid = false;
                }
                weight = true;
            }
            if hold.function == "search" {
                if search == true || weight == true {
                    index.error = "invalid-query-weight/search-both_exists".to_string();
                    index.valid = false;
                }
                search = true;
            }
            if index.tags_exists == false {
                index.tags_exists = true;
            }
            if hold.function == "equal" || hold.function == "search" || hold.function == "weight" {
                tags_vec.push(hold);
            }
        }
        //check if the tag data vec have 3 items
    }
    //loop through tags string split here

    //check if search, weigth and order does not exists in the same query
    if search == true || weight == true {
        if index.order_exists == true {
            index.valid = false;
            index.error = "invalid-query-weight/search-order_by-both_exists".to_string();
        }
    }

    if search == true {
        index.index_type = "search".to_string();
    }
    if weight == true {
        index.index_type = "weight".to_string();
    }
    if search == false && weight == false {
        index.index_type = "equal".to_string();
    }

    index.tags = sort_tags(tags_vec);

    return index

}
//indexify ends here

//sort tags by campatible order of equal search weight and order
#[allow(dead_code)]
fn sort_tags(t:Vec<Tag>) -> Vec<Tag> {

    let mut e = Vec::new();
    let mut w = Vec::new();
    let mut s = Vec::new();

    for i in t {
        if i.function == "equal" {e.push(i);}
        else if i.function == "weight" {w.push(i);}
        else if i.function == "search" {s.push(i);}
    }

    let mut f = Vec::new();
    for i in e {f.push(i);}
    for i in w {f.push(i);}
    for i in s {f.push(i);}
    f

}

//this makes a vec of all only collection names in a address string
#[allow(dead_code)]
pub fn address_collection_vec(a:String) -> Vec<String> {
    let mut b = Vec::new();
    for i in a.split("=>") {
        b.push(i);
    }
    let mut f = Vec::new();
    for i in b {
        let mut h = Vec::new();
        for j in i.split("_") {
            h.push(j.to_string());
        }
        f.push(h[0].to_string());
    }
    f
}

//this makes a vec of all the address pointers in a address string
#[allow(dead_code)]
pub fn address_vec(a:String) -> Vec<String> {
    let mut b = Vec::new();
    for i in a.split("=>") {
        b.push(i);
    }
    let mut f = Vec::new();
    for i in b {
        for j in i.split("_") {
            f.push(j.to_string());
        }
    }
    f
}

//this makes collection id to check indexes
#[allow(dead_code)]
pub fn collection_id(a:String) -> String {
    md5(addressify(a))
}

//this makes a stirng of collection names from address string for index recognition
#[allow(dead_code)]
pub fn addressify(a:String) -> String {
    let mut b = Vec::new();
    for i in a.split("=>") {
        b.push(i);
    }
    let mut first = true;
    let mut f = String::new();
    for i in b {
        let mut h = Vec::new();
        for j in i.split("_") {
            h.push(j.to_string());
        }
        if first == true {
            f = h[0].to_string();
        } else {
            f.push_str(&("-".to_string() + &h[0].to_string()));
        }
        first = false;
    }
    f
}

#[allow(dead_code)]
pub fn address_type(s:String) -> String {
    let a = address_vec(s);
    let mut hold = "doc";
    for _i in a {
        if hold == "collection" {
            hold = "doc";
        } else {
            hold = "collection";
        }
    }
    hold.to_string()
}

#[allow(dead_code)]
pub fn address_locatify(a:String) -> String {
    let h = address_vec(a.clone());
    let mut s = String::from("\\fuc\\collections");
    for i in h {
        s.push_str(&("\\".to_string() + &i.to_string()));
    }
    s
}

#[allow(dead_code)]
pub fn md5(s:String) -> String {
    let mut hasher = Md5::new();
    hasher.input_str(&s);
    hasher.result_str()
}

#[allow(dead_code)]
pub fn clean(s:String) -> String {
    let mut hold = String::new();
    for i in s.chars() {
        for j in i.to_string().bytes(){
            if j != 34 {
                hold.push_str(&i.to_string());
            }
        }
    }
    hold
}
