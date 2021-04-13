# gitlab-rs
gitlab client

## resources 

[gitlab-api-resources](https://docs.gitlab.com/ee/api/api_resources.html)

- [project resources](https://docs.gitlab.com/ee/api/api_resources.html#project-resources)
- [group resources](https://docs.gitlab.com/ee/api/api_resources.html#group-resources)
- [standalone resources](https://docs.gitlab.com/ee/api/api_resources.html#standalone-resources)

## Usage

```rust
use crate::access_requests::project::ListAccessRequests;
use crate::gitlab::Client;
use crate::common_resources::payload::{ErrorMessage};
use crate::gitlab::EndPointTrait;

#[test]
fn request_group_access_request() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::new("zFabz1E4tGc8HvUmo_26".to_string());
    client.set_base_url("http://gitlab.ccinn.com/".to_string()).unwrap();

    let response = client.request(ListAccessRequests::new("2"));

    if response.status().is_success() {
        let json = response.text().unwrap();
        let rs: serde_json::Value = serde_json::from_str(json.as_str()).unwrap();
    } else if response.status().is_client_error() {
        let json = response.text().unwrap();

        let rs: ErrorMessage
            = serde_json::from_str(json.as_str()).unwrap();
    }

    Ok(())
}

```

## derive-macro : Endpoint

The procedure macro [Endpoint] is implemented, which contains `three properties`, as follows:

properties:

- method(xx)
- endpoint("xx")
- query(xx)

struct method(impl EndpointTrait):

- `fn new(xx) -> Self`  new a resource object
- `fn get_endpoint(&self) -> String` get real endpoint 
- `fn get_query(&self) -> String` get query_string
- `fn get_query_fields(&self) -> Vec<&'static str>` get query_string_fields
- `fn get_method(&self) -> Kind`  get http method

### method

Method is to mark the resource request object way of HTTP request is `[GET | POST | PUT | DELETE]`, ignoring the case.

options:

- GET
- POST
- PUT
- DELETE

### endpoint

The endpoint is primarily the tagged `URL path` template

### query

Query mainly marks the `query_string` parameter field, which is set to `None` by default in Rust, so that the parameter is not passed to GitLab and can be changed directly by modifying the object's properties.