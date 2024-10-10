
// Brute forcing Module to find hidden directories.

use std::path::PathBuf;
use reqwest::{blocking::Client, StatusCode};
use threadpool::ThreadPool;
use crate::Scan;
use std::fs::File;
use std::io::{BufReader, BufRead}; // Added BufRead for reading lines
use std::sync::Arc;
#[allow(unused)]
pub struct BruteForce {
    path_file: PathBuf,
}

#[allow(unused)]
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

    pub fn run(client: reqwest::blocking::Client, target: &str){
        let path = PathBuf::from("/home/pythonic/Downloads/WordLists/Directories_small.txt");
        let run_git = BruteForce::new(path);
        run_git.enumerate(client, target);


    }

}

impl Scan for BruteForce {
    fn enumerate(&self,client: Client, url: &str) {
        let pool = ThreadPool::new(20);

        let client = Arc::new(client);
        match File::open(self.path_file.clone()) {
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
