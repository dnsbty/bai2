use bai2::Bai2File;
use clap::Parser;
use env_logger::Env;
use std::{fs, path::PathBuf};

/// Parse a BAI2 file into a rust object
#[derive(Debug, Parser)]
#[command(name = "bai2")]
#[command(about = "Parse a BAI2 file", long_about = None)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// path to your BAI2 file
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "warning")
        .write_style_or("LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let cli = Cli::parse();

    let content = fs::read_to_string(&cli.path)
        .map_err(|_| format!("could not read file `{}`", &cli.path.display()))?;

    match Bai2File::new(content) {
        Err(err) => println!("Failed to parse file: {}", err),
        Ok(file) => {
            println!("{}", serde_json::to_string_pretty(&file).unwrap());
        }
    };

    Ok(())
}
