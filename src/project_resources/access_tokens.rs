use derive_macro::Endpoint;
use serde::{Serialize, Deserialize};
use crate::gitlab::{EndPointTrait};
use crate::restful::Kind;

/// Get a list of project access tokens.
///
/// GitLab API docs:
/// https://docs.gitlab.com/ce/api/resource_access_tokens.html#list-project-access-tokens
#[derive(Debug, Endpoint)]
pub struct ListAccessTokens<'a> {
    #[method(GET)]
    pub method: &'a str,
    #[endpoint("/projects/{id}/access_tokens")]
    pub endpoint: &'a str,
    /// To be compatible with both, the string type is used
    /// integer/string
    pub id: &'a str,
}

/// Create a project access token.
///
/// GitLab API docs:
/// https://docs.gitlab.com/ce/api/resource_access_tokens.html#create-a-project-access-token
#[derive(Debug, Endpoint)]
#[derive(Serialize, Deserialize)]
pub struct CreateAccessTokens<'a> {
    #[method(POST)]
    #[serde(skip_serializing)]
    pub method: &'a str,
    #[endpoint("/projects/{id}/access_tokens")]
    #[serde(skip_serializing)]
    pub endpoint: &'a str,
    /// To be compatible with both, the string type is used
    /// integer/string
    #[serde(skip_serializing)]
    pub id: &'a str,

    #[query(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[query(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<&'a str>>,
    // pub scopes: Option<&'a str>,
    #[query(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<&'a str>,
}

mod test {
    use crate::gitlab::Client;
    use crate::common_resources::payload::ErrorMessage;
    use crate::project_resources::access_tokens::CreateAccessTokens;
    use crate::gitlab::EndPointTrait;

    #[test]
    fn test_create_access_tokens() {
        let mut instance = CreateAccessTokens::new("456");
        instance.name = Some("caiwenhui");

        let expected_endpoint = "/projects/456/access_tokens";
        assert_eq!(instance.get_endpoint(), expected_endpoint);

        let expected_json = r#"{"name":"caiwenhui"}"#;
        assert_eq!(expected_json,  serde_json::to_string(&instance).unwrap());
    }

    #[test]
    fn request_create_access_tokens() -> Result<(), Box<dyn std::error::Error>> {
        let mut client = Client::new("zFabz1E4tGc8HvUmo_26".to_string());
        client.set_base_url("http://gitlab.ccinn.com/".to_string()).unwrap();
        let mut instance = CreateAccessTokens::new("6");
        instance.name = Some("caiwenhui");
        instance.scopes = Some(vec!["api"]);
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
