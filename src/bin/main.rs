use anyhow::Result;
use artifact_query::ArtifactQuery;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to input file
    path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let query = ArtifactQuery::new()?;
    match query.from_path(&args.path)? {
        Some(v) => println!("This file is {v}."),
        None => println!("This file is not recognized."),
    }
    Ok(())
}
