use lazy_static::lazy_static;
use url::{Url, ParseError};
use crate::restful::Kind;
use reqwest;
use crate::common_resources::payload::{Namespace, InternalLinks, ContainerExpirationPolicy, Permissions};

lazy_static! {
    pub static ref DEFAULT_BASE_URL: &'static str ="https://gitlab.com/";
    pub static ref API_VERSION_PATH: &'static str ="api/v4";
    pub static ref USER_AGENT: &'static str ="gitlab-rs";

    pub static ref HEADER_RATE_LIMIT: &'static str ="RateLimit-Limit";
    pub static ref HEADER_RATE_RESET: &'static str ="RateLimit-Reset";
}

pub trait EndPointTrait {
    fn get_method(&self) -> Kind;
    fn get_endpoint(&self) -> String;
    fn get_query_fields(&self) -> Vec<&'static str>;
    fn get_query(&self) -> String;
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

    // if auth_type is PrivateToken or OAuthToken
    pub token: String,

    pub user_agent: String,

    pub base_url: String,

    pub http_client: reqwest::blocking::Client,

    // pub list_options: ListOptions,
}

pub struct ListOptions {
    pub page: i32,
    pub per_page: i32,
}

impl Default for Client {
    fn default() -> Client {
        let http_client = reqwest::blocking::Client::builder().build().unwrap();
        let client = Client {
            auth_type: AuthenticationKind::BasicAuth,

            username: "".to_string(),
            password: "".to_string(),

            token: "".to_string(),

            user_agent: (*USER_AGENT).to_string(),

            base_url: (*DEFAULT_BASE_URL).to_string(),

            http_client
        };
        client
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

    pub fn new(token: String) -> Client {
        Client {
            auth_type: AuthenticationKind::PrivateToken,
            token,
            ..Default::default()
        }
    }

    pub fn new_oauth_token(token: String) -> Client {
        Client {
            auth_type: AuthenticationKind::OAuthToken,
            token,
            ..Default::default()
        }
    }

    /// set_base_url sets the base URL for API requests to a custom endpoint.
    pub fn set_base_url(&mut self, mut base_url: String) -> Result<String, ParseError> {
        // Make sure the given URL end with a slash
        if !base_url.as_str().ends_with("/") {
            base_url += "/"
        }

        let mut base = Url::parse(base_url.as_str())?;

        if !base_url.as_str().ends_with(*API_VERSION_PATH) {
            base = base.join(*API_VERSION_PATH)?
        }

        self.base_url = base.to_string();
        Ok(base.to_string())
    }

    pub fn get_endpoint_url(&self, endpoint: String) -> Result<String, ParseError> {
        let url = Url::parse(self.base_url.as_str())?;

        let url = url.to_string() + endpoint.as_str();

        Ok(url.to_string())
    }


    pub fn request(&self, t: impl EndPointTrait+ serde::Serialize) -> reqwest::blocking::Response {
        let endpoint = t.get_endpoint();
        let r#type = t.get_method();

        let mut endpoint_chain = self.get_endpoint_url(endpoint).unwrap();

        let mut request_builder = match r#type {
            Kind::GET => {
                endpoint_chain += t.get_query().as_str();
                self.http_client.get(endpoint_chain)
            }
            Kind::PUT => {
                self.http_client.put(endpoint_chain)
            }
            Kind::POST => {
                self.http_client.post(endpoint_chain).json(&t)
            }
            Kind::DELETE => {
                self.http_client.delete(endpoint_chain)
            }
        };
        // let mut request_builder = self.http_client.get(endpoint_chain);

        request_builder = match self.auth_type {
            AuthenticationKind::BasicAuth => {
                // todo: this is error ,base-auth get the token
                request_builder.header("PRIVATE-TOKEN", self.token.to_string())
            }
            AuthenticationKind::OAuthToken => {
                request_builder.header("PRIVATE-TOKEN", self.token.to_string())
            }
            AuthenticationKind::PrivateToken => {
                request_builder.header("PRIVATE-TOKEN", self.token.to_string())
            }
        };

        let response = request_builder.send().unwrap();
        return response;
    }
}