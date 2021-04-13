use derive_macro::Endpoint;
use crate::gitlab::EndPointTrait;
use crate::restful::Kind;

/// ListAccessRequests gets a list of access requests
/// viewable by the authenticated user.
///
/// GitLab API docs:
/// https://docs.gitlab.com/ce/api/access_requests.html#list-access-requests-for-a-group-or-project
#[derive(Debug, Endpoint)]
pub struct ListAccessRequests<'a> {
    #[method(GET)]
    pub method: &'a str,
    #[endpoint("/projects/{id}/access_requests")]
    pub endpoint: &'a str,
    /// To be compatible with both, the string type is used
    /// integer/string
    pub id: &'a str,
}

/// RequestAccess requests access for the authenticated user
/// to a group.
///
/// GitLab API docs:
/// https://docs.gitlab.com/ce/api/access_requests.html#request-access-to-a-group-or-project
#[derive(Debug, Endpoint)]
pub struct RequestAccess<'a> {
    #[method(POST)]
    pub method: &'a str,
    #[endpoint("/projects/{id}/access_requests")]
    pub endpoint: &'a str,
    /// To be compatible with both, the string type is used
    /// integer/string
    pub id: &'a str,
}

/// ApproveAccessRequest approves an access request for the given user.
///
/// GitLab API docs:
/// https://docs.gitlab.com/ce/api/access_requests.html#approve-an-access-request
#[derive(Debug, Endpoint)]
pub struct ApproveAccessRequest<'a> {
    #[method(PUT)]
    pub method: &'a str,
    #[endpoint("/projects/{id}/access_requests/{user_id}/approve")]
    pub endpoint: &'a str,
    /// To be compatible with both, the string type is used
    /// integer/string
    pub id: &'a str,
    /// The user ID of the access requester
    pub user_id: i32,
    // /// A valid access level (defaults: 30, developer access level)
    #[query(None)]
    pub access_level: Option<i32>,
}

/// DenyAccessRequest denies an access request for the given user.
///
/// GitLab API docs:
/// https://docs.gitlab.com/ce/api/access_requests.html#deny-an-access-request
#[derive(Debug, Endpoint)]
pub struct DenyAccessRequest<'a> {
    #[method(DELETE)]
    pub method: &'a str,
    #[endpoint("/projects/{id}/access_requests/{user_id}")]
    pub endpoint: &'a str,
    /// To be compatible with both, the string type is used
    /// integer/string
    pub id: &'a str,
    /// The user ID of the access requester
    pub user_id: i32,
}
