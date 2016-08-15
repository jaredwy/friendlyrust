#[macro_use] extern crate hyper;
extern crate serde;
extern crate serde_json;

mod github_hooks;

use std::io::Read;
use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;
use hyper::uri::RequestUri::AbsolutePath;
use hyper::{Post};
use std::str;
use github_hooks::events::Events;



header! { (XGitHubEvent, "X-GitHub-Event") => [Events] }

fn post(mut req: Request, mut res: Response) {
    match req.uri.to_owned() {
        AbsolutePath(ref path) => match path.as_str() {
            "/payload" => {
                if let Some(&XGitHubEvent(ref e)) = req.headers.to_owned().get::<XGitHubEvent>() { 
                    let mut payload = String::new();
                    let result = req.read_to_string(&mut payload);
                    let payload = payload;
                    match result {
                        Ok(_) => github_hooks::handle_payload(e, payload),
                        Err(_) =>  *res.status_mut() =  StatusCode::BadRequest
                    }
                    return;
                } else {
                     *res.status_mut() =  StatusCode::BadRequest;
                }
            },
            _ => *res.status_mut() =  StatusCode::NotFound
        },
        _ => *res.status_mut() =  StatusCode::NotFound
    }
}

fn handle_method(req: Request, mut res: Response) {
    match req.method {
        Post => post(req, res),
        _ => *res.status_mut() = StatusCode::MethodNotAllowed
    }  
}

fn main() {

    println!("{:?}", "pull_request".parse::<Events>());
    Server::http("0.0.0.0:8082").unwrap().handle(handle_method).unwrap();
}
