
// Brute forcing Module to find hidden directories.

use std::path::PathBuf;
use reqwest::{blocking::Client, StatusCode};
use threadpool::ThreadPool;
use crate::Scan;
use std::fs::File;
use std::io::{BufReader, BufRead}; // Added BufRead for reading lines
use std::sync::Arc;

pub struct BruteForce {
    path_file: PathBuf,
}

impl BruteForce {
    pub fn new(path: PathBuf) -> Self {
        BruteForce { path_file: path }
    }

    fn is_exist(domain: &str, file: String, client: Arc<Client>) {
        let dest = format!("{}/{}", domain, file);
        match client.get(&dest).send() {
            Ok(res) => match res.status() {
                StatusCode::OK => {
                    println!("File returns 200: {}", dest);
                }
                _ => {}
            },
            Err(_) => {}
        }
    }
}

impl Scan for BruteForce {
    fn enumerate(client: Client, url: &str) {
        let bruteforce = Self::new(PathBuf::from("/home/pythonic01/make.txt"));
        let pool = ThreadPool::new(20);

        let client = Arc::new(client);
        match File::open(&bruteforce.path_file) {
            Ok(file) => {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    match line {
                        Ok(line_content) => {
                            // Use line_content as needed
                            let domain_clone = url.to_string();
                            let client_clone = Arc::clone(&client); // Clone the Arc to share ownership
                            pool.execute(move || {
                                BruteForce::is_exist(&domain_clone, line_content.clone(),  client_clone);
                            })
                        }
                        Err(e) => {
                            eprintln!("Error reading line: {}", e);
                        }
                    }
                }
                pool.join();
            }
            Err(e) => {
                eprintln!("Failed to open the file: {}", e);
            }
        }
    }
}
