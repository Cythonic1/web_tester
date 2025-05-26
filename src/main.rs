#[allow(non_snake_case)]
mod Models;
use std::time::Instant;
use crate::Models::port_scanner;
use crate::Models::cli::Cli;
use std::env;
use clap::Parser;
use Models::Scan;
use colored::*;
fn help(){
    println!("{}", "Please Enter IP address to scan and [-m] to scan with modules".red().bold());
    println!("{}", "usage: cargo run -- <IP> [-m]".to_string().red().bold());
}

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
    let args : Vec<String> = env::args().collect();
    if args.len() < 2{
        help();
        std::process::exit(1);
    }
    help();
    
    let target = &args[1];
    let scanner = port_scanner::PortScanner::new(50); // Create a scanner with 50 threads
    
    let start = Instant::now();
    scanner.scan_ports(target);
    if args.len() > 2{
        println!("{}", "-------------------------------------------------------------------------------------".to_string().bold());
        // Enumerate Robots File.
        println!("{}", "Start enumerate Robots file.".to_string().blue().bold());
        Models::robots::Robots::run(scanner.client.clone(), target);

        println!("{}", "-------------------------------------------------------------------------------------".to_string().bold());
        println!("{}", "Start enumerate git exposed directory.".to_string().blue().bold());
        Models::git_leak::FindGitExpose::run(scanner.client.clone(), target);

        println!("{}", "-------------------------------------------------------------------------------------".to_string().bold());
        println!("{}", "Start enumerate Passive Sub domains".to_string().blue().bold());
        Models::subdomains::passive::SubdminaPassive::run(scanner.client.clone(), target);

        println!("{}", "-------------------------------------------------------------------------------------".to_string().bold());
        println!("{}", "Start enumerate Active Sub domains".to_string().blue().bold());
        Models::subdomains::active::SubDomainActive::run(scanner.client.clone(), target);

        println!("{}", "-------------------------------------------------------------------------------------".to_string().bold());
        println!("{}", "Start enumerate hidden directories".to_string().blue().bold());
        Models::DirectoryBruteForceing::BruteForce::run(scanner.client.clone(), target);
    }
    let duration = start.elapsed();
    println!("Time taken to complete the scan: {:?}", duration);
}
