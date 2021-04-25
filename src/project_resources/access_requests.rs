use derive_macro::Endpoint;
use serde::{Serialize, Deserialize};
use crate::gitlab::EndPointTrait;
use crate::restful::Kind;

/// ListAccessRequests gets a list of access requests
/// viewable by the authenticated user.
///
/// GitLab API docs:
/// https://docs.gitlab.com/ce/api/access_requests.html#list-access-requests-for-a-group-or-project
#[derive(Debug, Endpoint)]
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
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

mod test {
    use crate::gitlab::Client;
    use crate::common_resources::payload::ErrorMessage;
    use crate::gitlab::EndPointTrait;
    use crate::project_resources::access_requests::ListAccessRequests;

    #[test]
    fn test_list_access_requests() {
        let mut instance = ListAccessRequests::new("6");

        let expected_endpoint = "/projects/6/access_requests";
        assert_eq!(instance.get_endpoint(), expected_endpoint);
    }

    #[test]
    fn request_list_access_requests() -> Result<(), Box<dyn std::error::Error>> {
        let mut client = Client::new("zFabz1E4tGc8HvUmo_26".to_string());
        client.set_base_url("http://gitlab.ccinn.com/".to_string()).unwrap();
        let mut instance = ListAccessRequests::new("6");
        let response = client.request(instance);
        if response.status().is_success() {
            let json = response.text().unwrap();
            println!("{}", json);
            let rs: serde_json::Value = serde_json::from_str(json.as_str()).unwrap();
            println!("{:#?}", rs);
        } else if response.status().is_client_error() {
            let json = response.text().unwrap();
            println!("{}", json);
            let rs: ErrorMessage
                = serde_json::from_str(json.as_str()).unwrap();
            println!("{:#?}", rs);
        }
        Ok(())
    }
}