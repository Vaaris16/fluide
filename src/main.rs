use clap::{Parser, Subcommand};
mod packages;

#[derive(Parser)]
#[command(name = "fluide")]
#[command(about = "something")]
struct Cli {
    #[command(subcommand)]
    commands: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add { package_name: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.commands {
        Some(Commands::Add { package_name }) => {
            if package_name == "tailwindcss" {
                let _ = packages::tailwindcss_package::setup_tailwindcss();
            }
        }
        None => {
            print!("nothing")
        }
    }
}
