use serde_json;

use std::fmt;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub src: i32,
    pub privacy: i32,
    pub content_warning: Option<String>,
    pub text: Option<String>,
    pub image_data: Option<serde_json::Value>,
    pub time: chrono::NaiveDateTime
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Account {
    pub id: i32,
    pub url: String
}

impl Account {
    fn to_string(&self) -> &String {
        &self.url
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Queryable)]
pub struct PostDest {
    pub id: i32,
    pub post_id: i32,
    pub dest_id: i32
}
