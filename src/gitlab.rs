use lazy_static::lazy_static;
use url::{Url, ParseError};
use crate::restful::Kind;
use reqwest;
use serde::{Serialize, Deserialize};
use crate::common_resources::payload::{Namespace, InternalLinks, ContainerExpirationPolicy, Permissions};



lazy_static! {
    pub static ref DEFAULT_BASE_URL: &'static str ="https://gitlab.com/";
    pub static ref API_VERSION_PATH: &'static str ="api/v4";
    pub static ref USER_AGENT: &'static str ="gitlab-rs";

    pub static ref HEADER_RATE_LIMIT: &'static str ="RateLimit-Limit";
    pub static ref HEADER_RATE_RESET: &'static str ="RateLimit-Reset";
}

#[derive(Debug)]
pub struct ListsProjectsRequest;

#[derive(Serialize, Deserialize, Debug)]
pub struct ListsProjectsResponse<'a> {
    pub id: i32,
    pub description: Option<&'a str>,
    pub name: &'a str,
    pub name_with_namespace: &'a str,
    pub path: &'a str,
    pub path_with_namespace: &'a str,
    pub created_at: &'a str,
    pub default_branch: Option<&'a str>,
    pub tag_list: Vec<&'a str>,
    pub ssh_url_to_repo: &'a str,
    pub http_url_to_repo: &'a str,
    pub web_url: &'a str,
    pub readme_url: Option<&'a str>,
    pub avatar_url: Option<&'a str>,
    pub forks_count: i32,
    pub star_count: i32,
    pub last_activity_at: &'a str,
    pub namespace: Namespace<'a>,
    pub _links: InternalLinks<'a>,
    pub packages_enabled: bool,
    pub empty_repo: bool,
    pub archived: bool,
    pub visibility: &'a str,
    pub resolve_outdated_diff_discussions: bool,
    pub container_registry_enabled: bool,
    pub container_expiration_policy: ContainerExpirationPolicy<'a>,
    pub issues_enabled: bool,
    pub merge_requests_enabled: bool,
    pub wiki_enabled: bool,
    pub jobs_enabled: bool,
    pub snippets_enabled: bool,
    pub service_desk_enabled: bool,
    pub service_desk_address: Option<&'a str>,
    pub can_create_merge_request_in: bool,
    pub issues_access_level: &'a str,
    pub repository_access_level: &'a str,
    pub merge_requests_access_level: &'a str,
    pub forking_access_level: &'a str,
    pub wiki_access_level: &'a str,
    pub builds_access_level: &'a str,
    pub snippets_access_level: &'a str,
    pub pages_access_level: &'a str,
    pub operations_access_level: &'a str,
    pub analytics_access_level: &'a str,
    pub emails_disabled: Option<bool>,
    pub shared_runners_enabled: bool,
    pub lfs_enabled: bool,
    pub creator_id: i32,
    pub import_status: &'a str,
    pub open_issues_count: i32,
    pub ci_default_git_depth: i32,
    pub ci_forward_deployment_enabled: bool,
    pub public_jobs: bool,
    pub build_timeout: i32,
    pub auto_cancel_pending_pipelines: &'a str,
    pub build_coverage_regex: Option<&'a str>,
    pub ci_config_path: Option<&'a str>,
    pub shared_with_groups: Vec<&'a str>,
    pub only_allow_merge_if_pipeline_succeeds: bool,
    pub allow_merge_on_skipped_pipeline: Option<bool>,
    pub request_access_enabled: bool,
    pub only_allow_merge_if_all_discussions_are_resolved: bool,
    pub remove_source_branch_after_merge: bool,
    pub printing_merge_request_link_enabled: bool,
    pub merge_method: &'a str,
    pub suggestion_commit_message: Option<&'a str>,
    pub auto_devops_enabled: bool,
    pub auto_devops_deploy_strategy: &'a str,
    pub autoclose_referenced_issues: bool,
    pub permissions: Permissions,

}


impl ListsProjectsRequest {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug)]
pub struct ListsGroupsRequest;

#[derive(Serialize, Deserialize, Debug)]
pub struct ListsGroupsResponse<'a> {
    pub id: i32,
    pub web_url: &'a str,
    pub name: &'a str,
    pub path: &'a str,
    pub description: &'a str,
    pub visibility: &'a str,
    pub share_with_group_lock: bool,
    pub require_two_factor_authentication: bool,
    pub two_factor_grace_period: i32,
    pub project_creation_level: &'a str,
    pub auto_devops_enabled: Option<bool>,
    pub subgroup_creation_level: &'a str,
    pub emails_disabled: Option<bool>,
    pub mentions_disabled: Option<bool>,
    pub lfs_enabled: bool,
    pub default_branch_protection: i32,
    pub avatar_url: Option<&'a str>,
    pub request_access_enabled: bool,
    pub full_name: &'a str,
    pub full_path: &'a str,
    pub created_at: &'a str,
    pub parent_id: Option<i32>,
}

impl ListsGroupsRequest {
    pub fn new() -> Self {
        Self
    }
}

pub trait EndPointTrait {
    fn get_endpoint(&self) -> String;
    fn get_query(&self) -> String;
    fn get_query_fields(&self) -> Vec<&str>;
    fn get_method(&self) -> Kind;
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

    // pub access_request_service: Option<Box<AccessRequestService>>
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

            http_client,

            // access_request_service: None
        };

        // client.access_request_service = Some(Box::new(AccessRequestService{client: &client}));

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


    pub fn request(&self, t: impl EndPointTrait) -> reqwest::blocking::Response {
        let endpoint = t.get_endpoint();
        let r#type = t.get_method();

        let endpoint_chain = self.get_endpoint_url(endpoint).unwrap();

        println!("{}", endpoint_chain);

        let mut request_builder = match r#type {
            Kind::GET => {
                self.http_client.get(endpoint_chain)
            }
            Kind::PUT => {
                self.http_client.put(endpoint_chain)
            }
            Kind::POST => {
                self.http_client.post(endpoint_chain)
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