use colored::*;
use reqwest::{blocking::Client, StatusCode};
use std::fs;

// This module is only for subdomains bruteforcing.
use crate::{Models::check_target, Context, Models::format_domain, Scan};
use std::{io::{BufRead, BufReader}, sync::Arc};


pub struct SubDomainActive{
    wordlist: String,
    domain: String
}

#[allow(unused)]
impl   SubDomainActive{
    pub fn new(path: String, domain: String) -> Self {
        SubDomainActive{
            wordlist : path,
            domain
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

        // TODO: FIX
        if let Ok(response) = client.get(&dest).send() {
            if response.status() == StatusCode::OK {
                println!("{}", format!("File returned 200: {}", dest).green().bold());
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


impl Scan for SubDomainActive {
    fn init(&self) {
        todo!();
    }
    fn enumerate(&self , ctx: &Context){
        println!("{}", self.wordlist);
        let reader = SubDomainActive::checK_file(&self.wordlist);

        let client = Arc::new(ctx.client.clone());

        for line in reader.lines() {
            let domain_clone = self.domain.to_string();
            let client_clone = Arc::clone(&client);

            ctx.thread_pool.execute(move || {
                SubDomainActive::is_exist(client_clone, &domain_clone, line);
            });
        }
    }

}
