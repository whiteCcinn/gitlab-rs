use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fmt::{Formatter, Display, Result};

lazy_static! {
    pub static ref ACCESS_REQUEST: &'static str ="access_requests";
    pub static ref APPROVE: &'static str ="approve";
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectsAccessRequestResponse<'a> {
    pub id: i32,
    pub username: &'a str,
    pub name: &'a str,
    pub state: &'a str,
    pub created_at: &'a str,
    pub requested_at: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupsAccessRequestResponse<'a> {
    pub id: i32,
    pub username: &'a str,
    pub name: &'a str,
    pub state: &'a str,
    pub created_at: &'a str,
    pub requested_at: &'a str,
}

#[derive(FromPrimitive, Debug, Copy, Clone)]
pub enum AccessLevel {
    NoAccess = 0,
    MinimalAccess = 5,
    Guest = 10,
    Reporter = 20,
    Developer = 30,
    Maintainer = 40,
    Owner = 50,
}

impl Display for AccessLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let query_string = format!("access_level={}", *self as i32);
        write!(f, "{}", query_string)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApproveAccessRequestResponse<'a> {
    pub id: i32,
    pub username: &'a str,
    pub name: &'a str,
    pub state: &'a str,
    pub created_at: &'a str,
    pub access_level: i32,
}
