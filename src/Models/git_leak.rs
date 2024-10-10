// processing : blue
// bad luck red
// good luck green
use crate::Models::{Desc, Scan};

use colored::*;
use reqwest::StatusCode;

pub struct FindGitExpose;

#[allow(unused)]
impl FindGitExpose {
    fn is_git_dir_listing(body: &str) -> bool {
        return body.contains("HEAD")
            && body.contains("refs")
            && body.contains("config")
            && body.contains("index")
            && body.contains("objects");
    }

    fn new() -> Self{
        FindGitExpose{}
    }
    pub fn run(client: reqwest::blocking::Client, target: &str){
        let run_git = FindGitExpose::new();
        run_git.enumerate(client, target);


    }
}

impl Scan for FindGitExpose {
    fn enumerate(&self,client: reqwest::blocking::Client, domain: &str) {
        // Check if any HTTP port is open and get the port

            // Adjust the protocol based on the port (443, 8443 -> HTTPS)
            // let mut search_git_file = domain_format(domain, open_port);
            let search_git_file = format!("{}/.git", domain);
            // Form the URL based on the open port

            match client.get(&search_git_file).send() {
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
                            if FindGitExpose::is_git_dir_listing(&body) {
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


impl Desc for FindGitExpose {
    fn name(&self) {
        println!("Git expose finder");
    }

    fn desc(&self) {
        println!(
            "Module to scan ports to ensure HTTP is running and to look for an exposed .git folder"
        );
    }
}
