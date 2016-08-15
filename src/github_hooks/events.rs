use std::str::FromStr;
use std::fmt;

#[derive(Debug, Hash, PartialEq, Clone)]
pub enum Events {
    CommitComment,
    Create,
    Delete,
    Deployment,
    DeploymentStatus,
    Download,
    Follow,
    Fork,
    ForkApply,
    Gist,
    Gollum,
    IssueComment,
    Issues,
    Member,
    Membership,
    PageBuild,
    Public,
    PullRequest,
    PullRequestReviewComment,
    Push,
    Release,
    Repository,
    Status,
    TeamAdd,
    Watch
} 

impl fmt::Display for Events {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Events {
    type Err = ();
    fn from_str(s: &str) -> Result<Events, ()> {
        match s {
            "commit_comment" => Ok(Events::CommitComment),
            "create" => Ok(Events::Create),
            "delete" => Ok(Events::Delete),
            "deployment" => Ok(Events::Deployment),
            "deployment_status" => Ok(Events::DeploymentStatus),
            "download" => Ok(Events::Download),
            "follow" => Ok(Events::Follow),
            "fork" => Ok(Events::Fork),
            "fork_apply" => Ok(Events::ForkApply),
            "gist" => Ok(Events::Gist),
            "gollum" => Ok(Events::Gollum),
            "issue_comment" => Ok(Events::IssueComment),
            "issues" => Ok(Events::Issues),
            "member" => Ok(Events::Member),
            "membership" => Ok(Events::Membership),
            "page_build" => Ok(Events::PageBuild),
            "public" => Ok(Events::Public),
            "pull_request" => Ok(Events::PullRequest),
            "pull_request_review_comment" => Ok(Events::PullRequestReviewComment),
            "push" => Ok(Events::Push),
            "release" => Ok(Events::Release),
            "repository" => Ok(Events::Repository),
            "status" => Ok(Events::Status),
            "team_add" => Ok(Events::TeamAdd),
            "watch" => Ok(Events::Watch),
            _ => Err(())
        }
    }
}


#[test]
fn test_fromstr_sane() {
    assert!("pull_request".parse::<Events>().is_ok());
    assert_eq!(Events::PullRequest, "pull_request".parse::<Events>().unwrap());
    assert_eq!(Events::PullRequestReviewComment, "pull_request_review_comment".parse::<Events>().unwrap());
    assert_eq!(Events::Watch, "watch".parse::<Events>().unwrap());
    assert!("fdsfds".parse::<Events>().is_err());
}


