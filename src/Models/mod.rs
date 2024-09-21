use reqwest::blocking::Client;

pub trait Scan {
    fn enumerate(client: Client, url : &str);
}
pub mod git_leak;
