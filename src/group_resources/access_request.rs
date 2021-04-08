use lazy_static::lazy_static;
use crate::gitlab::Client;
use crate::restful::Kind;
use std::fmt::Display;
use crate::common_resources::access_request as common_access_request;
use crate::common_resources::access_request::AccessLevel;
use derive_macro::Endpoint;
/// AccessRequest represents a access request for a group or project.
///
/// GitLab API docs:
/// https://docs.gitlab.com/ce/api/access_requests.html

/// Gets a list of access requests viewable by the authenticated user.
#[derive(Debug, Endpoint)]
pub struct ListGroupsAccessRequestRequest<'a> {
    #[method(GET)]
    pub method: &'a str,
    #[endpoint("/group/{id}/access_requests")]
    pub endpoint: &'a str,
    // To be compatible with both, the string type is used
    // integer/string
    pub id: &'a str
}

