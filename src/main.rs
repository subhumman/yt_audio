#![allow(non_snake_case)]

use anyhow::Result;
use dowAu::{darkprince_lover, get_current_dir};

#[derive(Debug)]
struct Args {
    url: String,
    output: String,
    format: String,
}

impl Args{
    fn parse() -> Result<Self>{
        let args = std::env::args().collect::<Vec<String>>();
        if args.len() < 2{
            anyhow::bail!("Usage: {} <url> [output] [format]", args[0]);
        }

        let url = args[1].clone();
        let output = args.get(2).cloned().unwrap_or_else(|| ".".to_string());
        let format = args.get(3).cloned().unwrap_or_else(|| "mp3".to_string());
        Ok(Self{url, output, format})
    }
}

#[tokio::main]
async fn main() -> Result<()>{
    let args = Args::parse()?;
    get_current_dir()?;
    darkprince_lover(&args.url, &args.format, &args.output).await
}
