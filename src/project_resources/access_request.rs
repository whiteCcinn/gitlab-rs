use lazy_static::lazy_static;
use crate::gitlab::{Client};
use crate::restful::Kind;
use std::fmt::Display;
use crate::common_resources::access_request as common_access_request;
use crate::common_resources::access_request::AccessLevel;

lazy_static! {
    pub static ref PROJECTS: &'static str ="projects";
}

/// AccessRequest represents a access request for a group or project.
///
/// GitLab API docs:
/// https://docs.gitlab.com/ce/api/access_requests.html


/// Gets a list of access requests viewable by the authenticated user.
#[derive(Debug)]
pub struct ListProjectsAccessRequestRequest<T> {
    pub projects: &'static str,
    pub id: T,
    pub access_request: &'static str,
}

impl<T: Default> Default for ListProjectsAccessRequestRequest<T> {
    fn default() -> Self {
        Self {
            projects: *PROJECTS,
            id: T::default(),
            access_request: *(common_access_request::ACCESS_REQUEST),
        }
    }
}
//
// impl<T: Display> EndPoint for ListProjectsAccessRequestRequest<T> {
//     fn get_endpoint(&self) -> (Kind, String) {
//         return (Kind::GET,
//                 format!("{projects}/{id}/{access_requests}",
//                         projects = self.projects,
//                         id = self.id,
//                         access_requests = self.access_request
//                 )
//         );
//     }
// }

impl<T: Default> ListProjectsAccessRequestRequest<T> {
    pub fn new(id: T) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

/// Requests access for the authenticated user to a group or project.
#[derive(Debug)]
pub struct RequestProjectsAccessRequestRequest<T> {
    pub projects: &'static str,
    pub id: T,
    pub access_request: &'static str,
}

impl<T: Default> Default for RequestProjectsAccessRequestRequest<T> {
    fn default() -> Self {
        Self {
            projects: *PROJECTS,
            id: T::default(),
            access_request: *(common_access_request::ACCESS_REQUEST),
        }
    }
}

// impl<T: Display> EndPoint for RequestProjectsAccessRequestRequest<T> {
//     fn get_endpoint(&self) -> (Kind, String) {
//         return (Kind::GET,
//                 format!("{projects}/{id}/{access_requests}",
//                         projects = self.projects,
//                         id = self.id,
//                         access_requests = self.access_request
//                 )
//         );
//     }
// }

// impl<T: Default> RequestProjectsAccessRequestRequest<T> {
//     pub fn new(id: T) -> Self {
//         Self {
//             id,
//             ..Default::default()
//         }
//     }
// }

/// Approves an access request for the given user.
#[derive(Debug)]
pub struct ApproveProjectsAccessRequestRequest<T> {
    projects: &'static str,
    pub id: T,
    access_request: &'static str,
    pub user_id: i32,
    approve: &'static str,
    pub access_level: AccessLevel,
}

impl<T: Default> Default for ApproveProjectsAccessRequestRequest<T> {
    fn default() -> Self {
        Self {
            projects: *PROJECTS,
            id: T::default(),
            access_request: *(common_access_request::ACCESS_REQUEST),
            user_id: 0,
            approve: *(common_access_request::APPROVE),
            access_level: AccessLevel::Developer,
        }
    }
}

// impl<T: Display> EndPoint for ApproveProjectsAccessRequestRequest<T> {
//     fn get_endpoint(&self) -> (Kind, String) {
//         return (Kind::PUT,
//                 format!("{projects}/{id}/{access_requests}/{user_id}/{approve}?{access_level}",
//                         projects = self.projects,
//                         id = self.id,
//                         access_requests = self.access_request,
//                         user_id = self.user_id,
//                         approve = self.approve,
//                         access_level = self.access_level
//                 )
//         );
//     }
// }

impl<T: Default> ApproveProjectsAccessRequestRequest<T> {
    pub fn new(id: T, user_id: i32) -> Self {
        Self {
            id,
            user_id,
            ..Default::default()
        }
    }
}

/// Denies an access request for the given user.
#[derive(Debug)]
pub struct DeleteProjectsAccessRequestRequest<T> {
    projects: &'static str,
    pub id: T,
    access_request: &'static str,
    pub user_id: i32,
}

impl<T: Default> Default for DeleteProjectsAccessRequestRequest<T> {
    fn default() -> Self {
        Self {
            projects: *PROJECTS,
            id: T::default(),
            access_request: *(common_access_request::ACCESS_REQUEST),
            user_id: 0,
        }
    }
}

// impl<T: Display> EndPoint for DeleteProjectsAccessRequestRequest<T> {
//     fn get_endpoint(&self) -> (Kind, String) {
//         return (Kind::DELETE,
//                 format!("{projects}/{id}/{access_requests}/{user_id}",
//                         projects = self.projects,
//                         id = self.id,
//                         access_requests = self.access_request,
//                         user_id = self.user_id,
//                 )
//         );
//     }
// }

impl<T: Default> DeleteProjectsAccessRequestRequest<T> {
    pub fn new(id: T, user_id: i32) -> Self {
        Self {
            id,
            user_id,
            ..Default::default()
        }
    }
}




