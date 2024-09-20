mod common_ports;
mod port_scanner;
use std::time::Instant;
fn main() {
    let scanner = port_scanner::PortScanner::new(50); // Create a scanner with 50 threads
    let start = Instant::now();

    // Scan ports
    scanner.scan_ports("mohe-dev.pro");

    // Measure duration
    let duration = start.elapsed();
    println!("Time taken to complete the scan: {:?}", duration);
}
