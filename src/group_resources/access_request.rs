use lazy_static::lazy_static;
use crate::gitlab::{EndPoint, Client};
use crate::restful::Kind;
use std::fmt::Display;
use crate::common_resources::access_request as common_access_request;
use crate::common_resources::access_request::AccessLevel;

lazy_static! {
    pub static ref GROUPS: &'static str ="groups";
}

/// AccessRequest represents a access request for a group or project.
///
/// GitLab API docs:
/// https://docs.gitlab.com/ce/api/access_requests.html

/// Gets a list of access requests viewable by the authenticated user.
#[derive(Debug)]
pub struct ListGroupsAccessRequestRequest<T> {
    groups: &'static str,
    pub id: T,
    access_request: &'static str,
}

impl<T: Default> Default for ListGroupsAccessRequestRequest<T> {
    fn default() -> Self {
        Self {
            groups: *GROUPS,
            id: T::default(),
            access_request: *(common_access_request::ACCESS_REQUEST),
        }
    }
}

impl<T: Display> EndPoint for ListGroupsAccessRequestRequest<T> {
    fn get_endpoint(&self) -> (Kind, String) {
        return (Kind::GET,
                format!("{groups}/{id}/{access_requests}",
                        groups = self.groups,
                        id = self.id,
                        access_requests = self.access_request
                )
        );
    }
}

impl<T: Default> ListGroupsAccessRequestRequest<T> {
    pub fn new(id: T) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

/// Requests access for the authenticated user to a group or project.
#[derive(Debug)]
pub struct RequestGroupsAccessRequestRequest<T> {
    groups: &'static str,
    pub id: T,
    access_request: &'static str,
}

impl<T: Default> Default for RequestGroupsAccessRequestRequest<T> {
    fn default() -> Self {
        Self {
            groups: *GROUPS,
            id: T::default(),
            access_request: *(common_access_request::ACCESS_REQUEST),
        }
    }
}

impl<T: Display> EndPoint for RequestGroupsAccessRequestRequest<T> {
    fn get_endpoint(&self) -> (Kind, String) {
        return (Kind::POST,
                format!("{groups}/{id}/{access_requests}",
                        groups = self.groups,
                        id = self.id,
                        access_requests = self.access_request
                )
        );
    }
}

impl<T: Default> RequestGroupsAccessRequestRequest<T> {
    pub fn new(id: T) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

/// Approves an access request for the given user.
#[derive(Debug)]
pub struct ApproveGroupsAccessRequestRequest<T> {
    groups: &'static str,
    pub id: T,
    access_request: &'static str,
    pub user_id: i32,
    approve: &'static str,
    pub access_level: AccessLevel,
}

impl<T: Default> Default for ApproveGroupsAccessRequestRequest<T> {
    fn default() -> Self {
        Self {
            groups: *GROUPS,
            id: T::default(),
            access_request: *(common_access_request::ACCESS_REQUEST),
            user_id: 0,
            approve: *(common_access_request::APPROVE),
            access_level: AccessLevel::Developer,
        }
    }
}

impl<T: Display> EndPoint for ApproveGroupsAccessRequestRequest<T> {
    fn get_endpoint(&self) -> (Kind, String) {
        return (Kind::PUT,
                format!("{groups}/{id}/{access_requests}/{user_id}/{approve}?{access_level}",
                        groups = self.groups,
                        id = self.id,
                        access_requests = self.access_request,
                        user_id = self.user_id,
                        approve = self.approve,
                        access_level = self.access_level
                )
        );
    }
}

impl<T: Default> ApproveGroupsAccessRequestRequest<T> {
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
pub struct DeleteGroupsAccessRequestRequest<T> {
    groups: &'static str,
    pub id: T,
    access_request: &'static str,
    pub user_id: i32,
}

impl<T: Default> Default for DeleteGroupsAccessRequestRequest<T> {
    fn default() -> Self {
        Self {
            groups: *GROUPS,
            id: T::default(),
            access_request: *(common_access_request::ACCESS_REQUEST),
            user_id: 0,
        }
    }
}

impl<T: Display> EndPoint for DeleteGroupsAccessRequestRequest<T> {
    fn get_endpoint(&self) -> (Kind, String) {
        return (Kind::DELETE,
                format!("{groups}/{id}/{access_requests}/{user_id}",
                        groups = self.groups,
                        id = self.id,
                        access_requests = self.access_request,
                        user_id = self.user_id,
                )
        );
    }
}

impl<T: Default> DeleteGroupsAccessRequestRequest<T> {
    pub fn new(id: T, user_id: i32) -> Self {
        Self {
            id,
            user_id,
            ..Default::default()
        }
    }
}

