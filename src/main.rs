use clap::Parser;

pub mod levelance;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input string to decode
    input: String,
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let chunks = levelance::decode(&args.input)?;
    println!(
        "{:?}",
        chunks
            .into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
    Ok(())
}
