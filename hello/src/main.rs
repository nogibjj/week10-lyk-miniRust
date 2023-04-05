use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yikai Liu",
    about = "Count distinct anagrams"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Yikai Liu")]
    Check {
        #[clap(short, long)]
        input: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Check { input }) => {
            let result = hello::roman_to_int(input);
            println!("{}", result);
        }
        None => println!("No subcommand was used"),
    }
}