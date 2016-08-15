
use std::str::FromStr;


include!(concat!(env!("OUT_DIR"), "/pullrequest.rs"));

impl FromStr for EventAction {
    type Err = ();
    fn from_str(s: &str) -> Result<EventAction, ()> {
        match s {
            "assigned" => Ok(EventAction::Assigned),
            "unassigned" => Ok(EventAction::Unassigned),
            "labeled" => Ok(EventAction::Labeled),
            "unlabeled" => Ok(EventAction::Unlabeled),
            "opened" => Ok(EventAction::Opened),
            "edited" => Ok(EventAction::Edited),
            "closed" => Ok(EventAction::Closed),
            "reopened" => Ok(EventAction::Reopened),
            "synchronize" => Ok(EventAction::Synchronize),
            _ => Err(())
        }
    }
}





// pub struct JsonPullRequestUser {
//     pub login: String,
//     pub id: i64,
//     pub avatar_url: String,
//     pub gravatar_id: String,
//     pub url: String,
//     pub html_url: String,
//     pub followers_url: String,
//     pub following_url: String,
//     pub gists_url: String,
//     pub starred_url: String,
//     pub subscriptions_url: String,
//     pub organizations_url: String,
//     pub repos_url: String,
//     pub events_url: String,
//     pub received_events_url: String,
//     pub type: String,
//     pub site_admin: bool,
// }

// pub struct JsonPullRequestHeadRepo {
//     pub id: i64,
//     pub name: String,
//     pub full_name: String,
//     pub owner: JsonPullRequestUser,
//     pub private: bool,
//     pub html_url: String,
//     pub description: String,
//     pub fork: bool,
//     pub url: String,
//     pub forks_url: String,
//     pub keys_url: String,
//     pub collaborators_url: String,
//     pub teams_url: String,
//     pub hooks_url: String,
//     pub issue_events_url: String,
//     pub events_url: String,
//     pub assignees_url: String,
//     pub branches_url: String,
//     pub tags_url: String,
//     pub blobs_url: String,
//     pub git_tags_url: String,
//     pub git_refs_url: String,
//     pub trees_url: String,
//     pub statuses_url: String,
//     pub languages_url: String,
//     pub stargazers_url: String,
//     pub contributors_url: String,
//     pub subscribers_url: String,
//     pub subscription_url: String,
//     pub commits_url: String,
//     pub git_commits_url: String,
//     pub comments_url: String,
//     pub issue_comment_url: String,
//     pub contents_url: String,
//     pub compare_url: String,
//     pub merges_url: String,
//     pub archive_url: String,
//     pub downloads_url: String,
//     pub issues_url: String,
//     pub pulls_url: String,
//     pub milestones_url: String,
//     pub notifications_url: String,
//     pub labels_url: String,
//     pub releases_url: String,
//     pub created_at: String,
//     pub updated_at: String,
//     pub pushed_at: String,
//     pub git_url: String,
//     pub ssh_url: String,
//     pub clone_url: String,
//     pub svn_url: String,
//     pub homepage: Option<?>,
//     pub size: i64,
//     pub stargazers_count: i64,
//     pub watchers_count: i64,
//     pub language: Option<?>,
//     pub has_issues: bool,
//     pub has_downloads: bool,
//     pub has_wiki: bool,
//     pub has_pages: bool,
//     pub forks_count: i64,
//     pub mirror_url: Option<?>,
//     pub open_issues_count: i64,
//     pub forks: i64,
//     pub open_issues: i64,
//     pub watchers: i64,
//     pub default_branch: String,
// }

// pub struct JsonPullRequestHead {
//     pub label: String,
//     pub ref: String,
//     pub sha: String,
//     pub user: JsonPullRequestUser,
//     pub repo: JsonPullRequestHeadRepo,
// }

// pub struct JsonPullRequestLinksSelf {
//     pub href: String,
// }

// pub struct JsonPullRequestLink {
//     pub self: JsonPullRequestLinksSelf,
//     pub html: JsonPullRequestLinksSelf,
//     pub issue: JsonPullRequestLinksSelf,
//     pub comments: JsonPullRequestLinksSelf,
//     pub review_comments: JsonPullRequestLinksSelf,
//     pub review_comment: JsonPullRequestLinksSelf,
//     pub commits: JsonPullRequestLinksSelf,
//     pub statuses: JsonPullRequestLinksSelf,
// }


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    #[test]
    fn test_pullrequest_parsing() {
     /*   let v = PullRequestData { action: EventAction::Closed, number: 4 };
        let s = serde_json::to_string(&v);
        assert!(s.is_ok());
        //has extra data
        let d: PullRequestData = serde_json::from_str({"{\"action\": \"opened\",\"noExist\":3, \"number\": 4}"}).unwrap();
        assert_eq!(EventAction::Opened, d.action);
        assert_eq!(4, d.number);
        */
    }
    
}

