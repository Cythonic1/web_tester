use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "scanner", version = "1.0", author = "Pythonic01")]
pub struct Cli {
    /// Global target for the scan
    #[arg(short, long, global = true, help="(e.g) https://pythonic01.com")]
    pub target: Option<String>,

    /// Global timeout for requests
    #[arg(long, default_value = "10")]
    pub timeout: u64,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}


#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run all scans
    All,

    /// Run only the Git leak module
    GitLeak,

    /// Run the robots.txt scanner
    Robots,


    SubDomainPassive{
        #[arg(short, long, help = "Custom wordlist to use")]
        domain: String,

    },

    DirEnum{
        #[arg(short, long, help = "Custom wordlist to use")]
        wordlist: String,

    },


    SubDomainActive{
        #[arg(short, long, help = "Custom wordlist to use")]
        domain: String,

        #[arg(short, long, help = "Custom wordlist to use")]
        wordlist: String,
    },
}
