use serde::{Deserialize, Serialize};

pub struct SignUpDto {
    pub token:String,
    pub user: UserOutDto,
}

#[derive(Serialize)]
pub struct UserOutDto {
    pub username:String,
    pub email:String,
    pub pubkey:String,
    pub encrypted_content:String,
}

#[derive(Serialize)]
pub struct ValidationError {
    pub loc:Vec<String>,
    pub msg:String,
    pub r#type:String,
}

#[derive(Serialize,Debug)]
pub struct LoginChallengeOutDto {
    pub salt:String,
    pub challenge:String,
    pub version:i32,
}

#[derive(Deserialize)]
pub struct LoginChallengeInDto {
    pub username:String,
}

#[derive(Deserialize)]
pub enum AccessLevels {
    ReadOnly = 0,
    Admin = 1,
    ReadWrite = 2,
}



