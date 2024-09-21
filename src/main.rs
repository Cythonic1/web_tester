mod Models;
mod common_ports;
mod port_scanner;
use std::time::Instant;

use Models::Scan;
fn main() {
    let scanner = port_scanner::PortScanner::new(50); // Create a scanner with 50 threads
    let start = Instant::now();

    Models::git_leak::FindGitExpose::enumerate(scanner.client, "http://127.0.0.1");
    // Scan ports

    // Measure duration
    let duration = start.elapsed();
    println!("Time taken to complete the scan: {:?}", duration);
}
