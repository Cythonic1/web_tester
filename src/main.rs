#[allow(non_snake_case)]
mod Models;
use std::time::Duration;
use crate::Models::cli::Commands;
use crate::Models::cli::Cli;
use clap::Parser;
use threadpool::ThreadPool;
use Models::Scan;


pub struct Context {
    cli: Cli,
    client: reqwest::blocking::Client,
    thread_pool: ThreadPool
}
// Because of setting the target to be option due to the 
// gloable flag. Then we use this function check if the target is added or not
// Target is a must if it does not exist then exit failure
fn check_target(args: &Cli) -> bool {
    if args.target.is_some() {
        return true ;
    }
    eprintln!("Error: missing target. Please specify it using the --target option.");
    std::process::exit(1);
}

fn execute_operation(op: &impl Scan, ctx: &Context) {
    op.enumerate(ctx);
}


fn main() {
    let cli = Cli::parse();

    // Make it check if the client provide timeout or not
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10)) // set timeout to 10 seconds
        .build()
        .expect("Failed to build HTTP client");
    // 20 for now later will be user input
    let thread_pool = ThreadPool::new(20);

    let ctx = Context{client, cli, thread_pool};

    match &ctx.cli.command{
        Commands::GitLeak => {
            let op = Models::GitLeak::GitLeak;
            check_target(&ctx.cli);
            execute_operation(&op, &ctx);
            println!("{:?}", ctx.cli);
        },
        Commands::Robots => {
            let op = Models::Robots::Robots{};
            check_target(&ctx.cli);
            execute_operation(&op, &ctx);
            println!("{:?}", ctx.cli);
        },
        Commands::All => {
            check_target(&ctx.cli);
            println!("{:?}", ctx.cli);
            todo!()
        },
        Commands::PortScan => {
            check_target(&ctx.cli);
            println!("{:?}", ctx.cli);
            todo!()

        },
        Commands::Search { wordlist, threads } => {
            check_target(&ctx.cli);
            println!("{:?}", ctx.cli);
            todo!()
        },
        Commands::SubDomainPassive { domain } => {
            let op = Models::subdomains::passive::SubdminaPassive::new(domain.to_string());
            check_target(&ctx.cli);
            execute_operation(&op, &ctx);
            println!("{:?}", ctx.cli);
        }
        Commands::SubDomainActive { domain, wordlist } => {
            let op = Models::subdomains::active::SubDomainActive::new (wordlist.to_string(), domain.to_string());
            check_target(&ctx.cli);
            execute_operation(&op, &ctx);
            println!("{:?}", ctx.cli);
        }

    }
}

