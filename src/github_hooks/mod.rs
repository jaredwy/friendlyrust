use std::str;
use std::env;

pub mod events;
pub mod github_data;
use serde_json;

use hyper::Client;
use hyper::header::{Headers, ContentType, Authorization, Bearer, UserAgent};
use hyper::status::StatusCode::Ok;
use hyper::mime::{Mime, TopLevel, SubLevel};
use std::io::Read;
pub use self::events::Events::*;
use self::github_data::pullrequest::PullRequestData as PR;


pub type GithubUsername = str;

pub fn handle_payload(e: &self::events::Events, body: String) {
    println!("{:?}", e);
    let d:PR = serde_json::from_str(&body[..]).unwrap();
    println!("{:?}",  d.pull_request.body);
    if let Some(requested_reviewer) = get_review_requested(d.pull_request.body) {
        //let client = Client::with_http_proxy("localhost", 8888);
        let client = Client::new();
        let mut headers = Headers::new();
        headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![])));
       

        headers.set(Authorization(Bearer{token: env::var("AuthToken").unwrap_or("".to_string())}));
        headers.set(UserAgent("friendlyrust".to_owned()));
        let res = client.patch(&d.pull_request.issue_url)
                .body(&get_assign_json(requested_reviewer))
                .headers(headers)
                .send()
                .unwrap();
        assert_eq!(res.status, Ok);
    }
}

//TODO: technically this is a slice.
//So lets figure out a way to return &str at some point
pub fn get_review_requested(body: String) -> Option<String> {
    //keep this as a if let as i want it to be able to handle multiple r? requests
    //so a while let is easy-Pa$$@012

    // r? blah 
    // r? blah2
    if let Some(x) = body.find("r?") {
        let from_request = &body[x+2..].chars()
                                    .skip_while(|y| y.is_whitespace() || y == &'@')
                                    .take_while(|y| y.is_alphanumeric())
                                    .collect::<String>();

        if from_request.len() > 0 {
            return Some(from_request.to_owned());
        }
    } 
    None
}


pub fn get_assign_json(name: String) -> String {
    format!("{{\"assignees\":[\"{}\"]}}", name)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_match_review_request() {
        assert!(get_review_requested(&String::from("there is no request here!")).is_none());
        assert_eq!(Some("jared".to_string()), get_review_requested(&String::from("r?jared")));
        assert_eq!(Some("jared".to_string()), get_review_requested(&String::from("r? jared")));
        assert_eq!(Some("jared".to_string()), get_review_requested(&String::from("r?@jared")));
        assert_eq!(Some("jared".to_string()), get_review_requested(&String::from("r? @jared")));
        assert_eq!(Some("jared".to_string()), get_review_requested(&String::from("r? @jared is awesome")));
        assert!(get_review_requested(&String::from("r? ")).is_none());
    }
}
