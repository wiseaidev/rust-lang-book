use clap::Args;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    author = "Mahmoud Harmouch",
    version = "1.0",
    about = "A command-line text manipulation utility",
    name = "Text Manipulation Utility"
)]
pub struct Cli {
    /// Turn debugging information on.
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    /// Find and Replace commands.
    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Subcommand for handling find operations.
    Find(FindCommands),
    /// Subcommand for handling replace operations.
    Replace(ReplaceCommands),
}

/// Represents find-related commands.
#[derive(Debug, Args)]
pub struct FindCommands {
    /// Sets the input file to process
    #[clap(short = 'i', long = "input")]
    pub input: Option<String>,
    /// Sets the pattern to find.
    #[clap(short = 'p', long = "pattern")]
    pub pattern: Option<String>,
}

/// Represents repalce-related commands.
#[derive(Debug, Args)]
pub struct ReplaceCommands {
    /// Sets the input file to process
    #[clap(short = 'i', long = "input")]
    pub input: Option<String>,
    /// Sets the pattern to find.
    #[clap(short = 'p', long = "pattern")]
    pub pattern: Option<String>,
    /// Sets the replacement text.
    #[clap(short = 'r', long = "replace")]
    pub replace: Option<String>,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Find(command)) => {
            match command.input {
                Some(ref input) => {
                    // cargo run -- find --input file_name
                    println!("{:?}", input);
                }
                None => {
                    println!("Please provide a file name.");
                }
            };
            match command.pattern {
                Some(ref pattern) => {
                    // cargo run -- find --pattern custom_pattern
                    println!("{:?}", pattern);
                }
                None => {
                    println!("Please provide a pattern.");
                }
            };
        }
        Some(Commands::Replace(command)) => {
            match command.input {
                Some(ref input) => {
                    // cargo run -- replace --input file_name
                    println!("{:?}", input);
                }
                None => {
                    println!("Please provide a file name.");
                }
            };
            match command.pattern {
                Some(ref pattern) => {
                    // cargo run -- replace --pattern custom_pattern
                    println!("{:?}", pattern);
                }
                None => {
                    println!("Please provide a pattern.");
                }
            };
            match command.replace {
                Some(ref replace) => {
                    // cargo run -- replace --replace string
                    println!("{:?}", replace);
                }
                None => {
                    println!("Please provide a pattern.");
                }
            };
        }
        None => println!(
            "Unknown command. Use '--help' for usage instructions."
        )
    };
}