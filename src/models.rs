use serde_json;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub src: i32,
    pub dest: i32,
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


