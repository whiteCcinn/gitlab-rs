pub mod gitlab;
pub mod restful;
pub mod http_client;
pub mod common_resources;
pub mod access_requests;


#[cfg(test)]
mod tests {
    use crate::access_requests::project::ListAccessRequests;
    use crate::gitlab::Client;
    use crate::common_resources::payload::{ErrorMessage};
    use crate::gitlab::EndPointTrait;

    #[test]
    fn test_list_groups_access_request_request() {
        let instance = ListAccessRequests::new("456");

        let expected_endpoint = "/projects/456/access_requests";
        assert_eq!(instance.get_endpoint(), expected_endpoint);

        let expected_fields: Vec<&str> = vec![];
        assert_eq!(expected_fields, instance.get_query_fields());
    }


//     #[test]
//     fn it_works() -> Result<(), Box<dyn std::error::Error>> {
//         use serde::{Serialize, Deserialize};
//
//         #[derive(Serialize, Deserialize, Debug)]
//         struct Point {
//             x: i32,
//             y: Option<i32>,
//         }
//
//         let data = r#"
//            {
//     "id":303,
//     "description":"发布系统",
//     "name":"publish",
//     "name_with_namespace":"mc-webapp / publish",
//     "path":"publish",
//     "path_with_namespace":"mc-webapp/publish",
//     "created_at":"2021-03-15T11:07:52.967+08:00",
//     "default_branch":"develop",
//     "tag_list":[
//
//     ],
//     "ssh_url_to_repo":"ssh://git@gitlab.mingchao.com:62222/mc-webapp/publish.git",
//     "http_url_to_repo":"https://gitlab.mingchao.com/mc-webapp/publish.git",
//     "web_url":"https://gitlab.mingchao.com/mc-webapp/publish",
//     "readme_url":"https://gitlab.mingchao.com/mc-webapp/publish/-/blob/develop/README.md",
//     "avatar_url":null,
//     "forks_count":0,
//     "star_count":0,
//     "last_activity_at":"2021-03-15T11:07:52.967+08:00",
//     "namespace":{
//         "id":118,
//         "name":"mc-webapp",
//         "path":"mc-webapp",
//         "kind":"group",
//         "full_path":"mc-webapp",
//         "parent_id":null,
//         "avatar_url":null,
//         "web_url":"https://gitlab.mingchao.com/groups/mc-webapp"
//     },
//     "_links":{
//         "self":"https://gitlab.mingchao.com/api/v4/projects/303",
//         "issues":"https://gitlab.mingchao.com/api/v4/projects/303/issues",
//         "merge_requests":"https://gitlab.mingchao.com/api/v4/projects/303/merge_requests",
//         "repo_branches":"https://gitlab.mingchao.com/api/v4/projects/303/repository/branches",
//         "labels":"https://gitlab.mingchao.com/api/v4/projects/303/labels",
//         "events":"https://gitlab.mingchao.com/api/v4/projects/303/events",
//         "members":"https://gitlab.mingchao.com/api/v4/projects/303/members"
//     },
//     "packages_enabled":true,
//     "empty_repo":false,
//     "archived":false,
//     "visibility":"private",
//     "resolve_outdated_diff_discussions":false,
//     "container_registry_enabled":true,
//     "container_expiration_policy":{
//         "cadence":"1d",
//         "enabled":false,
//         "keep_n":10,
//         "older_than":"90d",
//         "name_regex":".*",
//         "name_regex_keep":null,
//         "next_run_at":"2021-03-16T11:07:53.073+08:00"
//     },
//     "issues_enabled":true,
//     "merge_requests_enabled":true,
//     "wiki_enabled":true,
//     "jobs_enabled":true,
//     "snippets_enabled":true,
//     "service_desk_enabled":false,
//     "service_desk_address":null,
//     "can_create_merge_request_in":true,
//     "issues_access_level":"enabled",
//     "repository_access_level":"enabled",
//     "merge_requests_access_level":"enabled",
//     "forking_access_level":"enabled",
//     "wiki_access_level":"enabled",
//     "builds_access_level":"enabled",
//     "snippets_access_level":"enabled",
//     "pages_access_level":"private",
//     "operations_access_level":"enabled",
//     "analytics_access_level":"enabled",
//     "emails_disabled":null,
//     "shared_runners_enabled":false,
//     "lfs_enabled":true,
//     "creator_id":38,
//     "import_status":"none",
//     "open_issues_count":0,
//     "ci_default_git_depth":50,
//     "ci_forward_deployment_enabled":true,
//     "public_jobs":true,
//     "build_timeout":3600,
//     "auto_cancel_pending_pipelines":"enabled",
//     "build_coverage_regex":null,
//     "ci_config_path":"",
//     "shared_with_groups":[
//
//     ],
//     "only_allow_merge_if_pipeline_succeeds":false,
//     "allow_merge_on_skipped_pipeline":null,
//     "request_access_enabled":true,
//     "only_allow_merge_if_all_discussions_are_resolved":false,
//     "remove_source_branch_after_merge":true,
//     "printing_merge_request_link_enabled":true,
//     "merge_method":"merge",
//     "suggestion_commit_message":null,
//     "auto_devops_enabled":false,
//     "auto_devops_deploy_strategy":"continuous",
//     "autoclose_referenced_issues":true,
//     "permissions":{
//         "project_access":null,
//         "group_access":{
//             "access_level":40,
//             "notification_level":3
//         }
//     }
// }
//         "#;
//
//         println!("{}", data);
//
//         // Convert the JSON string back to a Point.
//         let deserialized: ListsProjectsResponse = serde_json::from_str(data).unwrap();
//
//         // Prints deserialized = Point { x: 1, y: 2 }
//         println!("deserialized = {:#?}", deserialized);
//
//         // println!("GET https://www.rust-lang.org");
//         //
//         // let mut res = reqwest::blocking::get("https://www.rust-lang.org/")?;
//         // // let mut res = reqwest::Client::builder().build().unwrap().get()
//         // println!("Status: {}", res.status());
//         // println!("Headers:\n{:?}", res.headers());
//         //
//         // // copy the response body directly to stdout
//         // res.copy_to(&mut std::io::stdout())?;
//         //
//         // println!("\n\nDone.");
//         Ok(())
//     }
//
//     #[test]
//     fn it_works2() -> Result<(), Box<dyn std::error::Error>> {
//         println!("{:?}", AccessLevel::Maintainer);
//         Ok(())
//     }
//
//     #[test]
//     fn list_projects() -> Result<(), Box<dyn std::error::Error>> {
//         // let mut client = Client::new("3MsxoW7-iEW6CyPzsqjR".to_string());
//         let mut client = Client::new("zFabz1E4tGc8HvUmo_26".to_string());
//         // [{"id":6,"description":"","name":"Wwww","name_with_namespace":"group1_public / Wwww","path":"wwww","path_with_namespace":"group1_public/wwww","created_at":"2021-03-15T02:09:33.678Z","default_branch":"master","tag_list":[],"ssh_url_to_repo":"git@gitlab.ccinn.com:group1_public/wwww.git","http_url_to_repo":"http://gitlab.ccinn.com/group1_public/wwww.git","web_url":"http://gitlab.ccinn.com/group1_public/wwww","readme_url":"http://gitlab.ccinn.com/group1_public/wwww/-/blob/master/README.md","avatar_url":null,"forks_count":0,"star_count":0,"last_activity_at":"2021-03-15T02:09:33.678Z","namespace":{"id":4,"name":"group1_public","path":"group1_public","kind":"group","full_path":"group1_public","parent_id":null,"avatar_url":null,"web_url":"http://gitlab.ccinn.com/groups/group1_public"},"_links":{"self":"http://gitlab.ccinn.com/api/v4/projects/6","issues":"http://gitlab.ccinn.com/api/v4/projects/6/issues","merge_requests":"http://gitlab.ccinn.com/api/v4/projects/6/merge_requests","repo_branches":"http://gitlab.ccinn.com/api/v4/projects/6/repository/branches","labels":"http://gitlab.ccinn.com/api/v4/projects/6/labels","events":"http://gitlab.ccinn.com/api/v4/projects/6/events","members":"http://gitlab.ccinn.com/api/v4/projects/6/members"},"packages_enabled":true,"empty_repo":false,"archived":false,"visibility":"private","resolve_outdated_diff_discussions":false,"container_registry_enabled":true,"container_expiration_policy":{"cadence":"1d","enabled":false,"keep_n":10,"older_than":"90d","name_regex":".*","name_regex_keep":null,"next_run_at":"2021-03-16T02:09:33.699Z"},"issues_enabled":true,"merge_requests_enabled":true,"wiki_enabled":true,"jobs_enabled":true,"snippets_enabled":true,"service_desk_enabled":false,"service_desk_address":null,"can_create_merge_request_in":true,"issues_access_level":"enabled","repository_access_level":"enabled","merge_requests_access_level":"enabled","forking_access_level":"enabled","wiki_access_level":"enabled","builds_access_level":"enabled","snippets_access_level":"enabled","pages_access_level":"private","operations_access_level":"enabled","analytics_access_level":"enabled","emails_disabled":null,"shared_runners_enabled":true,"lfs_enabled":true,"creator_id":1,"import_status":"none","open_issues_count":0,"ci_default_git_depth":50,"ci_forward_deployment_enabled":true,"public_jobs":true,"build_timeout":3600,"auto_cancel_pending_pipelines":"enabled","build_coverage_regex":null,"ci_config_path":null,"shared_with_groups":[],"only_allow_merge_if_pipeline_succeeds":false,"allow_merge_on_skipped_pipeline":null,"restrict_user_defined_variables":false,"request_access_enabled":true,"only_allow_merge_if_all_discussions_are_resolved":false,"remove_source_branch_after_merge":true,"printing_merge_request_link_enabled":true,"merge_method":"merge","suggestion_commit_message":null,"auto_devops_enabled":true,"auto_devops_deploy_strategy":"continuous","autoclose_referenced_issues":true,"repository_storage":"default","requirements_enabled":null,"security_and_compliance_enabled":null,"compliance_frameworks":[],"permissions":{"project_access":null,"group_access":{"access_level":50,"notification_level":3}}},{"id":5,"description":"","name":"Dadsdasd","name_with_namespace":"group1_public / Dadsdasd","path":"dadsdasd","path_with_namespace":"group1_public/dadsdasd","created_at":"2021-03-15T02:09:05.222Z","default_branch":"master","tag_list":[],"ssh_url_to_repo":"git@gitlab.ccinn.com:group1_public/dadsdasd.git","http_url_to_repo":"http://gitlab.ccinn.com/group1_public/dadsdasd.git","web_url":"http://gitlab.ccinn.com/group1_public/dadsdasd","readme_url":"http://gitlab.ccinn.com/group1_public/dadsdasd/-/blob/master/README.md","avatar_url":null,"forks_count":0,"star_count":0,"last_activity_at":"2021-03-15T02:09:05.222Z","namespace":{"id":4,"name":"group1_public","path":"group1_public","kind":"group","full_path":"group1_public","parent_id":null,"avatar_url":null,"web_url":"http://gitlab.ccinn.com/groups/group1_public"},"_links":{"self":"http://gitlab.ccinn.com/api/v4/projects/5","issues":"http://gitlab.ccinn.com/api/v4/projects/5/issues","merge_requests":"http://gitlab.ccinn.com/api/v4/projects/5/merge_requests","repo_branches":"http://gitlab.ccinn.com/api/v4/projects/5/repository/branches","labels":"http://gitlab.ccinn.com/api/v4/projects/5/labels","events":"http://gitlab.ccinn.com/api/v4/projects/5/events","members":"http://gitlab.ccinn.com/api/v4/projects/5/members"},"packages_enabled":true,"empty_repo":false,"archived":false,"visibility":"public","resolve_outdated_diff_discussions":false,"container_registry_enabled":true,"container_expiration_policy":{"cadence":"1d","enabled":false,"keep_n":10,"older_than":"90d","name_regex":".*","name_regex_keep":null,"next_run_at":"2021-03-16T02:09:05.322Z"},"issues_enabled":true,"merge_requests_enabled":true,"wiki_enabled":true,"jobs_enabled":true,"snippets_enabled":true,"service_desk_enabled":false,"service_desk_address":null,"can_create_merge_request_in":true,"issues_access_level":"enabled","repository_access_level":"enabled","merge_requests_access_level":"enabled","forking_access_level":"enabled","wiki_access_level":"enabled","builds_access_level":"enabled","snippets_access_level":"enabled","pages_access_level":"enabled","operations_access_level":"enabled","analytics_access_level":"enabled","emails_disabled":null,"shared_runners_enabled":true,"lfs_enabled":true,"creator_id":1,"import_status":"none","open_issues_count":0,"ci_default_git_depth":50,"ci_forward_deployment_enabled":true,"public_jobs":true,"build_timeout":3600,"auto_cancel_pending_pipelines":"enabled","build_coverage_regex":null,"ci_config_path":null,"shared_with_groups":[],"only_allow_merge_if_pipeline_succeeds":false,"allow_merge_on_skipped_pipeline":null,"restrict_user_defined_variables":false,"request_access_enabled":true,"only_allow_merge_if_all_discussions_are_resolved":false,"remove_source_branch_after_merge":true,"printing_merge_request_link_enabled":true,"merge_method":"merge","suggestion_commit_message":null,"auto_devops_enabled":true,"auto_devops_deploy_strategy":"continuous","autoclose_referenced_issues":true,"repository_storage":"default","requirements_enabled":null,"security_and_compliance_enabled":null,"compliance_frameworks":[],"permissions":{"project_access":null,"group_access":{"access_level":50,"notification_level":3}}},{"id":4,"description":"","name":"11111","name_with_namespace":"group1_private / 11111","path":"11111","path_with_namespace":"group1_private/11111","created_at":"2021-03-15T02:08:03.122Z","default_branch":"master","tag_list":[],"ssh_url_to_repo":"git@gitlab.ccinn.com:group1_private/11111.git","http_url_to_repo":"http://gitlab.ccinn.com/group1_private/11111.git","web_url":"http://gitlab.ccinn.com/group1_private/11111","readme_url":"http://gitlab.ccinn.com/group1_private/11111/-/blob/master/README.md","avatar_url":null,"forks_count":0,"star_count":0,"last_activity_at":"2021-03-15T02:08:03.122Z","namespace":{"id":3,"name":"group1_private","path":"group1_private","kind":"group","full_path":"group1_private","parent_id":null,"avatar_url":null,"web_url":"http://gitlab.ccinn.com/groups/group1_private"},"_links":{"self":"http://gitlab.ccinn.com/api/v4/projects/4","issues":"http://gitlab.ccinn.com/api/v4/projects/4/issues","merge_requests":"http://gitlab.ccinn.com/api/v4/projects/4/merge_requests","repo_branches":"http://gitlab.ccinn.com/api/v4/projects/4/repository/branches","labels":"http://gitlab.ccinn.com/api/v4/projects/4/labels","events":"http://gitlab.ccinn.com/api/v4/projects/4/events","members":"http://gitlab.ccinn.com/api/v4/projects/4/members"},"packages_enabled":true,"empty_repo":false,"archived":false,"visibility":"private","resolve_outdated_diff_discussions":false,"container_registry_enabled":true,"container_expiration_policy":{"cadence":"1d","enabled":false,"keep_n":10,"older_than":"90d","name_regex":".*","name_regex_keep":null,"next_run_at":"2021-03-16T02:08:03.231Z"},"issues_enabled":true,"merge_requests_enabled":true,"wiki_enabled":true,"jobs_enabled":true,"snippets_enabled":true,"service_desk_enabled":false,"service_desk_address":null,"can_create_merge_request_in":true,"issues_access_level":"enabled","repository_access_level":"enabled","merge_requests_access_level":"enabled","forking_access_level":"enabled","wiki_access_level":"enabled","builds_access_level":"enabled","snippets_access_level":"enabled","pages_access_level":"private","operations_access_level":"enabled","analytics_access_level":"enabled","emails_disabled":null,"shared_runners_enabled":true,"lfs_enabled":true,"creator_id":1,"import_status":"none","open_issues_count":0,"ci_default_git_depth":50,"ci_forward_deployment_enabled":true,"public_jobs":true,"build_timeout":3600,"auto_cancel_pending_pipelines":"enabled","build_coverage_regex":null,"ci_config_path":null,"shared_with_groups":[],"only_allow_merge_if_pipeline_succeeds":false,"allow_merge_on_skipped_pipeline":null,"restrict_user_defined_variables":false,"request_access_enabled":true,"only_allow_merge_if_all_discussions_are_resolved":false,"remove_source_branch_after_merge":true,"printing_merge_request_link_enabled":true,"merge_method":"merge","suggestion_commit_message":null,"auto_devops_enabled":true,"auto_devops_deploy_strategy":"continuous","autoclose_referenced_issues":true,"repository_storage":"default","requirements_enabled":null,"security_and_compliance_enabled":null,"compliance_frameworks":[],"permissions":{"project_access":null,"group_access":{"access_level":50,"notification_level":3}}},{"id":3,"description":"","name":"test_public","name_with_namespace":"Administrator / test_public","path":"test_public","path_with_namespace":"root/test_public","created_at":"2021-03-15T02:07:18.666Z","default_branch":"master","tag_list":[],"ssh_url_to_repo":"git@gitlab.ccinn.com:root/test_public.git","http_url_to_repo":"http://gitlab.ccinn.com/root/test_public.git","web_url":"http://gitlab.ccinn.com/root/test_public","readme_url":"http://gitlab.ccinn.com/root/test_public/-/blob/master/README.md","avatar_url":null,"forks_count":0,"star_count":0,"last_activity_at":"2021-03-15T02:07:18.666Z","namespace":{"id":1,"name":"Administrator","path":"root","kind":"user","full_path":"root","parent_id":null,"avatar_url":"https://www.gravatar.com/avatar/e64c7d89f26bd1972efa854d13d7dd61?s=80\u0026d=identicon","web_url":"http://gitlab.ccinn.com/root"},"_links":{"self":"http://gitlab.ccinn.com/api/v4/projects/3","issues":"http://gitlab.ccinn.com/api/v4/projects/3/issues","merge_requests":"http://gitlab.ccinn.com/api/v4/projects/3/merge_requests","repo_branches":"http://gitlab.ccinn.com/api/v4/projects/3/repository/branches","labels":"http://gitlab.ccinn.com/api/v4/projects/3/labels","events":"http://gitlab.ccinn.com/api/v4/projects/3/events","members":"http://gitlab.ccinn.com/api/v4/projects/3/members"},"packages_enabled":true,"empty_repo":false,"archived":false,"visibility":"public","owner":{"id":1,"name":"Administrator","username":"root","state":"active","avatar_url":"https://www.gravatar.com/avatar/e64c7d89f26bd1972efa854d13d7dd61?s=80\u0026d=identicon","web_url":"http://gitlab.ccinn.com/root"},"resolve_outdated_diff_discussions":false,"container_registry_enabled":true,"container_expiration_policy":{"cadence":"1d","enabled":false,"keep_n":10,"older_than":"90d","name_regex":".*","name_regex_keep":null,"next_run_at":"2021-03-16T02:07:18.797Z"},"issues_enabled":true,"merge_requests_enabled":true,"wiki_enabled":true,"jobs_enabled":true,"snippets_enabled":true,"service_desk_enabled":false,"service_desk_address":null,"can_create_merge_request_in":true,"issues_access_level":"enabled","repository_access_level":"enabled","merge_requests_access_level":"enabled","forking_access_level":"enabled","wiki_access_level":"enabled","builds_access_level":"enabled","snippets_access_level":"enabled","pages_access_level":"enabled","operations_access_level":"enabled","analytics_access_level":"enabled","emails_disabled":null,"shared_runners_enabled":true,"lfs_enabled":true,"creator_id":1,"import_status":"none","open_issues_count":0,"ci_default_git_depth":50,"ci_forward_deployment_enabled":true,"public_jobs":true,"build_timeout":3600,"auto_cancel_pending_pipelines":"enabled","build_coverage_regex":null,"ci_config_path":null,"shared_with_groups":[],"only_allow_merge_if_pipeline_succeeds":false,"allow_merge_on_skipped_pipeline":null,"restrict_user_defined_variables":false,"request_access_enabled":true,"only_allow_merge_if_all_discussions_are_resolved":false,"remove_source_branch_after_merge":true,"printing_merge_request_link_enabled":true,"merge_method":"merge","suggestion_commit_message":null,"auto_devops_enabled":true,"auto_devops_deploy_strategy":"continuous","autoclose_referenced_issues":true,"repository_storage":"default","requirements_enabled":null,"security_and_compliance_enabled":null,"compliance_frameworks":[],"permissions":{"project_access":{"access_level":40,"notification_level":3},"group_access":null}},{"id":2,"description":"","name":"test_private","name_with_namespace":"Administrator / test_private","path":"test_private","path_with_namespace":"root/test_private","created_at":"2021-03-15T02:06:33.217Z","default_branch":"master","tag_list":[],"ssh_url_to_repo":"git@gitlab.ccinn.com:root/test_private.git","http_url_to_repo":"http://gitlab.ccinn.com/root/test_private.git","web_url":"http://gitlab.ccinn.com/root/test_private","readme_url":"http://gitlab.ccinn.com/root/test_private/-/blob/master/README.md","avatar_url":null,"forks_count":0,"star_count":0,"last_activity_at":"2021-03-15T02:06:33.217Z","namespace":{"id":1,"name":"Administrator","path":"root","kind":"user","full_path":"root","parent_id":null,"avatar_url":"https://www.gravatar.com/avatar/e64c7d89f26bd1972efa854d13d7dd61?s=80\u0026d=identicon","web_url":"http://gitlab.ccinn.com/root"},"_links":{"self":"http://gitlab.ccinn.com/api/v4/projects/2","issues":"http://gitlab.ccinn.com/api/v4/projects/2/issues","merge_requests":"http://gitlab.ccinn.com/api/v4/projects/2/merge_requests","repo_branches":"http://gitlab.ccinn.com/api/v4/projects/2/repository/branches","labels":"http://gitlab.ccinn.com/api/v4/projects/2/labels","events":"http://gitlab.ccinn.com/api/v4/projects/2/events","members":"http://gitlab.ccinn.com/api/v4/projects/2/members"},"packages_enabled":true,"empty_repo":false,"archived":false,"visibility":"private","owner":{"id":1,"name":"Administrator","username":"root","state":"active","avatar_url":"https://www.gravatar.com/avatar/e64c7d89f26bd1972efa854d13d7dd61?s=80\u0026d=identicon","web_url":"http://gitlab.ccinn.com/root"},"resolve_outdated_diff_discussions":false,"container_registry_enabled":true,"container_expiration_policy":{"cadence":"1d","enabled":false,"keep_n":10,"older_than":"90d","name_regex":".*","name_regex_keep":null,"next_run_at":"2021-03-16T02:06:33.354Z"},"issues_enabled":true,"merge_requests_enabled":true,"wiki_enabled":true,"jobs_enabled":true,"snippets_enabled":true,"service_desk_enabled":false,"service_desk_address":null,"can_create_merge_request_in":true,"issues_access_level":"enabled","repository_access_level":"enabled","merge_requests_access_level":"enabled","forking_access_level":"enabled","wiki_access_level":"enabled","builds_access_level":"enabled","snippets_access_level":"enabled","pages_access_level":"private","operations_access_level":"enabled","analytics_access_level":"enabled","emails_disabled":null,"shared_runners_enabled":true,"lfs_enabled":true,"creator_id":1,"import_status":"none","open_issues_count":0,"ci_default_git_depth":50,"ci_forward_deployment_enabled":true,"public_jobs":true,"build_timeout":3600,"auto_cancel_pending_pipelines":"enabled","build_coverage_regex":null,"ci_config_path":null,"shared_with_groups":[],"only_allow_merge_if_pipeline_succeeds":false,"allow_merge_on_skipped_pipeline":null,"restrict_user_defined_variables":false,"request_access_enabled":true,"only_allow_merge_if_all_discussions_are_resolved":false,"remove_source_branch_after_merge":true,"printing_merge_request_link_enabled":true,"merge_method":"merge","suggestion_commit_message":null,"auto_devops_enabled":true,"auto_devops_deploy_strategy":"continuous","autoclose_referenced_issues":true,"repository_storage":"default","requirements_enabled":null,"security_and_compliance_enabled":null,"compliance_frameworks":[],"permissions":{"project_access":{"access_level":40,"notification_level":3},"group_access":null}},{"id":1,"description":"This project is automatically generated and helps monitor this GitLab instance. [Learn more](/help/administration/monitoring/gitlab_self_monitoring_project/index).","name":"Monitoring","name_with_namespace":"GitLab Instance / Monitoring","path":"Monitoring","path_with_namespace":"gitlab-instance-74e2ad6a/Monitoring","created_at":"2021-03-15T01:53:22.163Z","default_branch":null,"tag_list":[],"ssh_url_to_repo":"git@gitlab.ccinn.com:gitlab-instance-74e2ad6a/Monitoring.git","http_url_to_repo":"http://gitlab.ccinn.com/gitlab-instance-74e2ad6a/Monitoring.git","web_url":"http://gitlab.ccinn.com/gitlab-instance-74e2ad6a/Monitoring","readme_url":null,"avatar_url":null,"forks_count":0,"star_count":0,"last_activity_at":"2021-03-15T01:53:22.163Z","namespace":{"id":2,"name":"GitLab Instance","path":"gitlab-instance-74e2ad6a","kind":"group","full_path":"gitlab-instance-74e2ad6a","parent_id":null,"avatar_url":null,"web_url":"http://gitlab.ccinn.com/groups/gitlab-instance-74e2ad6a"},"_links":{"self":"http://gitlab.ccinn.com/api/v4/projects/1","issues":"http://gitlab.ccinn.com/api/v4/projects/1/issues","merge_requests":"http://gitlab.ccinn.com/api/v4/projects/1/merge_requests","repo_branches":"http://gitlab.ccinn.com/api/v4/projects/1/repository/branches","labels":"http://gitlab.ccinn.com/api/v4/projects/1/labels","events":"http://gitlab.ccinn.com/api/v4/projects/1/events","members":"http://gitlab.ccinn.com/api/v4/projects/1/members"},"packages_enabled":true,"empty_repo":true,"archived":false,"visibility":"internal","resolve_outdated_diff_discussions":false,"container_registry_enabled":true,"container_expiration_policy":{"cadence":"1d","enabled":false,"keep_n":10,"older_than":"90d","name_regex":".*","name_regex_keep":null,"next_run_at":"2021-03-16T01:53:22.440Z"},"issues_enabled":true,"merge_requests_enabled":true,"wiki_enabled":true,"jobs_enabled":true,"snippets_enabled":true,"service_desk_enabled":false,"service_desk_address":null,"can_create_merge_request_in":true,"issues_access_level":"enabled","repository_access_level":"enabled","merge_requests_access_level":"enabled","forking_access_level":"enabled","wiki_access_level":"enabled","builds_access_level":"enabled","snippets_access_level":"enabled","pages_access_level":"private","operations_access_level":"enabled","analytics_access_level":"enabled","emails_disabled":null,"shared_runners_enabled":true,"lfs_enabled":true,"creator_id":1,"import_status":"none","open_issues_count":0,"ci_default_git_depth":50,"ci_forward_deployment_enabled":true,"public_jobs":true,"build_timeout":3600,"auto_cancel_pending_pipelines":"enabled","build_coverage_regex":null,"ci_config_path":null,"shared_with_groups":[],"only_allow_merge_if_pipeline_succeeds":false,"allow_merge_on_skipped_pipeline":null,"restrict_user_defined_variables":false,"request_access_enabled":true,"only_allow_merge_if_all_discussions_are_resolved":false,"remove_source_branch_after_merge":true,"printing_merge_request_link_enabled":true,"merge_method":"merge","suggestion_commit_message":null,"auto_devops_enabled":true,"auto_devops_deploy_strategy":"continuous","autoclose_referenced_issues":true,"repository_storage":"default","requirements_enabled":null,"security_and_compliance_enabled":null,"compliance_frameworks":[],"permissions":{"project_access":null,"group_access":{"access_level":50,"notification_level":3}}}]
//         client.set_base_url("http://gitlab.ccinn.com/".to_string()).unwrap();
//
//         let json = client.request(gitla::ListsProjectsRequest::new()).text().unwrap();
//
//         println!("{}", json);
//         let rs: Vec<gitlab::ListsProjectsResponse>
//             = serde_json::from_str(json.as_str()).unwrap();
//         // let rs: serde_json::Value = client.http_client.get(endpoint_chain).header("PRIVATE-TOKEN", client.token).send().unwrap().json().unwrap();
//         println!("{:#?}", rs);
//
//         Ok(())
//     }
//
//     #[test]
//     fn list_groups() -> Result<(), Box<dyn std::error::Error>> {
//         let mut client = Client::new("3MsxoW7-iEW6CyPzsqjR".to_string());
//
//         client.set_base_url("https://gitlab.mingchao.com/".to_string()).unwrap();
//
//         // let rs: serde_json::Value = client.request(gitlab::ListsGroupsRequest::new()).json().unwrap();
//         let response = client.request(gitlab::ListsGroupsRequest::new());
//
//         if response.status().is_success() {
//             let json = response.text().unwrap();
//
//             println!("{}", json);
//             let rs: Vec<gitlab::ListsGroupsResponse>
//                 = serde_json::from_str(json.as_str()).unwrap();
//             // let rs: serde_json::Value = client.http_client.get(endpoint_chain).header("PRIVATE-TOKEN", client.token).send().unwrap().json().unwrap();
//             println!("{:#?}", rs);
//         } else if response.status().is_client_error() {
//             let json = response.text().unwrap();
//
//             println!("{}", json);
//             let rs: ErrorMessage
//                 = serde_json::from_str(json.as_str()).unwrap();
//             // let rs: serde_json::Value = client.http_client.get(endpoint_chain).header("PRIVATE-TOKEN", client.token).send().unwrap().json().unwrap();
//             println!("{:#?}", rs);
//         }
//
//         Ok(())
//     }
//
    #[test]
    fn request_group_access_request() -> Result<(), Box<dyn std::error::Error>> {
        // let mut client = Client::new("3MsxoW7-iEW6CyPzsqjR".to_string());
        let mut client = Client::new("zFabz1E4tGc8HvUmo_26".to_string());
        client.set_base_url("http://gitlab.ccinn.com/".to_string()).unwrap();

        let response = client.request(ListAccessRequests::new("2"));

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
            // let rs: serde_json::Value = client.http_client.get(endpoint_chain).header("PRIVATE-TOKEN", client.token).send().unwrap().json().unwrap();
            println!("{:#?}", rs);
        }

        Ok(())
    }
//
//     #[test]
//     fn request_project_access_request() -> Result<(), Box<dyn std::error::Error>> {
//         // let mut client = Client::new("3MsxoW7-iEW6CyPzsqjR".to_string());
//         let mut client = Client::new("zFabz1E4tGc8HvUmo_26".to_string());
//         client.set_base_url("http://gitlab.ccinn.com/".to_string()).unwrap();
//
//         let response = client.request(RequestProjectsAccessRequestRequest::new(2));
//
//         if response.status().is_success() {
//             let json = response.text().unwrap();
//
//             println!("{}", json);
//             let rs: Vec<ProjectsAccessRequestResponse> = serde_json::from_str(json.as_str()).unwrap();
//             println!("{:#?}", rs);
//         } else if response.status().is_client_error() {
//             let json = response.text().unwrap();
//
//             println!("{}", json);
//             let rs: ErrorMessage
//                 = serde_json::from_str(json.as_str()).unwrap();
//             // let rs: serde_json::Value = client.http_client.get(endpoint_chain).header("PRIVATE-TOKEN", client.token).send().unwrap().json().unwrap();
//             println!("{:#?}", rs);
//         }
//
//         Ok(())
//     }
//
//     #[test]
//     fn approve_project_access_request() -> Result<(), Box<dyn std::error::Error>> {
//         // let mut client = Client::new("3MsxoW7-iEW6CyPzsqjR".to_string());
//         let mut client = Client::new("zFabz1E4tGc8HvUmo_26".to_string());
//         client.set_base_url("http://gitlab.ccinn.com/".to_string()).unwrap();
//
//         let response = client.request(ApproveGroupsAccessRequestRequest::new(2, 1));
//
//         if response.status().is_success() {
//             let json = response.text().unwrap();
//
//             println!("{}", json);
//             let rs: ApproveAccessRequestResponse = serde_json::from_str(json.as_str()).unwrap();
//             println!("{:#?}", rs);
//         } else if response.status().is_client_error() {
//             let json = response.text().unwrap();
//
//             println!("{}", json);
//             let rs: ErrorMessage
//                 = serde_json::from_str(json.as_str()).unwrap();
//             // let rs: serde_json::Value = client.http_client.get(endpoint_chain).header("PRIVATE-TOKEN", client.token).send().unwrap().json().unwrap();
//             println!("{:#?}", rs);
//         }
//
//         Ok(())
//     }

    // #[test]
    // fn it_works3() -> Result<(), Box<dyn std::error::Error>> {
    //     // let mut client = Client::new_basic_auth_client("caiwenhui".to_string(), "casiwenhui123".to_string());
    //     let mut client = Client::new("3MsxoW7-iEW6CyPzsqjR".to_string());
    //     // println!("{:?}", client);
    //
    //     client.set_base_url("https://gitlab.mingchao.com/".to_string()).unwrap();
    //
    //     println!("{:?}", client);
    //
    //     let (r#type, endpoint) = ListsProjectsRequest::new("131").get_endpoint();
    //     // let (r#type, endpoint) = ListProjectAccessRequestRequest::new(2).get_endpoint();
    //
    //     println!("{:?}", endpoint);
    //     println!("{:?}", r#type);
    //
    //     let endpoint_chain = client.get_endpoint_url(endpoint).unwrap();
    //
    //     println!("{:?}", endpoint_chain);
    //     let rs: serde_json::Value = client.http_client.get(endpoint_chain).header("PRIVATE-TOKEN", client.token).send().unwrap().json().unwrap();
    //     println!("{}", rs);
    //     Ok(())
    // }
}