// Brute forcing Module to find hidden directories.
use colored::*;
use reqwest::{blocking::Client};
use crate::{Context, Scan};
use std::fs::{self};
use std::io::{BufReader, BufRead}; // Added BufRead for reading lines
use std::sync::Arc;
use super::{check_target};
use crate::Models::subdomains::ALLOWED_STATUS_CODES;


pub struct BruteForce {
    wordlist: String,
}

impl BruteForce {
    pub fn new(wordlist:String) -> Self {
        BruteForce { wordlist }
    }

    fn is_exist(url: url::Url, file: String, client: Arc<Client>) {
        let dest = format!("{}{}", url, file);
        if let Ok(res) = client.get(&dest).send() {
            if ALLOWED_STATUS_CODES.contains(&res.status()) {
                println!("{}", format!("Status: {}, {}", res.status(), dest).green().bold());
            }
        }
    }


    fn checK_file(wordlist:&str) -> BufReader<fs::File> {
        match fs::File::open(wordlist){
            Ok(file) => {
                BufReader::new(file)
            },
            Err(err) => {
                eprintln!("{err}");
                std::process::exit(1);
            }
        }
    }


}

impl Scan for BruteForce {
    fn init(&self) {
        todo!()
    }
    fn enumerate(&self,ctx:&Context) {
        let url = check_target(ctx);

        let client = Arc::new(ctx.client.clone());

        let reader = BruteForce::checK_file(&self.wordlist);

        for line in reader.lines() {
            match line {
                Ok(line_content) => {
                    // Use line_content as needed
                    let domain_clone = url.clone();
                    let client_clone = Arc::clone(&client); // Clone the Arc to share ownership
                    ctx.thread_pool.execute(move || {
                        BruteForce::is_exist(domain_clone, line_content.clone(),  client_clone);
                    })
                }
                Err(e) => {
                    eprintln!("Error reading line: {}", e);
                }
            }
        }
        ctx.thread_pool.join();
    }
}
