
include!(concat!(env!("OUT_DIR"), "/user.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    #[test]
    fn test_user_parsing() {
        let user_json =  r##"{"login": "baxterthehacker",
            "id": 6752317,
            "avatar_url": "https://avatars.githubusercontent.com/u/6752317?v=3",
            "gravatar_id": "",
            "url": "https://api.github.com/users/baxterthehacker",
            "html_url": "https://github.com/baxterthehacker",
            "followers_url": "https://api.github.com/users/baxterthehacker/followers",
            "following_url": "https://api.github.com/users/baxterthehacker/following{/other_user}",
            "gists_url": "https://api.github.com/users/baxterthehacker/gists{/gist_id}",
            "starred_url": "https://api.github.com/users/baxterthehacker/starred{/owner}{/repo}",
            "subscriptions_url": "https://api.github.com/users/baxterthehacker/subscriptions",
            "organizations_url": "https://api.github.com/users/baxterthehacker/orgs",
            "repos_url": "https://api.github.com/users/baxterthehacker/repos",
            "events_url": "https://api.github.com/users/baxterthehacker/events{/privacy}",
            "received_events_url": "https://api.github.com/users/baxterthehacker/received_events",
            "type": "User",
            "site_admin": false,
            "name": "octocat"
        }"##;
        let d: UserData = serde_json::from_str(user_json).unwrap();
        assert_eq!("User", d.user_type);
    }
}


