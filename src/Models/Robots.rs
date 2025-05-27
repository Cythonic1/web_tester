use reqwest::StatusCode;
use colored::*;
// Module to search for robots.txt file.
use crate::{Context, Scan};

use super::{check_target, format_domain};

pub struct Robots{}

impl Robots{
    fn check_listing(body: &str) -> bool{
        body.contains("User-agent: *")
    }

}
impl Scan for Robots{
    fn init(&self) {
        todo!()
    }
    fn enumerate(&self, ctx:&Context){
        let domain = check_target(ctx);

        let url = format_domain(&domain);

        let target = format!("{}/robots.txt", url);

        match ctx.client.get(target).send() {
            Ok(res) => match res.status() {
                StatusCode::OK => {
                    let content_len = res.content_length();
                    let res_url = res.url().to_string();
                    let body = res.text().expect("can not convert non-utf-8 char");
                    if !res_url.ends_with("robots.txt") || !res_url.contains("robots.txt"){
                        println!("{}", "The target is not vulnrable".red().bold());
                        return ;
                    }
                    println!("{}", "The traget is most likely vulnrable checking the directory listing".blue().bold());
                    // Check if the really needed
                    if !Robots::check_listing(&body){
                        println!("{}", "The target does not allow listing".red().bold());
                        return ;
                    }
                    println!("{}", "The traget is 70% vulnrable checking the content len".blue().bold());
                    if let Some(content_len)  = content_len{
                        if content_len < 10{
                            println!("{}", "The traget is 95% not vulnrable due to low content len".red())
                        }
                    }
                    println!("{}", "Target is vulnrable 99%".green().bold());
                },
                StatusCode::NOT_FOUND => {
                    println!("{}", "That target is not vulnrable".red().bold());
                },
                StatusCode::MOVED_PERMANENTLY =>{
                    println!("{}","Target maybe vulnrable".blue().bold());
                }
                _ =>{
                    println!("{}", "Unknown status code".red().bold());
                }

            },
            Err(e) => {
                println!("{}",format!("Error occur: {}", e).red().bold())
            },

        }
    }
}
