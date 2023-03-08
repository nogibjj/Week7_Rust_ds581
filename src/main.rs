use Week7_Rust_ds581::get_temperature;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <zip-code>", args[0]);
        return;
    }

    let zip_code = &args[1];
    match get_temperature(zip_code).await {
        Ok(temperature) => println!("Current temperature for {} is {}.", zip_code, temperature),
        Err(e) => eprintln!("Error: {}", e),
    }
}

