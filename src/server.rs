
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
