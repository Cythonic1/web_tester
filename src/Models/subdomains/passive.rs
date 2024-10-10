use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use crate::Models::Scan;
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct SubdminaPassive {
    name_value: String,
}

#[allow(unused)]
impl SubdminaPassive {
    pub fn new() -> Self {
        SubdminaPassive {
            name_value: String::new(),
        }
    }
    pub fn run(client: reqwest::blocking::Client, target: &str){
        let run_passive_domains = SubdminaPassive::new();
        run_passive_domains.enumerate(client, target);


    }

}

impl Scan for SubdminaPassive {
    fn enumerate(&self, client: reqwest::blocking::Client, url: &str) {
        let target = format!("https://crt.sh/?q={}&output=json", url);
        match client
            .get(target.clone())
            .send()
        {
            Ok(res) => {
                match res.json::<Vec<SubdminaPassive>>() {
                    // Deserialize into Vec, not HashSet
                    Ok(json_records) => {
                        let subdomains: HashSet<String> = json_records
                            .into_iter()
                            .map(|entry| {
                                entry
                                    .name_value
                                    .split('\n')
                                    .map(|subdomain| subdomain.trim().to_string())
                                    .collect::<Vec<String>>()
                            })
                            .flatten()
                            .filter(|subdomain: &String| {
                                subdomain != &target && !subdomain.contains('*')
                            })
                            .collect();
                        println!("{:#?}", subdomains);
                    }
                    Err(err) => println!("Failed to parse JSON: {:?}", err),
                }
            }
            Err(err) => println!("Something went wrong with the request: {:?}", err),
        };
    }
}
