use lazy_static::lazy_static;
use crate::gitlab::{Client};
use std::fmt::Display;
use crate::restful::Kind;
use crate::common_resources::merge_requests as common_merge_request;
use std::borrow::Cow;

lazy_static! {
    pub static ref PROJECTS: &'static str ="projects";
}

/// Get all merge requests the authenticated user has access to
///
/// GitLab API docs:
/// https://docs.gitlab.com/ce/api/merge_requests.html


/// Gets a list of access requests viewable by the authenticated user.
///
/// GitLab API docs:
/// https://docs.gitlab.com/ce/api/merge_requests.html#list-merge-requests
#[derive(Debug)]
pub struct ListMergeRequestRequest {
    pub merge_requests: &'static str,
    pub state: Option<&'static str>,
}

impl Default for ListMergeRequestRequest {
    fn default() -> Self {
        Self {
            merge_requests: *(common_merge_request::MERGE_REQUESTS),
            state: None,
        }
    }
}

// impl EndPoint for ListMergeRequestRequest {
//     fn get_endpoint(&self) -> (Kind, String) {
//         return (Kind::GET,
//                 format!("{merge_requests}",
//                         merge_requests = self.merge_requests,
//                 )
//         );
//     }
//
//     fn get_query_string<'a>(&self) -> String {
//         let mut query_string = String::new();
//         query_string += match self.state {
//             Some(v) => {
//                 "state=" + v
//             }
//             None => ""
//         };
//         query_string
//     }
// }

impl ListMergeRequestRequest {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}





