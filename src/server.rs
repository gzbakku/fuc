//***************************************************
//init

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Serialize)]
pub struct Key {
    pub key:String
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Serialize)]
pub struct ResultKey {
    pub success:bool,
    pub error:String,
    pub docs:Key,
    pub message:String
}

//***************************************************
//update

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Serialize)]
pub struct DocId {
    pub new_id:String
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Serialize)]
pub struct ResultUpdate {
    pub success:bool,
    pub error:String,
    pub docs:DocId,
    pub message:String
}

#[allow(dead_code)]
pub fn success_update(s:String) -> String {
    stringify_update(ResultUpdate {
        success:true,
        error:String::new(),
        docs:DocId{
            new_id:s
        },
        message:String::new(),
    })
}

#[allow(dead_code)]
fn stringify_update(hold: ResultUpdate) -> String {
    let work = serde_json::to_string(&hold);
    match work {
        Ok(n) => {
            return n
        },
        Err(err) => {
            println!("{:?}",err);
            return "error".to_string()
        }
    };
}


//***************************************************
//connect

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Serialize)]
pub struct Token {
    pub user:String,
    pub token:String
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Serialize)]
pub struct ResultToken {
    pub success:bool,
    pub error:String,
    pub docs:Token,
    pub message:String
}

//***************************************************
//query

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Serialize)]
pub struct ResultQuery {
    pub success:bool,
    pub error:String,
    pub docs:Vec<String>,
    pub message:String
}

#[allow(dead_code)]
pub fn success_query(c:Vec<String>) -> String {
    stringify_query(ResultQuery {
        success:true,
        error:String::new(),
        docs:c,
        message:String::new(),
    })
}

#[allow(dead_code)]
fn stringify_query(hold: ResultQuery) -> String {
    let work = serde_json::to_string(&hold);
    match work {
        Ok(n) => {
            return n
        },
        Err(err) => {
            println!("{:?}",err);
            return "error".to_string()
        }
    };
}

//***************************************************
//docs

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Serialize)]
pub struct ResultDocs {
    pub success:bool,
    pub error:String,
    pub docs:Vec<serde_json::value::Value>,
    pub message:String
}

#[allow(dead_code)]
pub fn success_docs(c:Vec<serde_json::value::Value>) -> String {
    stringify_docs(ResultDocs {
        success:true,
        error:String::new(),
        docs:c,
        message:String::new(),
    })
}

#[allow(dead_code)]
fn stringify_docs(hold: ResultDocs) -> String {
    let work = serde_json::to_string(&hold);
    match work {
        Ok(n) => {
            return n
        },
        Err(err) => {
            println!("{:?}",err);
            return "error".to_string()
        }
    };
}

//***************************************************
//common

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Serialize)]
pub struct Result {
    pub success:bool,
    pub error:String,
    pub docs:String,
    pub message:String
}

#[allow(dead_code)]
pub fn success() -> String {
    stringify(Result {
        success:true,
        error:String::new(),
        docs:String::new(),
        message:String::new(),
    })
}

#[allow(dead_code)]
pub fn error(err:String) -> String {
    stringify(Result {
        success:false,
        error:String::from(err),
        docs:String::new(),
        message:String::new(),
    })
}

#[allow(dead_code)]
fn stringify(hold: Result) -> String {
    let work = serde_json::to_string(&hold);
    match work {
        Ok(n) => {
            return n
        },
        Err(err) => {
            println!("{:?}",err);
            return "error".to_string()
        }
    };
}
