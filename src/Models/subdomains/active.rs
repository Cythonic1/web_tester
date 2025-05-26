use colored::*;
use reqwest::{blocking::Client, StatusCode};
use threadpool::ThreadPool;

// This module is only for subdomains bruteforcing.
use crate::Scan;
use std::{fs::File, io::{BufRead, BufReader}, path::PathBuf, sync::Arc};


pub struct SubDomainActive{
    wordlist: PathBuf
}

#[allow(unused)]
impl   SubDomainActive{
    pub fn new(path: PathBuf) -> Self{
         SubDomainActive{
            wordlist:path
        }
    }

    fn is_exist(client: Arc<Client>, domain: &str, word: Result<String, std::io::Error>) {
        let word = match word {
            Ok(val) => val,
            Err(err) => {
                eprintln!("{}", format!("Failed to read word: {}", err).red().bold());
                return;
            }
        };

        let dest = format!("{}.{}", word, domain);

        if let Ok(response) = client.get(&dest).send() {
            if response.status() == StatusCode::OK {
                println!("{}", format!("File returned 200: {}", dest).green().bold());
            }
        }
    }

    pub fn run(client: reqwest::blocking::Client, target: &str){
        let path = PathBuf::from("/home/pythonic/Downloads/WordLists/Directories_small.txt");
        let run_git = SubDomainActive::new(path);
        run_git.enumerate(client, target);
    }

}


impl Scan for SubDomainActive {
    fn enumerate(&self,client: reqwest::blocking::Client, url: &str) {

        let pool = ThreadPool::new(20);
        let client = Arc::new(client);
        if let Ok(file) = File::open(&self.wordlist) {
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let domain_clone = url.to_string();
                let client_clone = Arc::clone(&client);

                pool.execute(move || {
                    SubDomainActive::is_exist(client_clone, &domain_clone, line);
                });
            }
        } else {
           eprintln!("{}",format!("Cannot read the wordlist file: {:#?}", self.wordlist).red().bold());
        }
    }

}
