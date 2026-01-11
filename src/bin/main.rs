use clap::Parser;
use anyhow::Result;
use artifact_query::ArtifactQuery;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to input file
    path: String,
}

#[tokio::main]
async fn main() -> Result<()>{
    let args = Args::parse();
    let parse = ArtifactQuery::from_path(&args.path).await?;
    match parse {
        Some(v) => println!("This file is {}.",v),
        None => println!("This file is not been recognize.")
    }
    Ok(())
}