use crate::common_ports::MOST_COMMON_PORTS;
use colored::*;
use dns_lookup::lookup_host;
use reqwest::blocking::{Client, Response};
use std::{
    io::{self, Read},
    net::{IpAddr, SocketAddr, TcpStream},
    sync::{Arc, Mutex},
    time::Duration,
};
use url::Url; // Using `url` crate to parse the URL properly
use threadpool::ThreadPool;
#[derive(Debug)]
#[allow(unused)]
struct PageInfo(Response, String);

pub struct PortScanner {
    pub client: Client,
    pub open_ports: Arc<Mutex<Vec<u16>>>,
    pub pool: ThreadPool,
}

#[allow(unused)]
impl PortScanner {
    // Initialize a new PortScanner with a given number of threads
    pub fn new(num_threads: usize) -> Self {
        PortScanner {
            client: Client::new(),
            open_ports: Arc::new(Mutex::new(Vec::new())),
            pool: ThreadPool::new(num_threads),
        }
    }

    // Get web headers and body for a given URL
    fn get_web_headers(&self, url: &str) -> PageInfo {
        let mut res = self.client.get(url).send().expect("Failed to send request");
        println!("{res:#?}");
        println!("The website responded with status: {}", res.status());

        let mut body_byte = Vec::new();
        res.read_to_end(&mut body_byte)
            .expect("Failed to read body of response");

        let body = String::from_utf8_lossy(&body_byte).to_string();
        PageInfo(res, body)
    }

    // Perform DNS lookup for a given domain
    fn dns_look_up(&self, domain: &str) -> Result<Vec<IpAddr>, io::Error> {
        // Parse the URL and extract the host part
        let url = Url::parse(domain).expect("Failed to parse URL");

        let host = url.host_str().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No host found in URL"))?;

        // Check if the host is an IP address
        if let Ok(ip) = host.parse::<IpAddr>() {
            // If it's an IP address, return it without DNS lookup
            Ok(vec![ip])
        } else {
            // If it's a domain name, perform the DNS lookup
            let ips = lookup_host(host)?;
            Ok(ips.into_iter().filter(|ip| ip.is_ipv4()).collect())
        }
    }

    // Scan a port on a list of IPs and return the open port, if any
    fn scan_port(&self, ips: Vec<IpAddr>, port: u16) -> u16 {
        let sock_addr = SocketAddr::new(ips[0], port);
        let timeout = Duration::new(3, 0);

        match TcpStream::connect_timeout(&sock_addr, timeout) {
            Ok(_) => {
                println!(
                    "{}",
                    format!("Port: {} is open on IP {}", port, ips[0])
                        .green()
                        .bold()
                );
                port
            }
            Err(_) => 0,
        }
    }

    // Perform port scanning
    pub fn scan_ports(&self, domain: &str) {
        let record = self.dns_look_up(domain);
        println!("{:?}", record);

        match record {
            Ok(ips) => {
                for port in MOST_COMMON_PORTS {
                    let ips_clone = ips.clone(); // Clone the IPs for thread safety
                    let open_ports_clone = Arc::clone(&self.open_ports);
                    let port_clone = *port; // Clone the port to move into the closure

                    self.pool.execute(move || {
                        let scanner = PortScanner::new(1); // Create a new PortScanner for this thread
                        let res = scanner.scan_port(ips_clone, port_clone); // Each thread scans one port
                        if res != 0 {
                            let mut open_ports_guard =
                                open_ports_clone.lock().expect("Failed to lock mutex");
                            open_ports_guard.push(res);
                        }
                    });
                }
                self.pool.join(); // Wait for all threads to finish
            }
            Err(_) => println!("Something went wrong with the DNS lookup"),
        }

        let open_ports_guard = self.open_ports.lock().expect("Failed to lock mutex");
        println!("Open ports: {:#?}", *open_ports_guard);
    }
}
