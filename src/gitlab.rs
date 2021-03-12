use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref defaultBaseUrl: &'static str ="https://gitlab.com/";
    pub static ref apiVersionPath: &'static str ="api/v4/";
    pub static ref userAgent: &'static str ="gitlab-rs";

    pub static ref headerRateLimit: &'static str ="RateLimit-Limit";
    pub static ref headerRateReset: &'static str ="RateLimit-Reset";
}

/// List of available authentication types.
///
/// GitLab API docs: https://docs.gitlab.com/ce/api/
#[derive(Debug)]
pub enum AuthenticationKind {
    BasicAuth,
    OAuthToken,
    PrivateToken,
}

#[derive(Debug)]
pub struct Client {
    pub auth_type: AuthenticationKind,

    // if auth_type is BasicAuth
    pub username: String,
    pub password: String,

    pub user_agent: String,
}

pub struct ListOptions {
    pub page: i32,
    pub per_page: i32,
}

impl Default for Client {
    fn default() -> Client {
        Client {
            auth_type: AuthenticationKind::BasicAuth,
            username: "".to_string(),
            password: "".to_string(),

            user_agent: (*userAgent).to_string(),
        }
    }
}

impl Client {
    pub fn new_basic_auth_client(username: String, password: String) -> Client {
        Client {
            username,
            password,
            ..Default::default()
        }
    }
}