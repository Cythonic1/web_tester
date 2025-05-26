use reqwest::blocking::Client;

pub trait Scan {
    fn enumerate(&self, client: Client, url: &str);
}
#[allow(unused)]
pub trait Desc {
    fn name(&self);
    fn desc(&self);
}

#[allow(unused)]
pub fn domain_format(domain: &str, open_port: u16) -> String {
    // Adjust the protocol based on the port (443, 8443 -> HTTPS)
    let result: String;
    if !domain.starts_with("http") && !domain.starts_with("https") {
        let protocol = match open_port {
            443 | 8443 => "https",
            _ => "http",
        };
        result = format!("{}://{}:{}", protocol, domain, open_port);
        result
    } else {
        result = format!("{}:{}", domain, open_port);
        result
    }
}


pub mod DirectoryBruteForceing;
pub mod git_leak;
pub mod robots;
pub mod subdomains;
