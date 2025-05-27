// Brute forcing Module to find hidden directories.

use reqwest::{blocking::Client, StatusCode};
use crate::{Context, Scan};
use std::fs::{self};
use std::io::{BufReader, BufRead}; // Added BufRead for reading lines
use std::sync::Arc;
use super::{check_target, format_domain};

pub struct BruteForce {
    path_file: String,
}

impl BruteForce {

    fn is_exist(domain: &str, file: String, client: Arc<Client>) {
        let dest = format!("{}/{}", domain, file);
        if let Ok(res) = client.get(&dest).send() {
            if res.status() == StatusCode::OK {
                println!("File returns 200: {}", dest);
            }
        }
    }


    fn checK_file(wordlist:&str) -> BufReader<fs::File> {
        match fs::File::open(wordlist){
            Ok(file) => {
                let reader = BufReader::new(file);
                return reader;
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
        let domain = check_target(&ctx);
        let url = format_domain(&domain);

        let client = Arc::new(ctx.client.clone());

        let reader = BruteForce::checK_file(&self.path_file);

        for line in reader.lines() {
            match line {
                Ok(line_content) => {
                    // Use line_content as needed
                    let domain_clone = url.to_string();
                    let client_clone = Arc::clone(&client); // Clone the Arc to share ownership
                    ctx.thread_pool.execute(move || {
                        BruteForce::is_exist(&domain_clone, line_content.clone(),  client_clone);
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
