use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub email_verified: bool,
    pub email_verification_sent: bool,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub url: Option<String>,
    pub is_admin: bool,
    pub publish_notifications: bool,
}