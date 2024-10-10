


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

    fn is_exist(client: Arc<Client>, domain: &str, word:&str){

        let dest  = format!("{word}.{domain}");
        match client.get(&dest).send() {
            Ok(res) => match res.status(){
                StatusCode::OK => {
                    println!("{}", format!("File return 200: {}", {dest}).green().bold());
                }
                _ => {}

            },
            Err(_) => ()
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
        match File::open(&self.wordlist){
            Ok(file) => {
                let reader = BufReader::new(file);
                for line in reader.lines(){
                    match line{
                        Ok(line_content) => {
                            let domain_clone = url.to_string();
                            let client_clone = Arc::clone(&client);
                            pool.execute(move || {
                                SubDomainActive::is_exist(client_clone,&domain_clone, &line_content );
                            })
                        },
                        Err(_) => (),

                    }
                }

            },
            Err(err) => println!("{}",format!("can not read this file error occur :{err}").red().bold()),
        }
        
    }
    
}
