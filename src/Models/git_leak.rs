
use std::sync::{Arc, Mutex};
use reqwest::{StatusCode, Url};
use crate::{
    port_scanner::{self, PortScanner}, Models::Scan
};

pub struct FindGitExpose;

impl FindGitExpose {
    // Function to check if HTTP (port 80) is open
    fn is_http_open(domain: &str) -> bool {
        let port_scan = PortScanner::new(50);
        port_scan.scan_ports(domain);  // Assuming scan_ports is a blocking function
        let open_ports = port_scan.open_ports.lock().expect("Failed to lock mutex");

        let is_http_open = open_ports.contains(&80);
        println!("Scanning ports ended. HTTP is open: {is_http_open}");

        is_http_open
    }
}

impl Scan for FindGitExpose {
    fn enumerate(client: reqwest::blocking::Client, url: &str) {
        // Check if HTTP port is open
        if FindGitExpose::is_http_open(&url) {
            let search_git_file = format!("{}/.git", url);
            println!("{}", search_git_file);
            match client.get(&search_git_file).send() {
                Ok(response) => {
                    match response.status() {
                        StatusCode::OK => {
                            let url_res = response.url().to_string();
                            if url_res.ends_with("/.git/") || url_res.contains("/.git/") {
                                            println!("{:#?}", response);
                                            println!("Target is vulnerable; it has .git exposed.");
                                        } else {
                                            println!("The website return 200 but .git not in the url maybe redirection");
                                        }
                        },
                        StatusCode::MOVED_PERMANENTLY => {
                            println!("The target might be vulnerable (301 Redirect).");
                        },
                        _ => {
                            println!("Target is not vulnerable.");
                        }
                    }
                },
                Err(e) => {
                    println!("Failed to send the request: {}", e);
                }
            }
        } else {
            println!("HTTP is closed. No further action will be taken.");
        }
    }
}
