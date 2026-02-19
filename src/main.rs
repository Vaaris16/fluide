use clap::{Parser, Subcommand, ValueEnum};
mod packages;
use std::env;

use crate::packages::file_errors;

#[derive(Parser)]
#[command(name = "fluide")]
#[command(about = "something")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        package: String,

        #[arg(short, long, value_enum)]
        framework: Option<Framework>,
    },
}

#[derive(ValueEnum, Clone, Debug)]
enum Framework {
    Vanilla,
    React,
    Preact,
    Vue,
    Svelte,
    Solid,
    Lit,
    Qwik,
    Angular,
    Ember,
    Marko,
}

fn main() {
    let cli = Cli::parse();

    let cd = env::current_dir()
        .map_err(|_| file_errors::FileErrors::FailedToGetCurrentDir)
        .unwrap();

    match cli.command {
        Commands::Add { package, framework } => match package.as_str() {
            "tailwindcss" => {
                if framework.is_some() {
                    eprintln!("the framework flag is not required by tailwindcss");
                    std::process::exit(1);
                }

                let _ = packages::tailwindcss_package::setup_tailwindcss();
            }

            "sass" => {
                if !framework.is_some() {
                    eprintln!("sass requires a framework flag");
                    std::process::exit(1)
                }

                let _ = packages::sass_package::setup(framework, &cd);
            }
            _ => {
                print!("not programmed yet")
            }
        },
    }
}
