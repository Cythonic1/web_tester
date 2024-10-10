#[allow(non_snake_case)]
mod Models;
mod common_ports;
mod port_scanner;
use std::time::Instant;
use std::env;
use Models::Scan;
use colored::*;
fn help(){

    println!("{}", format!("Please Enter IP address to scan and [-m] to scan with modules").red().bold());
    println!("{}", format!("usage: cargo run -- <IP> [-m]").red().bold());
}
fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 2{
        help();
        std::process::exit(1);
    }
    help();
    
    let target = &args[1];
    let scanner = port_scanner::PortScanner::new(50); // Create a scanner with 50 threads
    
    let start = Instant::now();
    scanner.scan_ports(&target);
    if args.len() > 2{
        println!("{}", format!("-------------------------------------------------------------------------------------").bold());
        // Enumerate Robots File.
        println!("{}", format!("Start enumerate Robots file.").blue().bold());
        Models::robots::Robots::run(scanner.client.clone(), &target);

        println!("{}", format!("-------------------------------------------------------------------------------------").bold());
        println!("{}", format!("Start enumerate git exposed directory.").blue().bold());
        Models::git_leak::FindGitExpose::run(scanner.client.clone(), &target);

        println!("{}", format!("-------------------------------------------------------------------------------------").bold());
        println!("{}", format!("Start enumerate Passive Sub domains").blue().bold());
        Models::subdomains::passive::SubdminaPassive::run(scanner.client.clone(), &target);

        println!("{}", format!("-------------------------------------------------------------------------------------").bold());
        println!("{}", format!("Start enumerate Active Sub domains").blue().bold());
        Models::subdomains::active::SubDomainActive::run(scanner.client.clone(), &target);

        println!("{}", format!("-------------------------------------------------------------------------------------").bold());
        println!("{}", format!("Start enumerate hidden directories").blue().bold());
        Models::DirectoryBruteForceing::BruteForce::run(scanner.client.clone(), &target);
    }
    let duration = start.elapsed();
    println!("Time taken to complete the scan: {:?}", duration);
}
