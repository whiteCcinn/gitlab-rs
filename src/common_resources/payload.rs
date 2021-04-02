use serde::{Serialize, Deserialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessage<'a> {
    pub message: Option<&'a str>,
    pub error:  Option<&'a str>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessageWithoutSudo<'a> {
    pub error: &'a str,
    pub error_description: &'a str,
    pub message: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Namespace<'a> {
    pub id: i32,
    pub name: &'a str,
    pub path: &'a str,
    pub kind: &'a str,
    pub full_path: &'a str,
    pub parent_id: Option<i32>,
    pub avatar_url: Option<Cow<'a, str>>,
    pub web_url: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InternalLinks<'a> {
    #[serde(rename = "self")]
    pub _self: &'a str,
    pub issues: &'a str,
    pub merge_requests: &'a str,
    pub repo_branches: &'a str,
    pub labels: &'a str,
    pub events: &'a str,
    pub members: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerExpirationPolicy<'a> {
    pub cadence: &'a str,
    pub enabled: bool,
    pub keep_n: i32,
    pub older_than: &'a str,
    pub name_regex: &'a str,
    pub name_regex_keep: Option<Cow<'a, str>>,
    pub next_run_at: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectAccess {
    pub access_level: i32,
    pub notification_level: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupAccess {
    pub access_level: i32,
    pub notification_level: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Permissions {
    pub project_access: Option<ProjectAccess>,
    pub group_access: Option<GroupAccess>,
}

