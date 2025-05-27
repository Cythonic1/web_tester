use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use crate::{Context, Models::Scan};
pub struct SubdminaPassive {
    domain: String
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct CrtInfo {
    common_name: String
}

#[allow(unused)]
impl SubdminaPassive {
    pub fn new(domain:String) -> Self {
        SubdminaPassive {
            domain
        }
    }

}

impl Scan for SubdminaPassive {
    fn init(&self) {
        todo!()
    }
    fn enumerate(&self, ctx:&Context) {
         
        let target = format!("https://crt.sh/?q={}&output=json", self.domain);
        match ctx.client
            .get(target.clone())
            .send()
        {
            Ok(res) => {
                match res.json::<Vec<CrtInfo>>() {
                    // Deserialize into Vec, not HashSet
                    Ok(records) => {
                        let subdomains: HashSet<String> = records
                            .into_iter()
                            .map(|entry| entry.common_name)
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
