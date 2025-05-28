use colored::*;
use reqwest::{blocking::Client, header::{HeaderMap, HeaderName, HeaderValue}};
use std::{fs, str::FromStr, sync::Arc};

// This module is only for subdomains bruteforcing.
use crate::{Context, Models::check_target, Scan};
use std::{io::{BufRead, BufReader}};

use super::ALLOWED_STATUS_CODES;


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

    pub fn add_header(headers: &mut HeaderMap, key: &str, value: &str) {
        if let (Ok(header_name), Ok(header_value)) = (
            HeaderName::from_str(key),
            HeaderValue::from_str(value),
        ) {
            headers.append(header_name, header_value);
        } else {
            eprintln!("Invalid header: key='{}', value='{}'", key, value);
        }
    }

    //TODO: Get the target and add the host as header and check
    fn is_exist(client: Arc<Client>, domain: &str, word: String, target: url::Url) {



        let subdomain = format!("{}.{}", word, domain);
        let mut headers = HeaderMap::new();
        SubDomainActive::add_header(&mut headers, "Host", &subdomain);
        SubDomainActive::add_header(&mut headers, "User-Agent", "web_.01");

        if let Ok(response) = client.get(target).headers(headers).send() {
            if ALLOWED_STATUS_CODES.contains(&response.status()) {
                println!("{}", format!("Status: {}, {}", response.status(), subdomain).green().bold());
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


    fn enumerate(&self, ctx: &Context) {
        let reader = SubDomainActive::checK_file(&self.wordlist);
        let client = Arc::new(ctx.client.clone());
        let url = check_target(ctx);


        for line in reader.lines() {
            match line {
                Ok(word) => {
                    let domain_clone = self.domain.to_string();
                    let client_clone = Arc::clone(&client);
                    let target_clone = url.clone();

                    ctx.thread_pool.execute(move || {
                        SubDomainActive::is_exist(client_clone, &domain_clone, word, target_clone);
                    });
                }
                Err(e) => eprintln!("Error reading line from wordlist: {}", e),
            }
        }

        ctx.thread_pool.join(); 
    }

}
