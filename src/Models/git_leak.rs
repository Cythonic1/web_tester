use std::time::Duration;

// processing : blue
// bad luck red
// good luck green
use crate::{
    port_scanner::PortScanner,
    Models::{domain_format, Desc, Scan},
};
use colored::*;
use reqwest::StatusCode;
use tokio::time::sleep;

pub struct FindGitExpose;

impl FindGitExpose {
    // Function to check if HTTP port is open and return the open port, if any.
    fn get_open_http_port(domain: &str) -> Option<u16> {
        let port_scan = PortScanner::new(50);
        port_scan.scan_ports(domain); // Assuming scan_ports is a blocking function
        let open_ports = port_scan.open_ports.lock().expect("Failed to lock mutex");

        // Check for common HTTP ports (80, 443, 8080, etc.)
        for port in &[80, 443, 8080, 8000, 8443] {
            if open_ports.contains(port) {
                return Some(*port);
            }
        }
        None
    }

    fn is_git_dir_listing(body: &str) -> bool {
        return body.contains("HEAD")
            && body.contains("refs")
            && body.contains("config")
            && body.contains("index")
            && body.contains("objects");
    }
}

impl Scan for FindGitExpose {
    fn enumerate(client: reqwest::blocking::Client, domain: &str) {
        // Check if any HTTP port is open and get the port

        if let Some(open_port) = FindGitExpose::get_open_http_port(&domain) {
            // Adjust the protocol based on the port (443, 8443 -> HTTPS)
            let mut search_git_file = domain_format(domain, open_port);
            search_git_file = format!("{}/.git", search_git_file);
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
        } else {
            println!(
                "{}",
                "No HTTP ports are open. No further action will be taken."
                    .red()
                    .bold()
            );
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
