use crate::Context;
// Add the inisializtion function and inplemented for all of them
#[allow(dead_code)]
pub trait Scan {
    fn enumerate(&self, ctx:&Context);
    fn init(&self);
}

#[allow(unused)]
pub trait Desc {
    fn name(&self);
    fn desc(&self);
}

// This fucntion now validate that the given is url and return it 
// unwrapped
pub fn check_target(ctx:&Context) -> url::Url{
    if let Some(domain) = &ctx.cli.target {
        match url::Url::parse(domain) {
            Ok(url) => url,
            Err(err) => {
                eprintln!("{err}");
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("No target found Consider Adding --target <URL>");
        std::process::exit(1);
    }
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
pub mod GitLeak;
pub mod Robots;
pub mod subdomains;
pub mod port_scanner;
pub mod common_ports;
pub mod cli;
