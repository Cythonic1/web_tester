// processing : blue
// bad luck red
// good luck green
use crate::{
    Context,
    Models::{Desc, Scan},
};

use colored::*;
use reqwest::StatusCode;

use super::format_domain;
use super::check_target;

pub struct GitLeak;

impl GitLeak {
    fn is_git_dir_listing(body: &str) -> bool {
        body.contains("HEAD")
            && body.contains("refs")
            && body.contains("config")
            && body.contains("index")
            && body.contains("objects")
    }
}

impl Scan for GitLeak {
    fn init(&self) {
        todo!()
    }
    fn enumerate(&self, ctx: &Context) {
        // Check if any HTTP port is open and get the port


        let domain = check_target(ctx);
        // Should handle the case where there is a port
        // Form the URL based on the open port
        let search_git_file = format_domain(&domain);


        match ctx.client.get(search_git_file).send() {
            Ok(response) => match response.status() {
                StatusCode::OK => {
                    let url_res = response.url().to_string();
                    // Check if the URL ends with .git or contains .git (for redirection cases)
                    if url_res.ends_with("/.git/") || url_res.contains("/.git/") {
                        println!("{:#?}", response);
                        println!(
                            "{}",
                            "Target is vulnerable; it has .git exposed.".green().bold()
                        );
                        println!("{}", "Checking for directory listing...".blue().bold());
                        let body = response.text().expect("Cannot convert non-UTF-8 chars");
                        if GitLeak::is_git_dir_listing(&body) {
                            println!("{}", "Directory listing is enabled.".green().bold());
                        } else {
                            println!("{}", "Directory listing is not enabled.".red().bold());
                        }
                    } else {
                        println!("{}",
                            "The website returned 200, but .git is not in the URL, possibly due to redirection.".red().bold()
                        );
                    }
                }
                StatusCode::MOVED_PERMANENTLY => {
                    println!("The target might be vulnerable (301 Redirect).");
                }
                _ => {
                    println!("Target is not vulnerable.");
                }
            },
            Err(e) => {
                println!("Failed to send the request: {}", e);
            }
        }
    }
}

impl Desc for GitLeak {
    fn name(&self) {
        println!("Git expose finder");
    }

    fn desc(&self) {
        println!(
            "Module to scan ports to ensure HTTP is running and to look for an exposed .git folder"
        );
    }
}
