use std::env;
use std::fmt::{Debug};

#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}
impl Sizes {
    fn new(size: u64) -> Self {
        let bytes = format!("{} bytes", size);
        let kilobytes = format!("{:.2} kilobytes", size / 1000);
        let megabytes = format!("{:.2} megabytes", size / 1_000_000);
        let gigabytes = format!("{:.2} gigabytes", size / 1_000_000_000);

        Sizes {
            bytes,
            kilobytes,
            megabytes,
            gigabytes,
        } 
    }
}

fn parse_input(input: &str) -> Result<u64, &'static str> {
    let args: Vec<String> = env::args().collect();
    let parts: Vec<&str> = args[1].split_whitespace().collect();
    if parts.len() != 2 {
        return Err("Invalid input format. Expected format: <size> <unit>")
    }

    let size: u64 = parts[0].parse().map_err(|_| "Invalid size value")?;
    let unit = parts[1].to_lowercase();

    match unit.as_str() {
        "b" | "bytes" => Ok(size),
        "kb" | "kilobytes" => Ok(size * 1_000),
        "mb" | "megabytes" => Ok(size * 1_000_000),
        "gb" | "gigabytes" => Ok(size * 1_000_000_000),
        _ => Err("Invalid unit. Expected one of: b, kb, mb, gb"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <size> <unit>", args[0]);
        return;
    }

    let input = &args[1];
    match parse_input(input) {
        Ok(size) => {
            let sizes = Sizes::new(size);
            println!("{:?}", sizes);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}